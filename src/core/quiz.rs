use crate::core::data::{Question, QuestionVariant};
use rand::prelude::*;
use rand::seq::IteratorRandom;

pub trait Quiz {
    /// Sets the `Question` to be asked
    fn set_question(&mut self, q: &Question);
    /// Sets the context (a list of other Questions) that this Quiz belongs in
    fn set_context(&mut self, ctx: &[Question]);
    /// Ask the Question, returning a &str to be displayed
    fn ask(&self) -> &str;
    /// Returns a list of possible answers as &str's to be displayed
    fn get_choices(&self) -> Vec<&str>;
    /// Mutates the internal state so that get_choices() gives a hint
    fn get_hint(&mut self);
    /// Takes a user answer in the form of a &str, returning if it was correct
    /// and what the right answer was
    fn answer(&mut self, ans: &str) -> (bool, &str);
    /// Override the previous answer, marking it as correct
    fn i_was_right(&mut self);
    /// Checks which QuestionVariant is in Question, returning if this quiz is
    /// applicable to that variant
    fn is_applicable(&self, q: &Question) -> bool;
}

pub trait Game {
    // Add some ID so that questions can prefer a given game?
    fn progress(&self) -> (usize, i32, f64); // (# Remaining, # Seen, % Score)
    /// Moves to a new question and returns the question text
    fn next_question(&mut self) -> Option<&str>;
    fn get_choices(&self) -> Vec<&str>;
    /// Needs to remember how many hints are given (hence the mut)
    fn get_hint(&mut self) -> Vec<&str>; // Gives new choices
    fn answer(&mut self, ans: &str) -> (bool, String);
    fn i_was_right(&mut self);
}

// Use macros to implement the duplicate methods

// FIXME: I should be consistent here and cli.rs with Options vs Config (naming)

#[derive(Default)]
pub struct MCConfig {
    // Add number of choices & choice numbering
    pub flipped: bool,
}

pub struct FlashConfig;

pub struct MultipleChoice<'a> {
    config: MCConfig,
    rng: ThreadRng,
    // Maybe ditch these and use the seen and correct of the underlying questions
    // FIXME: Yeah, you should do that. Make it a method!
    correct: i32,
    seen: i32,
    idx: usize,
    questions: &'a mut [Question],
    choices: Vec<usize>,
}

pub struct Flashcards {
    _config: FlashConfig,
    rng: ThreadRng,
    // Maybe ditch these and use the seen and correct of the underlying questions
    correct: i32,
    seen: i32,
    idx: usize,
    questions: Vec<Question>,
}

impl<'a> MultipleChoice<'a> {
    pub fn new(config: MCConfig, questions: &'a mut [Question]) -> Self {
        //let mut questions = questions.to_vec();
        // FIXME: Flipping is disabled for the time being...
        // if config.flipped {
        //     for term in &mut questions {
        //         if let QuestionVariant::Term(card) = term.inner() {
        //             card.flip();
        //         }
        //     }
        // }
        Self {
            config,
            rng: thread_rng(),
            correct: 0,
            seen: 0,
            idx: 0,
            questions,
            choices: Vec::new(),
        }
    }

    fn current(&mut self) -> &mut Question {
        &mut self.questions[self.idx]
    }

    fn choices(&self) -> Vec<&Question> {
        self.choices
            .iter()
            .map(|&idx| &self.questions[idx])
            .collect()
    }
}

impl Flashcards {
    pub fn new(config: FlashConfig, questions: &[Question]) -> Self {
        let questions = questions.to_vec();
        Self {
            _config: config,
            rng: thread_rng(),
            correct: 0,
            seen: 0,
            idx: 0,
            questions,
        }
    }
    fn current(&mut self) -> &mut Question {
        &mut self.questions[self.idx]
    }
}

impl Game for MultipleChoice<'_> {
    fn progress(&self) -> (usize, i32, f64) {
        let score = f64::from(self.correct) / self.seen as f64 * 100.0;
        (self.questions.len(), self.seen, score)
    }

    fn next_question(&mut self) -> Option<&str> {
        let len = self.questions.len();
        if len == 0 {
            return None;
        }
        self.idx = self.rng.gen_range(0, len);
        self.seen += 1;
        let mut rng = self.rng;
        let mut choices: Vec<_> = (0..self.questions.len())
            .filter(|&c| c != self.idx)
            .choose_multiple(&mut rng, 3); // The config should determine the number of choices here
        choices.push(self.idx);
        choices.shuffle(&mut self.rng);
        self.choices = choices;
        Some(self.current().ask())
    }

    fn get_choices(&self) -> Vec<&str> {
        self.choices().iter().map(|q| q.peek()).collect()
    }

    fn get_hint(&mut self) -> Vec<&str> {
        let mut rng = self.rng;
        let n = self.choices.len();
        // Cheap clone... Find a better way...
        let mut choices = self
            .choices
            .clone()
            .into_iter()
            .filter(|&c| c != self.idx)
            .choose_multiple(&mut rng, n - 2);
        choices.push(self.idx);
        choices.shuffle(&mut rng);
        self.choices = choices;
        self.get_choices()
    }

    fn answer(&mut self, ans: &str) -> (bool, &str) {
        let idx: usize = ans.parse().unwrap();
        let ans_str = self.choices()[idx].peek().to_owned();
        let (correct, right_ans) = self.current().answer(&ans_str);
        if correct {
            self.correct += 1;
            // FIXME: What to do when questions are complete?
            //self.questions.remove(self.idx);
        }
        (correct, right_ans)
    }

    fn i_was_right(&mut self) {
        self.correct += 1;
        self.current().override_correct();
        // ^ This gets thrown away below...
        // FIXME: What to do when questions are complete?
        //self.questions.remove(self.idx);
    }
}

impl Game for Flashcards {
    fn progress(&self) -> (usize, i32, f64) {
        let score = f64::from(self.correct) / self.seen as f64 * 100.0;
        (self.questions.len(), self.seen, score)
    }

    fn next_question(&mut self) -> Option<&str> {
        let len = self.questions.len();
        if len == 0 {
            return None;
        }
        self.idx = self.rng.gen_range(0, len);
        self.seen += 1;
        Some(self.current().ask())
    }

    fn get_choices(&self) -> Vec<&str> {
        Vec::new()
    }

    fn get_hint(&mut self) -> Vec<&str> {
        todo!()
    }

    fn answer(&mut self, ans: &str) -> (bool, &str) {
        let (correct, right_ans) = self.current().answer(ans);
        if correct {
            self.correct += 1;
            self.questions.remove(self.idx);
        }
        (correct, right_ans)
    }

    fn i_was_right(&mut self) {
        self.correct += 1;
        self.current().override_correct();
        // ^ This gets thrown away below...
        self.questions.remove(self.idx);
    }
}

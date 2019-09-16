use crate::core::data::Question;
use rand::prelude::*;
use rand::seq::IteratorRandom;

pub trait Game {
    type Config;
    fn new(config: Self::Config, questions: &[Question]) -> Self;
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

pub struct MCConfig;
pub struct FlashConfig;

pub struct MultipleChoice {
    _config: MCConfig,
    rng: ThreadRng,
    // Maybe ditch these and use the seen and correct of the underlying questions
    correct: i32,
    seen: i32,
    idx: usize,
    questions: Vec<Question>,
    choices: Option<Vec<usize>>,
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

impl MultipleChoice {
    fn current(&mut self) -> &mut Question {
        &mut self.questions[self.idx]
    }

    fn choices(&self) -> Vec<&Question> {
        match &self.choices {
            Some(idxs) => idxs.iter().map(|&idx| &self.questions[idx]).collect(),
            None => Vec::new(),
        }
    }
}

impl Flashcards {
    fn current(&mut self) -> &mut Question {
        &mut self.questions[self.idx]
    }
}

impl Game for MultipleChoice {
    type Config = MCConfig;

    fn new(config: Self::Config, questions: &[Question]) -> MultipleChoice {
        let questions = questions.to_vec();
        MultipleChoice {
            _config: config,
            rng: thread_rng(),
            correct: 0,
            seen: 0,
            idx: 0,
            questions,
            choices: None,
        }
    }

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
        self.choices = Some(choices);
        Some(self.current().ask())
    }

    fn get_choices(&self) -> Vec<&str> {
        self.choices().iter().map(|q| q.peek()).collect()
    }

    fn get_hint(&mut self) -> Vec<&str> {
        unimplemented!();
    }

    fn answer(&mut self, ans: &str) -> (bool, String) {
        let idx: usize = ans.parse().unwrap();
        let ans_str = self.choices()[idx].peek().to_owned();
        let (correct, right_ans) = self.current().answer(&ans_str);
        let right_ans = right_ans.to_owned(); // This feels hacky
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

impl Game for Flashcards {
    type Config = FlashConfig;

    fn new(config: Self::Config, questions: &[Question]) -> Flashcards {
        let questions = questions.to_vec();
        Flashcards {
            _config: config,
            rng: thread_rng(),
            correct: 0,
            seen: 0,
            idx: 0,
            questions,
        }
    }

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
        unimplemented!()
    }

    fn answer(&mut self, ans: &str) -> (bool, String) {
        let (correct, right_ans) = self.current().answer(ans);
        let right_ans = right_ans.to_owned(); // This feels hacky
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

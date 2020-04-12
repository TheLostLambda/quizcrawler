use crate::core::data::{Question, QuestionVariant};
use rand::prelude::*;
use rand::seq::{IteratorRandom, SliceRandom};
use std::{
    borrow::Borrow,
    cell::{RefCell, RefMut},
    rc::Rc,
};

// FIXME: Add some explanations
type QuestionRef = Rc<RefCell<Question>>;
type QuizRef = Rc<RefCell<Box<dyn Quiz>>>;
type QuestionList = Rc<Vec<QuestionRef>>;
type QuizList = Rc<Vec<QuizRef>>;

pub struct QuizDispatcher {
    questions: QuestionList,
    quizzes: QuizList,
    reference: Vec<Question>,
    rng: ThreadRng,
}

impl QuizDispatcher {
    /// Set the list of `Question`'s to ask and `Quiz`'s to be dispatched
    pub fn new(questions: QuestionList, quizzes: QuizList) -> Self {
        // FIXME: Add some explanations
        let reference = questions
            .iter()
            .map(|rc| (**rc).clone().into_inner())
            .collect();
        let rng = thread_rng();
        Self {
            questions,
            quizzes,
            reference,
            rng,
        }
    }

    /// Sorts `Question`s by mastery, then dispatches a random `Quiz` if one
    /// is available
    pub fn next(&mut self) -> Option<QuizRef> {
        // FIXME: Implement priority sorting and end the quiz after mastery
        let question = self.questions.iter().cloned().choose(&mut self.rng)?;
        let quiz = self
            .quizzes
            .iter()
            .cloned()
            .filter(|qz| {
                let qz = (**qz).borrow();
                qz.is_applicable(&(*question).borrow())
            })
            .choose(&mut self.rng)?;
        {
            let mut quiz = quiz.borrow_mut();
            quiz.set_question(question);
            quiz.set_context(Rc::clone(&self.questions));
        }
        Some(quiz)
    }

    /// Returns the number of questions remaining and the current score as a
    /// percentage
    pub fn progress(&self) -> (usize, f64) {
        // FIXME: Put actual logic here
        (42, 33.3)
    }
}

pub trait Quiz {
    /// Sets the `Question` to be asked
    fn set_question(&mut self, q: QuestionRef);
    /// Sets the context (a list of `Questions`) that this Quiz belongs in
    fn set_context(&mut self, ctx: QuestionList);
    /// Ask the `Question`, returning a `&str` to be displayed
    fn ask(&self) -> &str;
    /// Returns a list of possible answers as `&str`'s to be displayed
    fn get_choices(&self) -> Vec<&str>;
    /// Mutates the internal state so that a hint is provided by other calls
    fn get_hint(&mut self);
    /// Takes a user answer in the form of a `&str`, returning if it was
    /// correct and what the right answer was
    fn answer(&mut self, ans: &str) -> (bool, &str);
    /// Override the previous answer, marking it as correct
    fn i_was_right(&mut self);
    /// Checks which `QuestionVariant` is in `Question`, returning if this quiz
    /// is applicable to that variant
    fn is_applicable(&self, q: &Question) -> bool;
}

// FIXME: I should be consistent here and cli.rs with Options vs Config (naming)

#[derive(Default)]
pub struct MCConfig {
    /// Sets whether `Term`'s should have their terms and definitions flipped
    pub flipped: bool,
    // Add number of choices & choice numbering
}

#[derive(Default)]
pub struct MultipleChoice {
    pub config: MCConfig,
    pub question: Option<QuestionRef>,
    pub context: QuestionList,
    choices: Vec<usize>,
}

impl MultipleChoice {
    pub fn new(config: MCConfig) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }
}

impl Quiz for MultipleChoice {
    fn set_question(&mut self, q: QuestionRef) {
        self.question = Some(q);
    }

    fn set_context(&mut self, ctx: QuestionList) {
        self.context = ctx;
    }

    fn ask(&self) -> &str {
        todo!()
    }

    fn get_choices(&self) -> Vec<&str> {
        todo!()
    }

    fn get_hint(&mut self) {
        todo!()
    }

    fn answer(&mut self, ans: &str) -> (bool, &str) {
        todo!()
    }

    fn i_was_right(&mut self) {
        todo!()
    }

    fn is_applicable(&self, q: &Question) -> bool {
        todo!()
    }
}

/* fn choices(&self) -> Vec<&Question> {
    self.choices
        .iter()
        .map(|&idx| &self.context[idx])
        .collect()
} */

/* impl Game for MultipleChoice<'_> {
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

    fn answer(&mut self, ans: &str) -> (bool, String) {
        let idx: usize = ans.parse().unwrap();
        let ans_str = self.choices()[idx].peek().to_owned();
        let (correct, right_ans) = self.current().answer(&ans_str);
        if correct {
            // self.correct += 1;
            // FIXME: What to do when questions are complete?
            //self.questions.remove(self.idx);
        }
        (correct, right_ans.to_owned())
    }

    fn i_was_right(&mut self) {
        self.correct += 1;
        self.current().override_correct();
        // ^ This gets thrown away below...
        // FIXME: What to do when questions are complete?
        //self.questions.remove(self.idx);
    }
}
 */

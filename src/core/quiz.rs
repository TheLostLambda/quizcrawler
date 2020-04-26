use super::data::{Question, QuestionRef, QuestionVariant};
use rand::{prelude::*, seq::IteratorRandom};
use std::{cell::RefCell, rc::Rc};

// FIXME: Add some explanations
pub type QuizRef = Rc<RefCell<Box<dyn Quiz>>>;
pub type Progress = (usize, f64); // FIXME: This should be a struct

// type QuestionCell = Rc<RefCell<Question>>
// ^ Make

pub struct QuizDispatcher {
    questions: Vec<QuestionRef>,
    quizzes: Vec<QuizRef>,
    reference: Vec<Question>,
    rng: ThreadRng,
}

// FIXME: Should this use the builder pattern?
impl QuizDispatcher {
    /// Set the list of `Question`'s to ask and `Quiz`'s to be dispatched
    pub fn new(questions: Vec<QuestionRef>) -> Self {
        // FIXME: Add some explanations
        let reference = questions
            .iter()
            .map(|rc| RefCell::clone(rc).into_inner())
            .collect();
        let rng = thread_rng();
        Self {
            questions,
            quizzes: Vec::new(), // Ew
            reference,
            rng,
        }
    }

    // FIXME: I'm not sure how I feel about this...
    pub fn register_quiz(&mut self, quiz: impl Quiz + 'static) {
        self.quizzes.push(Rc::new(RefCell::new(Box::new(quiz))));
    }

    /// Returns the number of questions remaining and the current score as a
    /// percentage
    pub fn progress(&self) -> Progress {
        // FIXME: Put actual logic here
        (self.questions.len(), 0.0)
    }
}

impl Iterator for QuizDispatcher {
    type Item = QuizRef;

    /// Sorts `Question`s by mastery, then dispatches a random `Quiz` if one
    /// is available
    fn next(&mut self) -> Option<QuizRef> {
        // FIXME: Implement priority sorting and end the quiz after mastery
        let question = self.questions.iter().cloned().choose(&mut self.rng)?;
        let quiz = self
            .quizzes
            .iter()
            .cloned()
            .filter(|qz| {
                let qz = qz.borrow();
                qz.is_applicable(&question.borrow())
            })
            .choose(&mut self.rng)?;
        {
            let mut quiz = quiz.borrow_mut();
            quiz.set_context(self.questions.to_vec());
            quiz.set_question(question);
        }
        Some(quiz)
    }
}

pub trait Quiz {
    /// Sets the `Question` to be asked
    fn set_question(&mut self, q: QuestionRef);
    /// Sets the context (a list of `Questions`) that this Quiz belongs in
    fn set_context(&mut self, ctx: Vec<QuestionRef>);
    /// Ask the `Question`, returning a `String` to be displayed. This returns
    /// a `String`, not a `&str`, so quizzes can do formatting on the question
    /// string before it's passed to the console
    fn ask(&self) -> String;
    /// Returns a list of possible answers as `String`'s to be displayed
    fn get_choices(&self) -> Vec<String>;
    /// Mutates the internal state so that a hint is provided by other calls
    fn get_hint(&mut self);
    /// Takes a user answer in the form of a `&str`, returning if it was
    /// correct and what the right answer was
    fn answer(&mut self, ans: &str) -> (bool, String);
    /// Override the previous answer, marking it as correct
    fn i_was_right(&mut self);
    /// Checks which `QuestionVariant` is in `Question`, returning if this quiz
    /// is applicable to that variant
    fn is_applicable(&self, q: &Question) -> bool;
}

// FIXME: I should be consistent here and cli.rs with Options vs Config (naming)

pub struct MCConfig {
    /// Whether `Term`'s should have their terms and definitions flipped
    pub flipped: bool,
    /// The number of answer choices for each question
    pub choices: usize,
    // Add choice numbering method? ABC vs 123, etc
}

impl Default for MCConfig {
    fn default() -> Self {
        Self {
            flipped: false,
            choices: 4,
        }
    }
}

#[derive(Default)]
pub struct MultipleChoice {
    pub config: MCConfig,
    pub question: Option<QuestionRef>,
    pub context: Vec<QuestionRef>,
    choices: Vec<QuestionRef>,
    rng: ThreadRng,
}

impl MultipleChoice {
    pub fn new(config: MCConfig) -> Self {
        Self {
            config,
            ..Self::default()
        }
    }
}

impl Quiz for MultipleChoice {
    fn set_question(&mut self, q: QuestionRef) {
        self.choices = self
            .context
            .clone()
            .into_iter()
            .filter(|cq| *cq.borrow() != *q.borrow())
            .choose_multiple(&mut self.rng, self.config.choices - 1);
        self.choices.push(q.clone());
        self.choices.shuffle(&mut self.rng);
        self.question = Some(q);
    }

    fn set_context(&mut self, ctx: Vec<QuestionRef>) {
        self.context = ctx;
    }

    // FIXME: Should this return a Cow<'static, str>?
    fn ask(&self) -> String {
        match self.question {
            Some(ref q) => q.borrow().ask().to_string(),
            None => String::new(),
        }
    }

    fn get_choices(&self) -> Vec<String> {
        self.choices
            .iter()
            .map(|q| q.borrow().peek().to_string())
            .collect()
    }

    fn get_hint(&mut self) {
        todo!()
    }

    fn answer(&mut self, ans: &str) -> (bool, String) {
        todo!()
    }

    fn i_was_right(&mut self) {
        todo!()
    }

    fn is_applicable(&self, q: &Question) -> bool {
        match q.data {
            QuestionVariant::Term(_) => true,
            _ => false,
        }
    }
}

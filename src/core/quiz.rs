use super::data::{Question, QuestionRef, QuestionVariant};
use rand::{prelude::*, seq::IteratorRandom};
use std::{cell::RefCell, cmp::Ordering, collections::HashSet, rc::Rc};

// FIXME: Add some explanations
pub type QuizRef = Rc<RefCell<Box<dyn Quiz>>>;

// FIXME: Where do I belong?
#[derive(Clone, Copy)]
pub struct Progress {
    pub questions: usize,
    pub learned: usize,
    pub score: f64,
}

// FIXME: Add QDOptions. Change MCConfig to MCOptions

pub struct QuizDispatcher {
    questions: Vec<QuestionRef>,
    quizzes: Vec<QuizRef>,
    reference: HashSet<Question>,
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
            quizzes: Vec::new(),
            reference,
            rng,
        }
    }

    // FIXME: I'm not sure how I feel about this...
    pub fn register_quiz(&mut self, quiz: impl Quiz + 'static) {
        self.quizzes.push(Rc::new(RefCell::new(Box::new(quiz))));
    }

    /// Returns the number of questions in the set, how many have been learned,
    /// and the current score as a percentage
    pub fn progress(&self) -> Progress {
        // FIXME: Put actual logic here
        Progress {
            questions: self.questions.len(),
            learned: 0,
            score: -1.0,
        }
    }

    fn remaining_questions(&self) -> Vec<QuestionRef> {
        todo!()
    }

    // FIXME: This needs to compare deltas from the reference
    fn delta_cmp(&self, a: QuestionRef, b: QuestionRef) -> Ordering {
        let (am, bm) = (a.borrow().mastery, b.borrow().mastery);
        // Lower mastery first
        am.cmp(&bm).reverse()//.then(...)
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
            .filter(|qz| qz.borrow().is_applicable(&question.borrow()))
            .choose(&mut self.rng)?;
        // Make sure that all contextual questions are supported as well
        let context: Vec<_> = self
            .questions
            .iter()
            .cloned()
            .filter(|q| quiz.borrow().is_applicable(&q.borrow()))
            .collect();
        {
            let mut quiz = quiz.borrow_mut();
            // FIXME: Should this just take a Vec<QuestionRef> directly?
            quiz.set_context(&context[..]);
            quiz.set_question(question);
        }
        Some(quiz)
    }
}

pub trait Quiz {
    /// Sets the `Question` to be asked
    fn set_question(&mut self, q: QuestionRef);
    /// Sets the context (a list of `Questions`) that this Quiz belongs in
    fn set_context(&mut self, ctx: &[QuestionRef]);
    /// Ask the `Question`, returning a `String` to be displayed. This returns
    /// a `String`, not a `&str`, so quizzes can do formatting on the question
    /// string before it's passed to the console
    fn ask(&self) -> String;
    /// Returns a list of possible answers as `String`'s to be displayed
    fn get_choices(&self) -> &[String];
    /// Mutates the internal state so that a hint is provided by other calls
    fn get_hint(&mut self);
    /// Takes a user answer in the form of a `&str` and if it's valid, returns
    /// if it was correct and what the right answer was
    fn answer(&mut self, ans: &str) -> Option<(bool, String)>;
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
    choices: Vec<String>,
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
        let answer = q.borrow().peek().to_string();
        let answer_bank: HashSet<_> = self
            .context
            .iter()
            .map(|q| q.borrow().peek().to_string())
            .collect();
        self.choices = answer_bank
            .into_iter()
            .filter(|s| s != &answer)
            .choose_multiple(&mut self.rng, self.config.choices - 1);
        self.choices.push(answer);
        self.choices.shuffle(&mut self.rng);
        self.question = Some(q);
    }

    fn set_context(&mut self, ctx: &[QuestionRef]) {
        self.context = ctx.to_vec();
    }

    // FIXME: Should this return a Cow<'static, str>?
    fn ask(&self) -> String {
        match self.question {
            Some(ref q) => q.borrow().ask().to_string(),
            None => String::new(),
        }
    }

    fn get_choices(&self) -> &[String] {
        &self.choices[..]
    }

    fn get_hint(&mut self) {
        todo!()
    }

    fn answer(&mut self, ans: &str) -> Option<(bool, String)> {
        let n: usize = ans.parse().ok()?;
        let choices = self.get_choices();
        match self.question {
            Some(ref q) if 0 < n && n <= choices.len() => {
                let mut q = q.borrow_mut();
                let (correct, answer) = q.answer(&choices[n - 1]);
                Some((correct, answer.to_string()))
            }
            _ => None,
        }
    }

    fn i_was_right(&mut self) {
        if let Some(ref q) = self.question {
            q.borrow_mut().override_correct()
        }
    }

    fn is_applicable(&self, q: &Question) -> bool {
        match q.data {
            QuestionVariant::Term(_) => true,
            _ => false,
        }
    }
}

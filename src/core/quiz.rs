use super::data::{Question, QuestionRef, QuestionVariant};
use rand::{prelude::*, seq::IteratorRandom};
use std::{cell::RefCell, collections::HashSet, rc::Rc};

// FIXME: Add some explanations
pub type QuizRef = Rc<RefCell<Box<dyn Quiz>>>;

// FIXME: Where do I belong?
#[derive(Clone, Copy)]
pub struct QuizProgress {
    pub questions: usize,
    pub learned: usize,
    pub score: f64,
}

// FIXME: Where do I belong?
#[derive(Clone, Copy)]
pub struct QuestionProgress {
    pub correct: usize,
    pub seen: usize,
}

// FIXME: Add QDOptions.

// FIXME: Maybe rename to just `Dispatcher`?
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
    pub fn progress(&self) -> QuizProgress {
        // FIXME: Put actual logic here
        QuizProgress {
            questions: self.questions.len(),
            learned: self.questions.len() - self.remaining_questions().len(),
            score: self.score(),
        }
    }

    // FIXME: Add a configurable mastery threshold for progression
    fn remaining_questions(&self) -> Vec<QuestionRef> {
        self.questions
            .iter()
            .cloned()
            .filter(|q| self.question_progress(q).correct < 1)
            .collect()
    }

    fn question_progress(&self, question: &QuestionRef) -> QuestionProgress {
        let question = question.borrow();
        // FIXME: Handle this unwrap a bit better
        let ref_question = self.reference.get(&question).unwrap();
        let correct = question.correct - ref_question.correct;
        let seen = question.seen - ref_question.seen;
        QuestionProgress { correct, seen }
    }

    fn score(&self) -> f64 {
        let (mut correct, mut seen) = (0, 0);
        for q in &self.questions {
            let progress = self.question_progress(q);
            correct += progress.correct;
            seen += progress.seen;
        }
        if seen > 0 {
            (correct as f64 / seen as f64) * 100.0
        } else {
            -1.0
        }
    }
}

impl Iterator for QuizDispatcher {
    type Item = QuizRef;

    /// Sorts `Question`s by mastery, then dispatches a random `Quiz` if one
    /// is available
    fn next(&mut self) -> Option<QuizRef> {
        let mut remaining = self.remaining_questions();
        remaining.shuffle(&mut self.rng);
        remaining.sort_unstable_by_key(|q| self.question_progress(q).seen);
        let question = Rc::clone(remaining.first()?); // This was nicer as .pop()
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

pub struct MCSettings {
    /// The number of answer choices for each question
    pub choices: usize,
    // Add choice numbering method? ABC vs 123, etc
}

impl Default for MCSettings {
    fn default() -> Self {
        Self { choices: 4 }
    }
}

#[derive(Default)]
pub struct MultipleChoice {
    pub settings: MCSettings,
    pub question: Option<QuestionRef>,
    pub context: Vec<QuestionRef>,
    choices: Vec<String>,
    rng: ThreadRng,
}

impl MultipleChoice {
    pub fn _new(settings: MCSettings) -> Self {
        Self {
            settings,
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
            .choose_multiple(&mut self.rng, self.settings.choices - 1);
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

    // FIXME: Should you count a question as "learned" if you needed to use a hint?
    fn get_hint(&mut self) {
        if let Some(ref q) = self.question {
            if self.choices.len() > 2 {
                let answer = q.borrow().peek().to_string();
                let wrong = self
                    .choices
                    .iter()
                    .enumerate()
                    .filter(|&(_, s)| s != &answer)
                    .map(|(i, _)| i)
                    .choose(&mut self.rng)
                    .unwrap();
                self.choices.remove(wrong);
            }
        }
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

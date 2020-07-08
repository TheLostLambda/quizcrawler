use super::data::{Question, QuestionRef, QuestionVariant, Section};
use derive_more::{Add, Sum};
use rand::{prelude::*, seq::IteratorRandom};
use std::{
    cell::RefCell,
    cmp,
    collections::{HashMap, HashSet},
    rc::Rc,
};
use uuid::Uuid;

// Like QuestionRef, this allows for multiple ownership, run-time borrow-checking
// and holds a trait-object (anything that implements the Quiz trait). This
// enables dynamic dispatch and makes it possible to store a Vec of QuizRefs
// that refer to different types of quizzes
pub type QuizRef = Rc<RefCell<Box<dyn Quiz>>>;

#[derive(Clone, Copy)]
pub struct QuizProgress {
    pub questions: usize,
    pub learned: usize,
    pub score: Option<f64>,
}

#[derive(Clone, Copy, Add, Sum)]
pub struct QuestionProgress {
    pub correct: usize,
    pub seen: usize,
    pub hints: f64,
}

#[derive(Default, Clone)]
pub struct QuestionCtx {
    pub path: Vec<String>,
    pub siblings: Vec<QuestionRef>,
}

// FIXME: Ensure that all "settings" structs implement Copy
#[derive(Clone, Copy)]
pub struct DSettings {
    pub recursive: bool,
    pub quiz_length: usize,
}

impl Default for DSettings {
    fn default() -> Self {
        DSettings {
            recursive: false,
            quiz_length: 20,
        }
    }
}

pub struct Dispatcher {
    questions: Vec<QuestionRef>,
    quizzes: Vec<QuizRef>,
    reference: HashMap<Uuid, (Question, Vec<String>)>, // FIXME: Should I really be lumping Question and Vec<String> together?
    settings: DSettings,
    rng: ThreadRng,
}

// FIXME: Should this use the builder pattern?
impl Dispatcher {
    /// Set the list of `Question`'s to ask and `Quiz`'s to be dispatched
    pub fn new(settings: DSettings, section: &Section) -> Self {
        // FIXME: Add some explanations
        #[derive(Default)]
        struct TraverseCtx {
            questions: Vec<QuestionRef>,
            reference: HashMap<Uuid, (Question, Vec<String>)>,
            settings: DSettings,
        }
        fn traverse_section(ctx: &mut TraverseCtx, mut path: Vec<String>, section: &Section) {
            path.push(section.name.clone());
            for q in &section.questions {
                ctx.questions.push(Rc::clone(q));
                ctx.reference.insert(
                    q.borrow().id,
                    (RefCell::clone(q).into_inner(), path.clone()),
                );
            }
            if ctx.settings.recursive {
                for c in &section.children {
                    traverse_section(ctx, path.clone(), &c);
                }
            }
        }
        let mut ctx = TraverseCtx {
            settings,
            ..Default::default()
        };
        traverse_section(&mut ctx, Vec::new(), section);
        Self {
            questions: ctx.questions,
            quizzes: Vec::new(),
            reference: ctx.reference,
            settings,
            rng: thread_rng(),
        }
    }

    pub fn register_quiz(&mut self, quiz: impl Quiz + 'static) {
        self.quizzes.push(Rc::new(RefCell::new(Box::new(quiz))));
    }

    /// Returns the number of questions in the set, how many have been learned,
    /// and the current score as a percentage
    pub fn progress(&self) -> QuizProgress {
        let questions = cmp::min(self.settings.quiz_length, self.questions.len());
        QuizProgress {
            questions,
            learned: questions - self.remaining_questions().len(),
            score: self.score(),
        }
    }

    // FIXME: Add a configurable mastery threshold for progression
    fn remaining_questions(&self) -> Vec<QuestionRef> {
        let mut todo: Vec<_> = self.reference.values().collect();
        todo.sort_unstable_by_key(|(q, _)| q.mastery);
        self.questions
            .iter()
            .cloned()
            .filter(|q| {
                todo.iter()
                    .take(self.settings.quiz_length)
                    .any(|(x, _)| x.id == q.borrow().id)
                    && self.question_progress(q).correct < 1
            })
            .collect()
    }

    fn question_progress(&self, question: &QuestionRef) -> QuestionProgress {
        let question = question.borrow();
        let (ref_question, _) = self.reference.get(&question.id).unwrap();
        let correct = question.correct - ref_question.correct;
        let seen = question.seen - ref_question.seen;
        let hints = question.hints - ref_question.hints;
        QuestionProgress {
            correct,
            seen,
            hints,
        }
    }

    fn score(&self) -> Option<f64> {
        let overall: QuestionProgress = self
            .questions
            .iter()
            .map(|q| self.question_progress(q))
            .sum();
        if overall.seen > 0 {
            Some((overall.correct as f64 - overall.hints) / overall.seen as f64 * 100.0)
        } else {
            None
        }
    }
}

impl Iterator for Dispatcher {
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
        let siblings: Vec<_> = self
            .questions
            .iter()
            .cloned()
            .filter(|q| quiz.borrow().is_applicable(&q.borrow()))
            .collect();
        let (_, path) = self.reference.get(&question.borrow().id).unwrap(); // FIXME: Spooky unwrap
        let path = path.clone();
        {
            let mut quiz = quiz.borrow_mut();
            quiz.set_context(&QuestionCtx { path, siblings });
            quiz.set_question(question);
        }
        Some(quiz)
    }
}
// FIXME: Add a `get_context` function so the renderer can handle paths
// FIXME: Create a context struct with the path and other questions
pub trait Quiz {
    /// Sets the `Question` to be asked
    fn set_question(&mut self, q: QuestionRef);
    /// Sets the context that this Quiz belongs in
    fn set_context(&mut self, ctx: &QuestionCtx);
    /// Gets the context that this Quiz belongs in
    fn get_context(&self) -> &QuestionCtx;
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
#[derive(Clone, Copy)]
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
    settings: MCSettings,
    question: Option<QuestionRef>,
    context: QuestionCtx,
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
        let answer_bank: HashSet<String> = self
            .context
            .siblings
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

    fn set_context(&mut self, ctx: &QuestionCtx) {
        self.context = ctx.to_owned();
    }

    fn get_context(&self) -> &QuestionCtx {
        &self.context
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
                let hints = 1.0 - self.choices.len() as f64 / self.settings.choices as f64;
                let (correct, answer) = q.answer(&choices[n - 1], hints);
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

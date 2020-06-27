#![allow(clippy::new_ret_no_self)]
use super::logic;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
    time::SystemTime,
};
use uuid::Uuid;

// The reference-counter (Rc) allows for several structures to own this type at
// once (avoiding messy lifetime tracking). The RefCell allows for the mutable
// borrowing of the underlying question at run-time. This shifts borrow-checking
// to run-time as the static checker can't prove this multiple ownership and
// mutation is safe. No copies means that mutation in one part of the program is
// always seen in the others.
pub type QuestionRef = Rc<RefCell<Question>>;

// I really don't know how I feel about these public fields...
#[derive(Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub questions: Vec<QuestionRef>,
    pub children: Vec<Section>,
}

impl Section {
    // Not sure if I should keep this around...
    pub fn new(name: String, children: Vec<Section>, questions: Vec<Question>) -> Section {
        let questions = questions
            .into_iter()
            .map(|c| Rc::new(RefCell::new(c)))
            .collect();
        Section {
            name,
            children,
            questions,
        }
    }

    pub fn is_parent(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn child_at_path(&self, path: &[impl AsRef<str>]) -> Option<&Section> {
        let mut current = self;
        for name in path {
            // FIXME: What happens when there are sections with the same name?
            current = current.children.iter().find(|c| c.name == name.as_ref())?;
        }
        Some(current)
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Strictness {
    Exact,
    Trimmed,
    Caseless,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Question {
    pub id: Uuid,
    pub data: QuestionVariant,
    pub comp_level: Strictness, // FIXME: Should this be moved up to the state machine?
    pub mastery: u8,            // 0-10 (Leitner System)
    pub correct: usize,
    pub seen: usize,
    pub last_correct: SystemTime,
}

/// Question Enum
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub enum QuestionVariant {
    Term(Term),
    List(List),
    Bullet(Bullet),
}

/// Flash Cards
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Term {
    term: String,
    definition: String,
    inverted: bool,
}

// Not a fan of this section having children, but I'll allow it for now
// Also, term? item? body? Pick one.
#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct List {
    order: u32,
    item: String,
    details: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Debug)]
pub struct Bullet {
    body: String,
}

impl Term {
    pub fn new(term: String, definition: String) -> Question {
        Question::new(QuestionVariant::Term(Self {
            term,
            definition,
            inverted: false,
        }))
    }

    pub fn _flip(&mut self) {
        self.inverted = !self.inverted;
    }
}

impl List {
    // FIXME: Slice of strings here? &[] not Vec?
    pub fn new(order: u32, item: String, details: Vec<String>) -> Question {
        Question::new(QuestionVariant::List(Self {
            order,
            item,
            details,
        }))
    }
}

impl Bullet {
    pub fn new(body: String) -> Question {
        Question::new(QuestionVariant::Bullet(Self { body }))
    }
}

impl Question {
    pub fn new(data: QuestionVariant) -> Question {
        Question {
            id: Uuid::new_v4(),
            data,
            comp_level: Strictness::Trimmed,
            mastery: 0,
            correct: 0,
            seen: 0,
            last_correct: SystemTime::now(),
        }
    }

    pub fn ask(&self) -> &str {
        match &self.data {
            QuestionVariant::Term(t) => {
                if t.inverted {
                    &t.definition
                } else {
                    &t.term
                }
            }
            _ => todo!(),
        }
    }

    pub fn peek(&self) -> &str {
        match &self.data {
            QuestionVariant::Term(t) => {
                if t.inverted {
                    &t.term
                } else {
                    &t.definition
                }
            }
            _ => todo!(),
        }
    }

    pub fn answer(&mut self, ans: &str) -> (bool, &str) {
        let right_ans = self.peek().to_owned();
        let correct = logic::check_answer(ans, &right_ans, &self.comp_level);
        self.seen += 1;
        if correct {
            self.correct += 1;
            self.increment_mastery();
            self.last_correct = SystemTime::now();
        } else {
            self.decrement_mastery();
        }
        (correct, self.peek())
    }

    pub fn override_correct(&mut self) {
        self.correct += 1;
        if self.correct > self.seen {
            self.correct = self.seen;
        } else {
            // Give back the mastery lost from the wrong answer
            self.increment_mastery();
            // Give the new mastery for being correct
            self.increment_mastery();
        }
    }

    pub fn _get_variant(&mut self) -> &mut QuestionVariant {
        &mut self.data
    }

    // FIXME: Make the range of min and max mastery configurable
    fn increment_mastery(&mut self) {
        if self.mastery < 10 {
            self.mastery += 1;
        }
    }

    fn decrement_mastery(&mut self) {
        if self.mastery > 0 {
            self.mastery -= 1;
        }
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Eq for Question {}

impl Hash for Question {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_term(t: &str, d: &str) -> Question {
        Term::new(t.to_string(), d.to_string())
    }

    #[test]
    fn variant_equality() {
        let a = make_term("Bonjour", "Hello");
        let b = make_term("Bonjour", "Hello");
        assert_eq!(a, b);
        assert_ne!(a.id, b.id);
    }

    #[test]
    fn mastery_lower_bound() {
        let mut a = make_term("", "right");
        for _ in 1..100 {
            a.answer("wrong");
        }
        assert_eq!(a.mastery, 0);
    }

    #[test]
    fn mastery_upper_bound() {
        let mut a = make_term("", "right");
        for _ in 1..100 {
            a.answer("right");
        }
        assert_eq!(a.mastery, 10);
    }

    #[test]
    fn mastery_up_and_down() {
        let mut a = make_term("", "right");
        a.answer("right");
        a.answer("right");
        a.answer("right");
        a.answer("wrong");
        a.answer("right");
        a.answer("wrong");
        assert_eq!(a.mastery, 2);
    }

    #[test]
    fn last_correct_time() {
        let mut a = make_term("", "right");
        let t1 = a.last_correct.clone();
        a.answer("wrong");
        a.answer("wrong");
        let t2 = a.last_correct.clone();
        a.answer("right");
        let t3 = a.last_correct.clone();
        assert_eq!(t1, t2);
        assert!(t3 > t2);
    }

    #[test]
    fn override_correct_works() {
        let mut a = make_term("", "right");
        a.answer("right");
        a.answer("wrong");
        assert!(a.correct < a.seen);
        assert_eq!(a.mastery, 0);
        a.override_correct();
        assert_eq!(a.correct, a.seen);
        assert_eq!(a.mastery, 2);
    }

    #[test]
    fn override_correct_cant_be_cheated() {
        let mut a = make_term("", "right");
        a.answer("right");
        assert_eq!(a.correct, a.seen);
        assert_eq!(a.mastery, 1);
        a.override_correct();
        assert_eq!(a.correct, a.seen);
        assert_eq!(a.mastery, 1);
    }

    #[test]
    fn ask_term() {
        let a = make_term("question", "right");
        assert_eq!(a.ask(), "question");
    }

    #[test]
    fn peek_term() {
        let a = make_term("question", "right");
        assert_eq!(a.peek(), "right");
        assert_eq!(a.seen, 0);
    }
}

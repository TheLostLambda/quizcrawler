use super::logic;
use serde::{Deserialize, Serialize};
use std::{
    cell::RefCell,
    hash::{Hash, Hasher},
    rc::Rc,
    time::SystemTime,
};

// FIXME: Add some explanations
pub type QuestionRef = Rc<RefCell<Question>>;

// FIXME: Where do I belong
pub type Path = Vec<String>;

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

#[derive(Clone, Serialize, Deserialize)]
pub enum Strictness {
    Exact,
    Trimmed,
    Caseless,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Question {
    pub data: QuestionVariant,
    pub comp_level: Strictness, // FIXME: Should this be moved up to the state machine?
    pub mastery: u8,            // 0-10 (Leitner System)
    pub correct: usize,
    pub seen: usize,
    pub last_correct: SystemTime, // FIXME: Should be the time of the last correct answer!
}

/// Question Enum
#[derive(Clone, Serialize, Deserialize)]
pub enum QuestionVariant {
    Term(Term),
    List(List),
    Bullet(Bullet),
}

/// Flash Cards
#[derive(Clone, Serialize, Deserialize)]
pub struct Term {
    term: String,
    definition: String,
    inverted: bool,
}

// Not a fan of this section having children, but I'll allow it for now
// Also, term? item? body? Pick one.
#[derive(Clone, Serialize, Deserialize)]
pub struct List {
    order: u32,
    item: String,
    details: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
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
        self.increment_mastery();
        if self.correct > self.seen {
            self.correct = self.seen;
        }
    }

    pub fn _get_variant(&mut self) -> &mut QuestionVariant {
        &mut self.data
    }

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
        match (&self.data, &other.data) {
            (QuestionVariant::Term(s), QuestionVariant::Term(o)) => s.term == o.term,
            (QuestionVariant::List(s), QuestionVariant::List(o)) => {
                s.order == o.order && s.item == o.item
            }
            (QuestionVariant::Bullet(s), QuestionVariant::Bullet(o)) => s.body == o.body,
            _ => false,
        }
    }
}

impl Eq for Question {}

impl Hash for Question {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.data {
            QuestionVariant::Term(s) => s.term.hash(state),
            QuestionVariant::List(s) => {
                s.order.hash(state);
                s.item.hash(state);
            }
            QuestionVariant::Bullet(s) => s.body.hash(state),
        }
    }
}

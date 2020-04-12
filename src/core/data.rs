use crate::core::logic;
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, rc::Rc, time::SystemTime};

// I really don't know how I feel about these public fields...
#[derive(Clone, Serialize, Deserialize)]
pub struct Section {
    pub name: String,
    pub questions: Rc<Vec<Rc<RefCell<Question>>>>,
    pub children: Vec<Section>,
}

impl Section {
    // Not sure if I should keep this around...
    pub fn new(name: String, children: Vec<Section>, questions: Vec<Question>) -> Section {
        let questions = questions.into_iter().map(|c| Rc::new(RefCell::new(c))).collect();
        Section {
            name,
            // FIXME: Ew
            children: children,
            questions: Rc::new(questions),
        }
    }

    pub fn is_parent(&self) -> bool {
        !self.children.is_empty()
    }

    pub fn child_at_path(&self, path: &[impl AsRef<str>]) -> Option<&Section> {
        let mut current = self;
        for name in path {
            // FIXME: What happens when there are sections with the same name?
            current = current.children.iter().find(|c| &c.name == name.as_ref())?;
        }
        Some(current)
    }

    // FIXME: Ew
    pub fn get_questions(&self) -> Rc<Vec<Rc<RefCell<Question>>>> {
        Rc::clone(&self.questions)
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
    pub seen: usize,
    pub correct: usize,
    pub atime: SystemTime,
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

    pub fn flip(&mut self) {
        self.inverted = !self.inverted;
    }
}

impl List {
    // Slice of strings here? &[] not Vec?
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
            seen: 0,
            correct: 0,
            atime: SystemTime::now(),
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
        }
        (correct, self.peek())
    }

    pub fn override_correct(&mut self) {
        self.correct += 1;
        if self.correct > self.seen {
            self.correct = self.seen;
        }
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        match (&self.data, &other.data) {
            (QuestionVariant::Term(s), QuestionVariant::Term(o)) => s.term == o.term,
            _ => false,
        }
    }
}

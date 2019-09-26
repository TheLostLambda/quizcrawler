use crate::core::logic;

// Just drafting ideas here
#[derive(Debug)]
pub struct Section {
    name: String,
    children: Vec<Section>,
    questions: Vec<Question>,
}

impl Section {
    // Gah, this is kinda pointless right now...
    pub fn new(name: String, children: Vec<Section>, questions: Vec<Question>) -> Section {
        Section { name, children, questions }
    }
}
// Is it possible to match an identical regex group a second time?
// If so, I should write a regex that matches *'s followed by whitespace and
// should continue until the same number of *'s are encountered again (sibling
// section) or the end of the file is reached. I think I should be able to do
// that with groups.

// Two passes follow, first the body of the section is parsed into questions
// (this may actually involve several subpasses, one for each question type)
// then the body is scanned for children and the process repeats.

// I need to find a way to not double scan the questions within the children —
// unless this becomes desirable. Perhaps the children pass happens first and
// those matches are snipped from the string before questions are scanned
// for. It's also possible just to build a second, negated regex that matches
// things that aren't children and scrapes questions from those.

#[derive(Debug, Clone)]
pub enum Strictness {
    Exact,
    Trimmed,
    Caseless,
}

#[derive(Debug, Clone)]
pub struct Question {
    data: QuestionVariant,
    comp_level: Strictness,
    mastery: u8, // (0-5 or 0-10?)
    seen: u32,
    correct: u32,
}

/// Question Enum
#[derive(Debug, Clone)]
pub enum QuestionVariant {
    Term(Term),
    List(List),
    Bullet(Bullet),
    _Equation(),
}

/// Flash Cards
#[derive(Debug, Clone)]
pub struct Term {
    term: String,
    definition: String,
    inverted: bool,
}

// Not a fan of this section having children, but I'll allow it for now
// Also, term? item? body? Pick one.
#[derive(Debug, Clone)]
pub struct List {
    order: u32,
    item: String,
    details: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Bullet {
    body: String,
}

impl Term {
    pub fn new(term: String, definition: String) -> Question {
        Question::new(
            QuestionVariant::Term(Self {
                term,
                definition,
                inverted: false,
            })
        )
    }
    
    pub fn flip(&mut self) {
        self.inverted = !self.inverted;
    }
}

impl List {
    // Slice of strings here? &[] not Vec?
    pub fn new(order: u32, item: String, details: Vec<String>) -> Question {
        Question::new(
            QuestionVariant::List(Self {
                order,
                item,
                details,
            })
        )
    }
}

impl Bullet {
    pub fn new(body: String) -> Question {
        Question::new(
            QuestionVariant::Bullet(Self {
                body,
            })
        )
    }
}

impl Question {
    pub fn new(data: QuestionVariant) -> Question {
        Question {
            data,
            comp_level: Strictness::Trimmed,
            mastery: 0, // (0-5 or 0-10?)
            seen: 0,
            correct: 0,
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
            _ => "",
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
            _ => "",
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

    // I'm not a massive fan of this...
    pub fn inner(&mut self) -> &QuestionVariant {
        &self.data
    }
    
    pub fn _set_comp_level(&mut self, cl: Strictness) {
        self.comp_level = cl;
    }

    pub fn _get_comp_level(&self) -> &Strictness {
        &self.comp_level
    }
    // Replace this and times_correct with a single "mastery" value
    pub fn _times_seen(&self) -> u32 {
        self.seen
    }

    pub fn _times_correct(&self) -> u32 {
        self.correct
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

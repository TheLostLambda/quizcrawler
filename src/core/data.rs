use crate::core::logic;

// Just drafting ideas here
pub struct Section {
    name: String,
    children: Vec<Section>,
    questions: Vec<Question>,
}

impl Section {
    // Gah, this is kinda pointless right now...
    pub fn new(name: &str, children: Vec<Section>, questions: Vec<Question>) -> Section {
        Section { name: name.to_owned(), children, questions }
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

/// Question Enum
#[derive(Debug, Clone)]
pub enum Question {
    Term(Term),
    _List(),
    _Bullet(),
    _Equation(),
}

/// Flash Cards
#[derive(Debug, Clone)]
pub struct Term {
    term: String,
    definition: String,
    inverted: bool,
    seen: u32,
    correct: u32,
    comp_level: u8,
}

impl Term {
    pub fn new(t: &str, d: &str) -> Question {
        Question::Term(Self {
            term: t.to_owned(),
            definition: d.to_owned(),
            inverted: false,
            seen: 0,
            correct: 0,
            comp_level: 1,
        })
    }
    pub fn flip(&mut self) {
        self.inverted = !self.inverted;
    }
}

impl Question {
    pub fn ask(&self) -> &str {
        match self {
            Question::Term(t) => {
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
        match self {
            Question::Term(t) => {
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
        let right_ans = self.peek().to_owned(); // FIXME: This feels unneeded
        let correct = match self {
            Question::Term(t) => {
                let correct = logic::check_answer(ans, &right_ans, t.comp_level);
                t.seen += 1;
                if correct {
                    t.correct += 1;
                }
                correct
            }
            _ => false,
        };
        (correct, self.peek())
    }

    pub fn _set_comp_level(&mut self, cl: u8) {
        match self {
            Question::Term(t) => {
                t.comp_level = cl;
            }
            _ => (),
        }
    }

    pub fn _get_comp_level(&self) -> u8 {
        match self {
            Question::Term(t) => t.comp_level,
            _ => 0,
        }
    }
    // Replace this and times_correct with a single "mastery" value
    pub fn _times_seen(&self) -> u32 {
        match self {
            Question::Term(t) => t.seen,
            _ => 0,
        }
    }

    pub fn _times_correct(&self) -> u32 {
        match self {
            Question::Term(t) => t.correct,
            _ => 0,
        }
    }

    pub fn override_correct(&mut self) {
        match self {
            Question::Term(t) => {
                t.correct += 1;
                if t.correct > t.seen {
                    t.correct = t.seen;
                }
            }
            _ => (),
        }
    }
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Question::Term(s), Question::Term(o)) => s.term == o.term,
            _ => false,
        }
    }
}

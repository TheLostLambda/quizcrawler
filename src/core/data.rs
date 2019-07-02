use crate::core::logic;

/// All types of questions implement this trait
pub trait Question {
    /// Ask the question
    fn ask(&self) -> &str;
    /// Peek at the correct answer
    fn peek(&self) -> &str;
    /// This should increment the times seen and if correct, increment the
    /// times correct
    fn answer(&mut self, ans: &str) -> (bool, &str);
    /// Changes how strict equality comparison is
    fn set_comp_level(&mut self, cl: u8);
    fn get_comp_level(&self) -> u8;
    fn times_seen(&self) -> u32;
    fn times_correct(&self) -> u32;
    fn override_correct(&mut self);
    fn score(&self) -> f64 {
        self.times_correct() as f64 / self.times_seen() as f64
    }
}

// Just drafting ideas here
pub struct Section {
    title: &'static str, // Should I just make this a String?
    questions: Vec<Box<dyn Question>>,
    children: Vec<Box<Section>>,
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

/// Flash Cards
#[derive(Debug, Clone)]
pub struct Flash {
    term: String,
    definition: String,
    inverted: bool,
    seen: u32,
    correct: u32,
    comp_level: u8,
}

impl Flash {
    pub fn new(t: &str, d: &str) -> Flash {
        Flash {
            term: t.to_owned(),
            definition: d.to_owned(),
            inverted: false,
            seen: 0,
            correct: 0,
            comp_level: 1,
        }
    }
    pub fn flip(&mut self) {
        self.inverted = !self.inverted;
    }
}

impl Question for Flash {
    fn ask(&self) -> &str {
        if self.inverted { &self.definition } else { &self.term }
    }
    fn peek(&self) -> &str {
        if self.inverted { &self.term } else { &self.definition }
    }
    fn answer(&mut self, ans: &str) -> (bool, &str) {
        let correct = logic::check_answer(ans, self.peek(), self.comp_level);
        self.seen += 1;
        if correct {
            self.correct += 1;
        }
        (correct, self.peek())
    }
    fn set_comp_level(&mut self, cl: u8) {
        self.comp_level = cl;
    }
    fn get_comp_level(&self) -> u8 {
        self.comp_level
    }
    fn times_seen(&self) -> u32 {
        self.seen
    }
    fn times_correct(&self) -> u32 {
        self.correct
    }
    fn override_correct(&mut self) {
        self.correct += 1;
        if self.correct > self.seen {
            self.correct = self.seen;
        }
    }
}

impl PartialEq for Flash {
    fn eq(&self, other: &Self) -> bool {
        self.term == other.term
    }
}

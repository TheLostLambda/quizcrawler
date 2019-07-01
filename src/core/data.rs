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

use crate::core::logic;

/// All types of questions implement this trait
pub trait Question {
    /// This should increment the times seen
    fn ask(&mut self) -> &str;
    /// If correct, increment the times correct
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
    fn ask(&mut self) -> &str {
        self.seen += 1; // Maybe move this to answer?
        if self.inverted { &self.definition } else { &self.term }
    }
    fn answer(&mut self, ans: &str) -> (bool, &str) {
        let correct_ans = if self.inverted { &self.term } else { &self.definition };
        let correct = logic::check_answer(ans, correct_ans, self.comp_level);
        if correct {
            self.correct += 1;
        }
        (correct, correct_ans)
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

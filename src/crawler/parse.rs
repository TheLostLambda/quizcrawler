use super::data::Crawler;
use super::util;
use crate::core::data::*;
use onig::Regex;
use std::fs;
use std::path::Path;

impl Crawler {
    /// Parse flashcards from str
    // I should also make this return the unmatched portions of string so that the
    // notes that aren't flashcards can be passed on to the next question parser. By
    // passing only the remainder of the string, the parsers can become
    // progressively more general.
    fn parse_terms(&self, src: &str) -> (String, Vec<Question>) {
        let rules = self.term.as_ref().unwrap();
        let re_str = format!(
            "{}({}){}({}){}",
            rules.leader, rules.term, rules.separator, rules.definition, rules.terminator,
        );
        let matches = Regex::new(&re_str).unwrap();
        let mut remainder = String::from(src);
        let mut questions = Vec::new();
        for caps in matches.captures_iter(src) {
            remainder = remainder.replace(caps.at(0).unwrap(), "");
            questions.push(Term::new(
                util::unflow_string(caps.at(1).unwrap()),
                util::unflow_string(caps.at(2).unwrap()),
            ));
        }
        (remainder, questions)
    }

    fn parse_lists(&self, src: &str) -> (String, Vec<Question>) {
        let rules = self.list.as_ref().unwrap();
        let re_str = format!(
            "({}){} ({})\\s*({}{})*{}",
            rules.numerals,
            rules.leader,
            rules.body,
            rules.sub_leader,
            rules.body,
            rules.terminator
        );
        let sub_re_str = format!(
            "{}({}){}",
            rules.sub_leader, rules.body, rules.sub_terminator
        );
        // Yikes, this needs a lot of refactoring
        let matches = Regex::new(&re_str).unwrap();
        let sub_matches = Regex::new(&sub_re_str).unwrap();
        let mut remainder = String::from(src);
        let mut questions = Vec::new();
        for caps in matches.captures_iter(src) {
            remainder = remainder.replace(caps.at(0).unwrap(), "");
            let details = match caps.at(3) {
                Some(b) => sub_matches
                    .captures_iter(b)
                    .map(|sub_caps| util::unflow_string(sub_caps.at(1).unwrap()))
                    .collect(),
                None => Vec::new(),
            };
            questions.push(List::new(
                caps.at(1).unwrap().parse().unwrap(),
                util::unflow_string(caps.at(2).unwrap()),
                details,
            ));
        }
        (remainder, questions)
    }

    fn parse_bullets(&self, src: &str) -> (String, Vec<Question>) {
        let rules = self.bullet.as_ref().unwrap();
        let re_str = format!("{}({}){}", rules.leader, rules.body, rules.terminator,);
        // There is some duplication to fix here!
        let matches = Regex::new(&re_str).unwrap();
        let mut remainder = String::from(src);
        let mut questions = Vec::new();
        for caps in matches.captures_iter(src) {
            remainder = remainder.replace(caps.at(0).unwrap(), "");
            questions.push(Bullet::new(util::unflow_string(caps.at(1).unwrap())));
        }
        (remainder, questions)
    }

    pub fn parse_questions(&self, src: &str) -> Vec<Question> {
        let mut questions = Vec::new();
        let (src, chunk) = self.parse_terms(src);
        questions.extend(chunk);
        let (src, chunk) = self.parse_lists(&src);
        questions.extend(chunk);
        let (_, chunk) = self.parse_bullets(&src);
        questions.extend(chunk);
        questions
    }

    /// Get section
    pub fn parse_sections(&self, src: &str) -> Vec<Section> {
        let rules = self.section.as_ref().unwrap();
        let sect_re_str = format!(
            "(^\\{}+ )({})\\s({})((?=^\\1)|\\z)",
            rules.marker, rules.name, rules.body
        );
        let quest_re_str = format!("({})(^\\{}+ |\\z)", rules.body, rules.marker);
        let sect_re = Regex::new(&sect_re_str).unwrap();
        let quest_re = Regex::new(&quest_re_str).unwrap();
        sect_re
            .captures_iter(src)
            .map(|caps| {
                let name = caps.at(2).unwrap();
                let body = caps.at(3).unwrap();
                let children = self.parse_sections(body);
                let question_body = quest_re.captures(body).unwrap().at(1).unwrap();
                let questions = self.parse_questions(question_body);
                Section::new(name.to_owned(), children, questions)
            })
            .collect()
    }

    // This feels a tad out of place
    // FIXME: Needs some testing too
    pub fn parse_file(&self, filename: &str) -> Section {
        let src = fs::read_to_string(filename).unwrap();
        Section::new(
            Path::new(filename)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            self.parse_sections(&src),
            Vec::new(),
        )
    }
}

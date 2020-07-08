use super::{data::Crawler, util};
use crate::core::data::*;
use onig::Regex;
use std::{fs, mem, path::Path};

impl Crawler {
    /// Parse flashcards from str
    fn parse_terms(&self, src: &str) -> (String, Vec<Question>) {
        if let Some(rules) = self.term.as_ref() {
            let re_str = format!(
                "{}({}){}({}){}",
                rules.leader, rules.term, rules.separator, rules.definition, rules.terminator,
            );
            let matches = Regex::new(&re_str).unwrap();
            let mut remainder = String::from(src);
            let mut questions = Vec::new();
            for caps in matches.captures_iter(src) {
                remainder = remainder.replace(caps.at(0).unwrap(), "");
                let mut term = util::reflow_string(&self.flow, caps.at(1).unwrap());
                let mut definition = util::reflow_string(&self.flow, caps.at(2).unwrap());
                if rules.flipped.is_some() && rules.flipped.unwrap() {
                    mem::swap(&mut term, &mut definition);
                }
                questions.push(Term::new(term, definition));
            }
            (remainder, questions)
        } else {
            (src.to_owned(), Vec::new())
        }
    }

    fn parse_lists(&self, src: &str) -> (String, Vec<Question>) {
        if let Some(rules) = self.list.as_ref() {
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
                        .map(|sub_caps| util::reflow_string(&self.flow, sub_caps.at(1).unwrap()))
                        .collect(),
                    None => Vec::new(),
                };
                questions.push(List::new(
                    caps.at(1).unwrap().parse().unwrap(),
                    util::reflow_string(&self.flow, caps.at(2).unwrap()),
                    details,
                ));
            }
            (remainder, questions)
        } else {
            (src.to_owned(), Vec::new())
        }
    }

    fn parse_bullets(&self, src: &str) -> (String, Vec<Question>) {
        if let Some(rules) = self.bullet.as_ref() {
            let re_str = format!("{}({}){}", rules.leader, rules.body, rules.terminator,);
            // There is some duplication to fix here!
            let matches = Regex::new(&re_str).unwrap();
            let mut remainder = String::from(src);
            let mut questions = Vec::new();
            for caps in matches.captures_iter(src) {
                remainder = remainder.replace(caps.at(0).unwrap(), "");
                questions.push(Bullet::new(util::reflow_string(
                    &self.flow,
                    caps.at(1).unwrap(),
                )));
            }
            (remainder, questions)
        } else {
            (src.to_owned(), Vec::new())
        }
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
        if let Some(rules) = self.section.as_ref() {
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
        } else {
            Vec::new()
        }
    }

    // This feels a tad out of place
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

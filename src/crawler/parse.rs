use super::data::Crawler;
use crate::core::data::Question;
use crate::core::data::Section;
use crate::core::data::Term;
use onig::Regex;

impl Crawler {
    /// Parse flashcards from str
    // I should also make this return the unmatched portions of string so that the
    // notes that aren't flashcards can be passed on to the next question parser. By
    // passing only the remainder of the string, the parsers can become
    // progressively more general.
    pub fn parse_flashcards(&self, src: &str) -> Vec<Question> {
        let rules = &self.flash;
        let re_str = format!(
            "{}({}){}({}){}",
            rules.leader, rules.term, rules.separator, rules.definition, rules.terminator,
        );
        Regex::new(&re_str)
            .unwrap()
            .captures_iter(src)
            .map(|caps| Term::new(caps.at(1).unwrap(), caps.at(2).unwrap()))
            .collect()
    }

    /// Get section
    // This needs to read from the config. This is testing
    pub fn parse_sections(&self, src: &str) -> Vec<Section> {
        let rules = r"(^\*+ )(.*)\s([\s\S]*?)((?=^\1)|\z)"; // Implement me
        let re = Regex::new(rules).unwrap();
            re.captures_iter(src)
                .map(|caps| {
                    let name = caps.at(2).unwrap();
                    let children = self.parse_sections(caps.at(3).unwrap());
                    let questions = self.parse_flashcards(src); // This isn't right, needs a sectionless string
                    Section::new(name, children, questions)
                })
                .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const config_str: &str = r#"
title = "Borg (Brooks's Org Format)"
exts = ["org"]
[flash]
leader = "- "
term = ".*"
separator = " :: "
definition = "[\\s\\S]*?"
terminator = "(?=\n\\s*[-*0-9]+)"#;

    #[test]
    fn test_parse_flashcards() {
        let data_str = r#"
  - der / die Lehrer(in) :: teacher
  - wissen / weiß / hat gewusst :: to know
  - in der Zwischenzeit :: [in the] meantime"#;

        let crawler = Crawler::new(config_str).unwrap_or_else(|err| {
            panic!(
                "Failed to parse a config from string. The error was: {}\n The config string was: {}",
                err, config_str);
        });
        let cards = crawler.parse_flashcards(data_str);
    }
}

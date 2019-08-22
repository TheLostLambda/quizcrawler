use crate::core::data::Flash;
use crate::crawler::data::FlashConfig;
use onig::Regex;

/// Parse flashcards from str
// I should also make this return the unmatched portions of string so that the
// notes that aren't flashcards can be passed on to the next question parser. By
// passing only the remainder of the string, the parsers can become
// progressively more general.
pub fn flashcards(src: &str, rules: &FlashConfig) -> Vec<Flash> {
    let re_str = format!(
        "{}({}){}({}){}",
        rules.leader, rules.term, rules.separator, rules.definition, rules.terminator,
    );
    Regex::new(&re_str)
        .unwrap()
        .captures_iter(src)
        .map(|caps| Flash::new(caps.at(1).unwrap(), caps.at(2).unwrap()))
        .collect()
}

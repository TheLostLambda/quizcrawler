use crate::crawler::data::FlashConfig;
use crate::core::data::Flash;
use regex::Regex;

/// Parse flashcards from str
pub fn flashcards(src: &str, rules: FlashConfig) -> Vec<Flash> {
    let re_str = format!("{}({}){}({}){}",
                         rules.leader,
                         rules.term,
                         rules.separator,
                         rules.definition,
                         rules.terminator,
    );
    Regex::new(&re_str)
        .unwrap()
        .captures_iter(src)
        .map(|caps| Flash::new(caps.get(1).unwrap().as_str(),
                               caps.get(2).unwrap().as_str()))
        .collect()
}

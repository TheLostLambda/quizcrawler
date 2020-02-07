use super::data::ReflowStrategy;
use onig::Regex;

pub fn reflow_string(strategy: &ReflowStrategy, src: &str) -> String {
    match strategy {
        ReflowStrategy::Unflow => Regex::new(r"\s*\n\s*").unwrap().replace_all(src, " "),
        _ => src.to_string(),
    }
}

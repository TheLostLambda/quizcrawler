use onig::Regex;

pub fn unflow_string(src: &str) -> String {
    let re = Regex::new(r"\s*\n\s*").unwrap();
    re.replace_all(src, " ")
}

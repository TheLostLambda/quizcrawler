use super::data::ReflowStrategy;
use onig::Regex;

pub fn reflow_string(strategy: &ReflowStrategy, src: &str) -> String {
    match strategy {
        ReflowStrategy::Unflow => Regex::new(r"\s*\n\s*")
            .unwrap()
            .replace_all(src.trim(), " "),
        ReflowStrategy::Unindent => {
            if let Some(indent) = Regex::new(r"\n\s*").unwrap().captures(src.trim()) {
                Regex::new(indent.at(0).unwrap())
                    .unwrap()
                    .replace_all(src.trim(), "\n")
            } else {
                src.trim().to_string()
            }
        }
        ReflowStrategy::Preserve => src.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA_STR: &'static str = r#"*verb*
   1) to make something (such as a colour or a painting) less brilliant by
      covering with a thin coat of opaque or semiopaque colour applied with a
      nearly dry brush
   2) to soften the lines or colours of (a drawing) by rubbing lightly
"#;

    #[test]
    fn reflow_unflow() {
        let result = r#"*verb* 1) to make something (such as a colour or a painting) less brilliant by covering with a thin coat of opaque or semiopaque colour applied with a nearly dry brush 2) to soften the lines or colours of (a drawing) by rubbing lightly"#;
        assert_eq!(
            reflow_string(&ReflowStrategy::Unflow, DATA_STR),
            result.to_string()
        );
    }

    #[test]
    fn reflow_unindent() {
        let result = r#"*verb*
1) to make something (such as a colour or a painting) less brilliant by
   covering with a thin coat of opaque or semiopaque colour applied with a
   nearly dry brush
2) to soften the lines or colours of (a drawing) by rubbing lightly"#;
        assert_eq!(
            reflow_string(&ReflowStrategy::Unindent, DATA_STR),
            result.to_string()
        )
    }

    #[test]
    fn reflow_preserve() {
        assert_eq!(
            reflow_string(&ReflowStrategy::Preserve, DATA_STR),
            DATA_STR.to_string()
        );
    }
}

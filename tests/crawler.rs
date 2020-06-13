use insta::assert_ron_snapshot;
use quizcrawler::crawler::data::Crawler;
use std::fs;

const CONF_FILE: &str = "confs/borg.toml";

fn crawler() -> Crawler {
    let conf_str = fs::read_to_string(CONF_FILE).unwrap();
    Crawler::new(&conf_str).unwrap_or_else(|err| {
        panic!(
            "Failed to parse a config from string. The error was: {}\n The config string was: {}",
            err, conf_str
        );
    })
}

#[test]
fn test_parse_terms() {
    let data_str = r#"
* Terms
  - der / die Lehrer(in) :: teacher
  - wissen / weiß / hat gewusst :: to know
  - in der Zwischenzeit :: [in the] meantime"#;

    let cards = crawler().parse_sections(data_str);
    assert_ron_snapshot!(cards, {".**.last_correct" => "[last_correct]"});
}

#[test]
fn test_parse_lists() {
    let data_str = r#"
* Terms
  1) Use numbers to indicate some sort of process or ranking.
     - This is an annotation about this step in the process.
  2) This, for example, happens after the first point.
  3) Or perhaps this is the third most expensive solution."#;

    let cards = crawler().parse_sections(data_str);
    assert_ron_snapshot!(cards, {".**.last_correct" => "[last_correct]"});
}

#[test]
fn test_parse_bullets() {
    let data_str = r#"
* Terms
  - Here is some short, relevant fact regarding this subtopic.
  - And another one! Only use these when there is no better option.
  - Definitions and processes have their own structures."#;

    let cards = crawler().parse_sections(data_str);
    assert_ron_snapshot!(cards, {".**.last_correct" => "[last_correct]"});
}

#[test]
fn test_parse_multiline_terms() {
    let data_str = r#"
* Terms
  - Quizcrawler :: Quizcrawler is an application that, when fed a file of
    class-notes, crawls the structure and generates interactive quizzes that can be
    used as review. It leverages spaced repetition and forced / active recall to
    enhance learning. The gamification of studying should further increase
    engagement and recall.
"#;

    let cards = crawler().parse_sections(data_str);
    assert_ron_snapshot!(cards, {".**.last_correct" => "[last_correct]"});
}

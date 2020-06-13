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

// It's a little bit gross to have crawler stuff in the core test file, but
// generating sections to test is messy without the crawler

// FIXME: I need to decide if data_str should be a constant or a let everywhere
// I'm leaning towards changing things to be `const`

#[test]
fn find_child_at_path() {
    let data_str = r#"
* Theme
** Topic 1
*** Subtopic A
  - Not quite right...
** Topic 2
*** Subtopic A
  - Well done!
"#;

    let path = vec!["Topic 2", "Subtopic A"];
    let section = &crawler().parse_sections(data_str)[0];
    let child = section.child_at_path(&path);
    assert_ron_snapshot!(child, {".**.last_correct" => "[last_correct]"});
}

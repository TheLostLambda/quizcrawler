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

    let pretty_result = r#"Some(
    Section {
        name: "Subtopic A",
        questions: [
            Question {
                data: Bullet(
                    Bullet {
                        body: "Well done!",
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
        ],
        children: [],
    },
)"#;

    let path = vec!["Topic 2", "Subtopic A"];
    let section = &crawler().parse_sections(data_str)[0];
    println!("{:#?}", section);
    let child = format!("{:#?}", section.child_at_path(&path));
    assert_eq!(child, pretty_result);
}

use quizcrawler::crawler::data::Crawler;
use std::fs;

const CONF_FILE: &str = "confs/borg.toml";

fn crawler() -> Crawler {
    let conf_str = fs::read_to_string(CONF_FILE).unwrap();
    Crawler::new(&conf_str).unwrap_or_else(|err| {
        panic!(
            "Failed to parse a config from string. The error was: {}\n The config string was: {}",
            err, conf_str);
    })
}

#[test]
fn test_parse_terms() {
    let data_str = r#"
* Terms
  - der / die Lehrer(in) :: teacher
  - wissen / weiß / hat gewusst :: to know
  - in der Zwischenzeit :: [in the] meantime"#;

    let pretty_result = r#"[
    Section {
        name: "Terms",
        children: [],
        questions: [
            Question {
                data: Term(
                    Term {
                        term: "der / die Lehrer(in)",
                        definition: "teacher",
                        inverted: false,
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
            Question {
                data: Term(
                    Term {
                        term: "wissen / weiß / hat gewusst",
                        definition: "to know",
                        inverted: false,
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
            Question {
                data: Term(
                    Term {
                        term: "in der Zwischenzeit",
                        definition: "[in the] meantime",
                        inverted: false,
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
        ],
    },
]"#;

    let cards = crawler().parse_sections(data_str);
    println!("{:#?}", cards);
    let parsed = format!("{:#?}", cards);
    assert_eq!(parsed, pretty_result);
}

#[test]
fn test_parse_lists() {
    let data_str = r#"
* Terms
  1) Use numbers to indicate some sort of process or ranking.
     - This is an annotation about this step in the process.
  2) This, for example, happens after the first point.
  3) Or perhaps this is the third most expensive solution."#;

    let pretty_result = r#"[
    Section {
        name: "Terms",
        children: [],
        questions: [
            Question {
                data: List(
                    List {
                        order: 1,
                        item: "Use numbers to indicate some sort of process or ranking.",
                        details: [
                            "This is an annotation about this step in the process.",
                        ],
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
            Question {
                data: List(
                    List {
                        order: 2,
                        item: "This, for example, happens after the first point.",
                        details: [],
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
            Question {
                data: List(
                    List {
                        order: 3,
                        item: "Or perhaps this is the third most expensive solution.",
                        details: [],
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
        ],
    },
]"#;

    let cards = crawler().parse_sections(data_str);
    println!("{:#?}", cards);
    let parsed = format!("{:#?}", cards);
    assert_eq!(parsed, pretty_result);
}

#[test]
fn test_parse_bullets() {
    let data_str = r#"
* Terms
  - Here is some short, relevant fact regarding this subtopic.
  - And another one! Only use these when there is no better option.
  - Definitions and processes have their own structures."#;

    let pretty_result = r#"[
    Section {
        name: "Terms",
        children: [],
        questions: [
            Question {
                data: Bullet(
                    Bullet {
                        body: "Here is some short, relevant fact regarding this subtopic.",
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
            Question {
                data: Bullet(
                    Bullet {
                        body: "And another one! Only use these when there is no better option.",
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
            Question {
                data: Bullet(
                    Bullet {
                        body: "Definitions and processes have their own structures.",
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
        ],
    },
]"#;

    let cards = crawler().parse_sections(data_str);
    println!("{:#?}", cards);
    let parsed = format!("{:#?}", cards);
    assert_eq!(parsed, pretty_result);
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

    let pretty_result = r#"[
    Section {
        name: "Terms",
        children: [],
        questions: [
            Question {
                data: Term(
                    Term {
                        term: "Quizcrawler",
                        definition: "Quizcrawler is an application that, when fed a file of class-notes, crawls the structure and generates interactive quizzes that can be used as review. It leverages spaced repetition and forced / active recall to enhance learning. The gamification of studying should further increase engagement and recall.",
                        inverted: false,
                    },
                ),
                comp_level: Trimmed,
                mastery: 0,
                seen: 0,
                correct: 0,
            },
        ],
    },
]"#;

    let cards = crawler().parse_sections(data_str);
    println!("{:#?}", cards);
    let parsed = format!("{:#?}", cards);
    assert_eq!(parsed, pretty_result);
}

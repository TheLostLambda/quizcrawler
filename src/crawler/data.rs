/// This module holds the data types and implementations used by the crawler,
/// but not the core data types of the program. All of the code for the
/// intermediate data representation lives in the core data module.
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
pub enum ReflowStrategy {
    Unflow,
    Unindent,
    Preserve,
}

/// This struct holds all of the configuration data that is parsed from the TOML
#[derive(Debug, Deserialize)]
pub struct Crawler {
    pub title: String,
    pub exts: Vec<String>,
    pub flow: ReflowStrategy, // Should this be in each subsection?
    pub section: Option<SectionConfig>,
    pub term: Option<TermConfig>,
    pub list: Option<ListConfig>,
    pub bullet: Option<BulletConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SectionConfig {
    pub marker: String,
    pub name: String,
    pub body: String,
}

/// This struct holds the regex components needed to extract flashcards
#[derive(Debug, Deserialize)]
pub struct TermConfig {
    pub flipped: Option<bool>,
    pub leader: String,
    pub term: String,
    pub separator: String,
    pub definition: String,
    pub terminator: String,
}

#[derive(Debug, Deserialize)]
pub struct ListConfig {
    pub leader: String,
    pub numerals: String,
    pub body: String,
    pub sub_leader: String,
    pub sub_terminator: String,
    pub terminator: String,
}

#[derive(Debug, Deserialize)]
pub struct BulletConfig {
    pub leader: String,
    pub body: String,
    pub terminator: String,
}

impl Crawler {
    // It's low priority, but this method should be tested somewhere
    pub fn new(toml_str: &str) -> Result<Self, impl Error> {
        toml::from_str(&toml_str)
    }
}

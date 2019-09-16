/// This module holds the data types and implementations used by the crawler,
/// but not the core data types of the program. All of the code for the
/// intermediate data representation lives in the core data module.
use serde::Deserialize;
use std::error::Error;

/// This struct holds all of the configuration data that is parsed from the TOML
#[derive(Debug, Deserialize)]
pub struct Crawler {
    pub title: String,
    pub exts: Vec<String>,
    pub flash: FlashConfig,
}

/// This struct holds the regex components needed to extract flashcards
#[derive(Debug, Deserialize)]
pub struct FlashConfig {
    pub leader: String,
    pub term: String,
    pub separator: String,
    pub definition: String,
    pub terminator: String,
}

impl Crawler {
    // It's low priority, but this method should be tested somewhere
    pub fn new(toml_str: &str) -> Result<Self, impl Error> {
        toml::from_str(&toml_str)
    }
}

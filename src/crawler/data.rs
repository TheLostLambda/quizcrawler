use serde::Deserialize;
use std::error::Error;
use std::fs;

/// This struct holds all of the configuration data that is parsed from the TOML
#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub exts: Vec<String>,
    pub flash: FlashConfig,
}

impl Config {
    pub fn new(file_name: &str) -> Result<Self, Box<dyn Error>> {
        let toml_str = fs::read_to_string(file_name)?;
        Ok(toml::from_str(&toml_str)?)
    }
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

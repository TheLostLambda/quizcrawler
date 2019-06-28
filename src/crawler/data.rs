use crate::crawler::util::*;
use serde_derive::Deserialize;

/// Generic Result for using `?`
type BoxResult<T> = Result<T,Box<dyn std::error::Error>>;

/// This struct holds all of the configuration data that is parsed from the TOML
#[derive(Debug, Deserialize)]
pub struct Config {
    pub title: String,
    pub exts: Vec<String>,
    pub flash: FlashConfig,
}

impl Config {
    pub fn from_file(file_name: &str) -> BoxResult<Config> {
        let toml_str = read_file_as_string(file_name)?;
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

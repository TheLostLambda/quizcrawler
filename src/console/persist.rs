use super::data::Quizcrawler;
use directories::ProjectDirs;
use ron::ser::{to_string_pretty, PrettyConfig};
use std::fs;

fn get_project_dir() -> ProjectDirs {
    ProjectDirs::from("", "", "Quizcrawler")
        .expect("No valid home directory could be found for this user!")
}

// FIXME: This needs error handling, not two unwraps...
pub fn save_state(state: &Quizcrawler) {
    let mut path = get_project_dir().config_dir().to_path_buf();
    fs::create_dir_all(&path).unwrap();
    path.push("saved_tree.ron");
    let ron = to_string_pretty(state, PrettyConfig::new()).unwrap();
    fs::write(path, &ron).unwrap();
}

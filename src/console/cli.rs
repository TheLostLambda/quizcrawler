use crate::crawler::data::Config;
use crate::crawler::util::*;
use crate::crawler::parse;
use crate::console::games;
use structopt::StructOpt;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "Quizcrawler", about = "Automagically generate interactive quizzes from preexisting notes")]
struct QC {
    /// The file containing the notes to be scraped during quiz generation
    #[structopt(parse(from_os_str))]
    notes: PathBuf,
    /// The TOML file containing the grammar used to parse the note file
    #[structopt(parse(from_os_str))]
    recipe: PathBuf,
    /// Reverses the terms and definitions when quizzing flashcards
    #[structopt(short = "f", long = "flipped")]
    flipped: bool
}

pub fn run() {
    let args = QC::from_args();
    let parse_data = read_file_as_string(&args.notes).unwrap();
    let config = Config::from_file(&args.recipe).unwrap();
    let mut flashcards = parse::flashcards(&parse_data, config.flash);
    if args.flipped {
        for card in &mut flashcards {
            card.flip();
        }
    }
    games::mc_quiz(&flashcards);
    //games::flash_quiz(&flashcards);
}

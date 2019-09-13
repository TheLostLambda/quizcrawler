use crate::console::games;
use crate::crawler::data::Config;
use crate::crawler::parse;
use std::fs;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Quizcrawler",
    about = "Automagically generate interactive quizzes from preexisting notes"
)]
struct QC {
    /// The file containing the notes to be scraped during quiz generation
    notes: String,
    /// The TOML file containing the grammar used to parse the note file
    recipe: String,
    /// Reverses the terms and definitions when quizzing flashcards
    #[structopt(short = "f", long = "flipped")]
    flipped: bool,
}

pub fn run() {
    let args = QC::from_args();
    let parse_data = fs::read_to_string(&args.notes).unwrap();
    let config = Config::new(&args.recipe).unwrap();
    let mut flashcards = parse::flashcards(&parse_data, &config.flash);
    if args.flipped {
        for card in &mut flashcards {
            card.flip();
        }
    }
    games::mc_quiz(&flashcards);
    games::flash_quiz(&flashcards);
}

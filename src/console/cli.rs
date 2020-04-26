use super::{
    data::{QCOptions, Quizcrawler},
    util,
};
use crate::crawler::data::Crawler;
use crossterm::event::{self, Event, KeyCode::Char};
use std::{error::Error, fs};
use structopt::StructOpt;
// Tend to these imports! ^

// Interaction in this file!

// This will be broken up as we go along and find better places for things

#[derive(StructOpt)]
#[structopt(
    name = "Quizcrawler",
    about = "Automagically generate interactive quizzes from preexisting notes"
)]
struct QCArgs {
    /// The file containing the notes to be scraped during quiz generation
    notes: String,
    /// The TOML file containing the grammar used to parse the note file
    recipe: String,
    /// Reverses the terms and definitions when quizzing flashcards
    #[structopt(short = "f", long = "flipped")]
    flipped: bool, // This won't stay here long, will move to the UI
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = QCArgs::from_args();
    let crawler_recipe = fs::read_to_string(&args.recipe)?;
    let crawler = Crawler::new(&crawler_recipe)?;
    let tree = crawler.parse_file(&args.notes);

    let mut tui = util::setup_tui()?;

    let mut quizcrawler = Quizcrawler::new(QCOptions::default(), tree);

    loop {
        quizcrawler.tick();
        tui.draw(|mut f| quizcrawler.render(&mut f))?;
        match event::read()? {
            Event::Key(key) if key.code == Char('q') => break,
            Event::Key(key) => quizcrawler.handle_key(key.code),
            _ => continue,
        }
    }

    util::teardown_tui(tui)?;

    Ok(())
}

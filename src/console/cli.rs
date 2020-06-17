use super::{
    data::{QCSettings, Quizcrawler},
    util,
};
use crate::crawler::data::Crawler;
use crossterm::event::{self, Event};
use std::{error::Error, fs};
use structopt::StructOpt;
// FIXME: I really don't belong here...
use crate::core::data::QuestionVariant::Term;
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

    // FIXME: This is a gross hack that doesn't belong here. The ability to
    // toggle options should be exposed by Core
    if args.flipped {
        for q in tree.get_questions(true) {
            if let Term(t) = q.borrow_mut().get_variant() {
                t.flip();
            }
        }
    }

    let mut quizcrawler = Quizcrawler::new(QCSettings::default(), tree);

    let mut tui = util::setup_tui()?;

    while quizcrawler.tick() {
        tui.draw(|mut f| quizcrawler.render(&mut f))?;
        if let Event::Key(key) = event::read()? {
            quizcrawler.handle_key(key);
        }
    }

    util::teardown_tui(tui)?;

    Ok(())
}

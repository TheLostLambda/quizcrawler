use super::{
    data::{QCSettings, Quizcrawler},
    util,
};
use crate::crawler::data::Crawler;
use crossterm::event::{self, Event};
use std::{error::Error, fs};
use structopt::StructOpt;

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
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = QCArgs::from_args();
    let crawler_recipe = fs::read_to_string(&args.recipe)?;
    let crawler = Crawler::new(&crawler_recipe)?;
    let tree = crawler.parse_file(&args.notes);

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

// use crate::core::data::QuestionVariant;
use crate::console::util::*;
use crate::core::data::Section;
use crate::core::games::Game;
use crate::core::games::*;
use crate::crawler::data::Crawler;
use std::char;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io;
use structopt::StructOpt;
use termion::event::Key; // Do I really want this here?
use termion::*;

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

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = QC::from_args();
    // Change the ? to unwrap_or_else so I can include more detail
    let parse_config = fs::read_to_string(&args.recipe)?;
    let crawler = Crawler::new(&parse_config)?;
    // let mut flashcards = crawler.parse_flashcards(&parse_data);
    let section = crawler.parse_file(&args.notes);
    // println!("{:#?}", sections);
    tree_nav(section);
    return Ok(());
    // if args.flipped {
    //     for card in &mut flashcards {
    //         if let QuestionVariant::Term(card) = card.inner() {
    //             card.flip();
    //         }
    //     }
    // }
    // let game = MultipleChoice::new(MCConfig, &flashcards);
    // play_game(game);
    // // games::flash_quiz(&flashcards);
    // Ok(())
}

fn tree_nav(tree: Section) {
    // Press i or ? for more info about the selected item?
    let mut path = Vec::new();
    let mut sel_hist = HashMap::new();
    let section = loop {
        let node = tree.child_at_path(&path).unwrap();
        if !node.is_parent() {
            break node;
        }
        let mut children = node.children().iter().map(|x| x.name());
        new_screen();

        print!("{} > ", tree.name());
        for node in &path {
            print!("{} > ", node);
        }
        println!("\n");

        let selected = sel_hist.entry(path.clone()).or_insert(0);
        // ^ I think that the clone here is okay

        for (id, child) in children.clone().enumerate() {
            // Not a fan of the clone here...
            if id == *selected {
                print!(">")
            } else {
                print!(" ");
            }
            println!(" {}", child);
        }

        match get_valid_key(&[Key::Up, Key::Down, Key::Right, Key::Left]) {
            Key::Up if node.is_parent() && *selected > 0 => *selected -= 1,
            Key::Down if node.is_parent() && *selected < children.len() - 1 => *selected += 1,
            Key::Right if node.is_parent() => path.push(children.nth(*selected).unwrap()),
            Key::Left => {
                path.pop();
            }
            _ => (),
        };
    };
    play_game(MultipleChoice::new(MCConfig, section.questions()));
}

fn play_game(mut game: impl Game) {
    loop {
        // Clear the screen before the next question
        new_screen();
        let (remaining, seen, score) = game.progress();
        print!("{} flashcards to go!", remaining);
        if seen > 0 {
            float_right(&format!("Your score is {:.2}%", score));
        }

        let question = match game.next_question() {
            Some(q) => q,
            None => break,
        };

        println!("\n{}", question);

        println!();

        let choices = game.get_choices();
        let (result, correct_ans) = if choices.len() > 0 {
            let id_shift: u8 = 49;
            let mut valid = Vec::new();
            for (i, opt) in choices.iter().enumerate() {
                let id = char::from(i as u8 + id_shift);
                println!("{}) {} ", id, opt);
                valid.push(id);
            }

            println!();

            // The char to digit thing here is hacky
            let choice_idx = get_valid_char(&valid) as u8 - id_shift;
            // char::from(u8)
            backtrack(choices.len() as u16 + 1);
            game.answer(&choice_idx.to_string())
        } else {
            let mut ans = String::new();
            io::stdin().read_line(&mut ans).unwrap();
            game.answer(&ans)
        };

        let ans_string = format!(", the answer is: {}", correct_ans);
        // Clear the choices before showing the answer. Keep the definition

        if result {
            // Find a cleaner way to do this, maybe with a macro with this all
            // predefined (all the {em} bits).
            print!(
                "{em}{g}Well done",
                em = style::Bold,
                g = color::Fg(color::Green),
            );
        } else {
            print!("{em}{r}Sorry", em = style::Bold, r = color::Fg(color::Red),);
        }

        println!("{}{re}", ans_string, re = style::Reset);

        if override_prompt(!result) {
            game.i_was_right();
        }
    }
}

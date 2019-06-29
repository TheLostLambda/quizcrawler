mod crawler;
mod core;
mod console;

use crate::crawler::data::Config;
use crate::crawler::util::*;
use crate::crawler::parse;
use crate::core::data::Question;
use crate::core::logic;
use rand::prelude::*;
use std::io;

fn main() {
    let parse_data = read_file_as_string("MLT106.org").unwrap();
    let config = Config::from_file("borg.toml").unwrap();
    let mut flashcards = parse::flashcards(&parse_data, config.flash);
    let mut rng = thread_rng();
    let (mut correct, mut seen) = (0.0, 0.0);
    println!("{} flashcards matched", flashcards.len());
    loop {
        let idx = rng.gen_range(0, &flashcards.len());
        let card = &mut flashcards[idx];
        println!("Card {} is: {}", idx, card.ask());
        seen += 1.0;
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        let (result, correct_ans) = card.answer(&ans);
        println!("{}, the answer is: {}", if result { "Well done" }
                 else { "Sorry" }, correct_ans);
        println!("ENTER to continue{}...", if result { "" }
                 else { ", 'o' for manual override"});
        ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if logic::check_answer(&ans, "o", 2) {
            correct += 1.0;
            card.override_correct();
            println!("Score overridden\n");
        }
        if result { correct += 1.0; }
        println!("Your score is {:.2}%", correct / seen * 100.0);
        card.flip();
    }
}

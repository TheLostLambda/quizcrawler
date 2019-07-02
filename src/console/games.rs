use crate::console::util::*;
use crate::core::data::Question;
use crate::core::logic;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::prelude::*;
use termion::*;
use std::io;

pub fn flash_quiz<Q: Question + Clone>(qs: &Vec<Q>) {
    let mut deck = qs.clone();
    let mut rng = thread_rng();
    let (mut correct, mut seen) = (0.0, 0.0);
    loop {
        // Clear the screen before the next question
        new_screen();
        let len = deck.len();
        if len == 0 {
            break;
        }
        println!("{} flashcards to go!", len);
        
        let idx = rng.gen_range(0, len);
        let card = &mut deck[idx];
        println!("Card {} is: {}", idx, card.ask());
        seen += 1.0;
        
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        let (mut result, correct_ans) = card.answer(&ans);

        println!("{}, the answer is: {}", if result { "Well done" }
                 else { "Sorry" }, correct_ans);
        println!("ENTER to continue{}...", if result { "" }
                 else { ", 'o' for manual override"});

        ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        if logic::check_answer(&ans, "o", 2) {
            result = true;
            card.override_correct();
            println!("Score overridden\n");
        }
        
        if result {
            correct += 1.0;
            deck.remove(idx);
        }
        println!("Your score is {:.2}%", correct / seen * 100.0);
    }
} 

pub fn mc_quiz<Q: Question + Clone + PartialEq>(qs: &Vec<Q>) {
    // I need configs for the games. For now, 4 options.
    let mut deck = qs.clone();
    let mut rng = thread_rng();
    let (mut correct, mut seen) = (0.0, 0.0);
    loop {
        // Clear the screen before the next question
        new_screen();
        let len = deck.len();
        if len == 0 {
            break;
        }
        print!("{} flashcards to go!", len);
        if seen != 0.0 {
            float_right(&format!("Your score is {:.2}%",
                                correct / seen * 100.0));
        }
        
        let idx = rng.gen_range(0, len);
        let card = &mut deck[idx];
        println!("\nCard {} is: {}", idx, card.ask());
        seen += 1.0;

        // Use an array and slices here?
        let mut others = qs.clone();
        others.retain(|c| c != card);
        let mut cards = others.into_iter().choose_multiple(&mut rng,3);
        cards.push(card.clone());

        let options = &mut cards.iter()
            .map(|c| c.peek()).collect::<Vec<_>>()[..];
        options.shuffle(&mut rng);
        
        // Is print!("\n") clearer?
        println!("");

        let id_shift = 49;
        let mut valid = Vec::new();
        for (i, opt) in options.iter().enumerate() {
            let id = std::char::from_u32(i as u32 + id_shift).unwrap();
            println!("{}) {} ", id, opt);
            valid.push(id);
        }

        println!("");

        // The char to digit thing here is hacky
        let choice_idx = get_valid_char(&valid) as u32 - id_shift;
        
        let (result, correct_ans) = card.answer(options[choice_idx as usize]);
        let ans_string = format!(", the answer is: {}", correct_ans);
        // Clear the choices before showing the answer. Keep the definition

        backtrack(options.len() as u16 + 1);
        
        if result {
            // Find a cleaner way to do this, maybe with a macro with this all
            // predefined (all the {em} bits).
            print!("{em}{g}Well done",
                   em = style::Bold,
                   g = color::Fg(color::Green),
            );
            correct += 1.0;
            deck.remove(idx);
        } else {
            print!("{em}{r}Sorry",
                   em = style::Bold,
                   r = color::Fg(color::Red),
            );
        }
        
        println!("{}{re}", ans_string, re = style::Reset);
        
        enter_pause();
    }
}

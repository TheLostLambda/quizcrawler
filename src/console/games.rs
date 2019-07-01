use crate::core::data::Question;
use crate::core::logic;
use rand::seq::IteratorRandom;
use rand::seq::SliceRandom;
use rand::prelude::*;
use std::io;

// Remove this
use std::fmt::Debug;

pub fn flash_quiz<Q: Question + Clone>(qs: &Vec<Q>) {
    let mut deck = qs.clone();
    let mut rng = thread_rng();
    let (mut correct, mut seen) = (0.0, 0.0);
    loop {
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

// Remove Debug
pub fn mc_quiz<Q: Question + Clone + PartialEq + Debug>(qs: &Vec<Q>) {
    // I need configs for the games. For now, 4 options.
    let mut deck = qs.clone();
    let mut rng = thread_rng();
    let (mut correct, mut seen) = (0.0, 0.0);
    loop {
        let len = deck.len();
        if len == 0 {
            break;
        }
        println!("{} flashcards to go!", len);

        let idx = rng.gen_range(0, len);
        let mut card = &mut deck[idx];
        println!("Card {} is: {}", idx, card.ask());
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
        
        for (i, opt) in options.iter().enumerate() {
            print!("{}) {} ", std::char::from_u32(i as u32 + 97).unwrap(), opt);
        }

        println!("");
        
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).unwrap();
        let choice_idx = ans.trim().to_lowercase()
            .chars().next().unwrap() as u32 - 97;
        
        let (result, correct_ans) = card.answer(options[choice_idx as usize]);
        println!("{}, the answer is: {}", if result { "Well done" }
                 else { "Sorry" }, correct_ans);
        
        println!("ENTER to continue...");
        io::stdin().read_line(&mut ans).unwrap();
        
        if result {
            deck.remove(idx);
        }
    }
}

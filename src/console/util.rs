use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::*;
use std::io;

/// This clears the screen and moves the cursor to (1,1)
pub fn new_screen() {
    print!("{}{}", clear::All, cursor::Goto(1,1));
}

/// Deletes N lines from above the current position
pub fn backtrack(n: u16) {
    print!("{}\r{}", cursor::Up(n), clear::AfterCursor);
}

pub fn get_valid_char(valid: &Vec<char>) -> char {
    // Figure this out
    //let hidden = cursor::HideCursor::from(io::stdout());
    let _stdout = io::stdout().into_raw_mode().unwrap();
    for k in io::stdin().keys() {
        match k.unwrap() {
            Key::Char('q') => panic!("This needs more grace"),
            Key::Char(c) if valid.contains(&c) => return c,
            _ => continue,
        }
    }
    ' ' // This shouldn't need to be here
}

/// Floats text to be right-aligned
// Make this less brittle
pub fn float_right(text: &str) {
    let (c,_) = terminal_size().unwrap();
    print!("\r{}{}", cursor::Right(c - text.len() as u16), text);
}

pub fn enter_pause() {
    println!("ENTER to continue...");
    let _stdout = io::stdout().into_raw_mode().unwrap();
    for k in io::stdin().keys() {
        match k.unwrap() {
            Key::Char('q') => panic!("This needs more grace"),
            Key::Char('\n') => break,
            _ => continue,
        }
    }
}

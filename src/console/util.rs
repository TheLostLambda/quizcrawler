use std::io;
use std::io::Write;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

/// This clears the screen and moves the cursor to (1,1)
pub fn new_screen() {
    print!("{}{}", clear::All, cursor::Goto(1, 1));
}

/// Deletes N lines from above the current position
pub fn backtrack(n: u16) {
    print!("{}\r{}", cursor::Up(n), clear::AfterCursor);
}

pub fn get_valid_char(valid: &[char]) -> char {
    let keys: Vec<_> = valid.iter().map(|&c| Key::Char(c)).collect();
    if let Key::Char(chr) = get_valid_key(&keys[..]) {
        chr
    } else {
        ' ' // Just appeasing the type-checker...
    }
}

pub fn get_valid_key(valid: &[Key]) -> Key {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();
    stdout.flush().unwrap();
    for k in io::stdin().keys() {
        match k.unwrap() {
            Key::Char('q') => graceful_death(&mut stdout),
            k if valid.contains(&k) => return k,
            _ => continue,
        }
    }
    Key::Null // This shouldn't need to be here
}

/// Floats text to be right-aligned
// Make this less brittle
pub fn float_right(text: &str) {
    let (c, _) = terminal_size().unwrap();
    print!("\r{}{}", cursor::Right(c - text.len() as u16), text);
}

pub fn override_prompt(wrong: bool) -> bool {
    print!("ENTER to continue");
    if wrong {
        println!(", 'o' for manual override...");
    } else {
        println!("...")
    }
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", cursor::Hide).unwrap();
    stdout.flush().unwrap();
    for k in io::stdin().keys() {
        match k.unwrap() {
            Key::Char('q') => graceful_death(&mut stdout),
            Key::Char('\n') => return false,
            Key::Char('o') if wrong => return true,
            _ => continue,
        }
    }
    false // Shouldn't be here... Use break and loop?
}

pub fn graceful_death<W: Write>(term: &mut raw::RawTerminal<W>) {
    write!(term, "{}", cursor::Show).unwrap();
    term.flush().unwrap();
    term.suspend_raw_mode().unwrap();
    std::process::exit(0);
}

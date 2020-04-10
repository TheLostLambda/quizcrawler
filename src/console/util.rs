// use std::io;
// use std::io::Write;
// use termion::event::Key;
// use termion::input::TermRead;
// use termion::raw::IntoRawMode;
// use termion::*;

use crossterm::{terminal, ExecutableCommand};
use std::error::Error;
use std::io::{self, Stdout};
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub type TUI = Terminal<CrosstermBackend<Stdout>>;
pub type Frame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

// Tiny helper functions here!

// /// This clears the screen and moves the cursor to (1,1)
// pub fn new_screen() {
//     print!("{}{}", clear::All, cursor::Goto(1, 1));
// }

// /// Deletes N lines from above the current position
// pub fn backtrack(n: u16) {
//     print!("{}\r{}", cursor::Up(n), clear::AfterCursor);
// }

// pub fn get_valid_char(valid: &[char]) -> Option<char> {
//     let keys: Vec<_> = valid.iter().map(|&c| Key::Char(c)).collect();
//     // This feels a bit verbose
//     if let Key::Char(chr) = get_valid_key(&keys[..]) {
//         Some(chr)
//     } else {
//         None
//     }
// }

// // Use option instead of Key::Null?
// pub fn get_valid_key(valid: &[Key]) -> Key {
//     let mut stdout = io::stdout().into_raw_mode().unwrap();
//     write!(stdout, "{}", cursor::Hide).unwrap();
//     stdout.flush().unwrap();
//     for k in io::stdin().keys() {
//         match k.unwrap() {
//             Key::Char('q') => return Key::Null,
//             k if valid.contains(&k) => return k,
//             _ => continue,
//         }
//     }
//     Key::Null
// }

// /// Floats text to be right-aligned
// // Make this less brittle
// pub fn float_right(text: &str) {
//     let (c, _) = terminal_size().unwrap();
//     print!("\r{}{}", cursor::Right(c - text.len() as u16), text);
// }

// // Make this return an Option. When quitting, return None so that the parent
// // function can handle that fallout. As it stands now, this just nukes the whole
// // program, so there is no "back" button
// pub fn override_prompt(wrong: bool) -> bool {
//     print!("ENTER or SPACE to continue");
//     if wrong {
//         println!(", 'o' for manual override...");
//     } else {
//         println!("...")
//     }
//     let mut stdout = io::stdout().into_raw_mode().unwrap();
//     write!(stdout, "{}", cursor::Hide).unwrap();
//     stdout.flush().unwrap();
//     // Should this be using get_valid_key()?
//     for k in io::stdin().keys() {
//         match k.unwrap() {
//             Key::Char('q') => graceful_death(),
//             Key::Char('\n') | Key::Char(' ') => return false,
//             Key::Char('o') if wrong => return true,
//             _ => continue,
//         }
//     }
//     false // Shouldn't be here... Use break and loop?
// }

pub fn render_titlebar(left: String, spacer: &str, right: String, width: u16) -> String {
    // The 2 comes from each corner taking up one char
    let used_space = left.len() + right.len() + 2;
    if let Some(padding) = (width as usize).checked_sub(used_space) {
        let spacer = spacer.repeat(padding);
        [left, spacer, right].concat()
    } else {
        [left, right].concat()
    }
}

pub fn setup_tui() -> Result<TUI, Box<dyn Error>> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(terminal::EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut tui = Terminal::new(backend)?;
    tui.hide_cursor()?;
    Ok(tui)
}

// Should I make a type alias for Result<(), Box<dyn Error>>?
pub fn teardown_tui(mut tui: TUI) -> Result<(), Box<dyn Error>> {
    terminal::disable_raw_mode()?;
    let stdout = tui.backend_mut();
    stdout.execute(terminal::LeaveAlternateScreen)?;
    tui.show_cursor()?;
    Ok(())
}

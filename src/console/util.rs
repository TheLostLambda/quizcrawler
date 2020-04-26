use crossterm::{terminal, ExecutableCommand};
use std::{
    error::Error,
    io::{self, Stdout},
};
use tui::{backend::CrosstermBackend, Terminal};

pub type TUI = Terminal<CrosstermBackend<Stdout>>;
pub type Frame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

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

pub fn teardown_tui(mut tui: TUI) -> Result<(), Box<dyn Error>> {
    terminal::disable_raw_mode()?;
    let stdout = tui.backend_mut();
    stdout.execute(terminal::LeaveAlternateScreen)?;
    tui.show_cursor()?;
    Ok(())
}

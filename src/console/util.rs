use crossterm::{terminal, ExecutableCommand};
use std::{
    cmp,
    error::Error,
    io::{self, Stdout},
};
use tui::{backend::CrosstermBackend, Terminal};
use unicode_segmentation::UnicodeSegmentation;

pub type TUI = Terminal<CrosstermBackend<Stdout>>;
pub type Frame<'a> = tui::Frame<'a, CrosstermBackend<Stdout>>;

// FIXME: Not sold on this &[impl ToString] business...
pub fn compact_path(path: &[impl ToString], sep: &str, target_len: usize) -> String {
    let mut path: Vec<String> = path.iter().map(ToString::to_string).collect();
    for i in 0..path.len() {
        let path_len = path
            .iter()
            .filter(|&w| w != &path[i])
            .map(|w| grapheme_len(w) + grapheme_len(sep))
            .sum::<usize>();
        let room = target_len.checked_sub(path_len).unwrap_or_default();
        path[i] = compact_title(&path[i], room);
    }
    path.join(sep)
}

pub fn compact_title(title: &str, target_len: usize) -> String {
    // Split into words prefixed with spaces
    let chunks: Vec<_> = title.split_word_bounds().collect();
    let mut words = Vec::new();
    let mut i = 0;
    // FIXME: This could use a second pass for style and sanity
    while i < chunks.len() {
        if let Some(next) = chunks.get(i + 1) {
            if chunks[i].trim().is_empty() && !next.trim().is_empty() {
                words.push(format!("{}{}", chunks[i], next));
                i += 2;
                continue;
            }
        }
        words.push(chunks[i].to_string());
        i += 1;
    }
    // A buffer for building up the compacted string
    let mut compacted = String::new();
    // Calculate how many graphemes are downstream of any given word
    let downstream_len = |i| words[i..].iter().map(grapheme_len).sum::<usize>();
    // Loop through each word, tracking the index of each
    for (i, word) in words.iter().enumerate() {
        // If the number of graphemes downstream plus those already compacted
        // doesn't fit in target_len, compress the current word
        if grapheme_len(&compacted) + downstream_len(i) > target_len {
            // To compress, drop leading whitespace and attempt to get the first
            // grapheme, falling back to the empty string
            compacted += &word.trim_start().graphemes(true).next().unwrap_or_default();
        } else {
            // Otherwise, just push the current word (which includes a leading space)
            compacted += word;
        }
    }
    compacted
}

pub fn grapheme_len(s: impl AsRef<str>) -> usize {
    s.as_ref().graphemes(true).count()
}

pub fn render_titlebar(left: String, spacer: &str, right: String, width: u16) -> String {
    // The 2 comes from each corner taking up one char
    let used_space = grapheme_len(&left) + grapheme_len(&right) + 2;
    let padding = (width as usize).checked_sub(used_space).unwrap_or_default();
    let spacer = spacer.repeat(cmp::max(1, padding));
    [left, spacer, right].concat()
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

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_titlebar_with_space() {
        let titlebar = render_titlebar(
            "This/Is/A/Test/Path".to_string(),
            "─",
            "20 Questions".to_string(),
            60,
        );
        assert_eq!(
            titlebar,
            "This/Is/A/Test/Path───────────────────────────20 Questions"
        );
    }

    #[test]
    fn test_render_titlebar_without_space() {
        let titlebar = render_titlebar(
            "This/Is/A/Test/Path".to_string(),
            "─",
            "20 Questions".to_string(),
            20,
        );
        assert_eq!(titlebar, "This/Is/A/Test/Path─20 Questions");
    }

    #[test]
    fn test_compact_path_plenty_of_space() {
        let path = [
            "German.org",
            "Sorted Class Notes",
            "Grammar",
            "Adjective Endings",
        ];
        assert_eq!(
            compact_path(&path, "/", 55),
            "German.org/Sorted Class Notes/Grammar/Adjective Endings"
        );
    }

    #[test]
    fn test_compact_path_less_space() {
        let path = [
            "German.org",
            "Sorted Class Notes",
            "Grammar",
            "Adjective Endings",
        ];
        assert_eq!(
            compact_path(&path, "/", 36),
            "G/SC Notes/Grammar/Adjective Endings"
        );
    }

    #[test]
    fn test_compact_path_less_space_with_wide_sep() {
        let path = [
            "German.org",
            "Sorted Class Notes",
            "Grammar",
            "Adjective Endings",
        ];
        assert_eq!(
            compact_path(&path, " ⤕ ", 36),
            "G ⤕ SCN ⤕ G ⤕ Adjective Endings"
        );
    }

    #[test]
    fn test_compact_path_hardly_any_space() {
        let path = [
            "German.org",
            "Sorted Class Notes",
            "Grammar",
            "Adjective Endings",
        ];
        assert_eq!(compact_path(&path, "/", 20), "G/SCN/G/A Endings");
    }

    #[test]
    fn test_compact_path_fully_compact() {
        let path = [
            "German.org",
            "Sorted Class Notes",
            "Grammar",
            "Adjective Endings",
        ];
        assert_eq!(compact_path(&path, "/", 10), "G/SCN/G/AE");
    }

    #[test]
    fn test_compact_title_plenty_of_space() {
        let title = "The University of Sheffield";
        assert_eq!(compact_title(title, 40), "The University of Sheffield");
    }

    #[test]
    fn test_compact_title_just_enough_space() {
        let title = "The University of Sheffield";
        assert_eq!(compact_title(title, 27), "The University of Sheffield");
    }

    #[test]
    fn test_compact_title_less_space() {
        let title = "The University of Sheffield";
        assert_eq!(compact_title(title, 20), "TU of Sheffield");
    }

    #[test]
    fn test_compact_title_fully_compact() {
        let title = "The University of Sheffield";
        assert_eq!(compact_title(title, 5), "TUoS");
    }

    #[test]
    fn test_compact_title_snapshot() {
        let title = "The University of Sheffield";
        let mut result = String::new();
        for i in (0..=30).rev() {
            result += &format!("{}: {}\n", i, compact_title(title, i));
        }
        assert_snapshot!(result);
    }

    #[test]
    fn test_compact_title_unicode_snapshot() {
        let title = "Thé Ünivęrsïty ôf ẞheƒƒiėld";
        let mut result = String::new();
        for i in (0..=30).rev() {
            result += &format!("{}: {}\n", i, compact_title(title, i));
        }
        assert_snapshot!(result);
    }

    #[test]
    fn test_compact_title_symbols_snapshot() {
        let title = "/The Invasion of the Tearling/ by Erika Johansen";
        let mut result = String::new();
        for i in (0..=50).rev() {
            result += &format!("{}: {}\n", i, compact_title(title, i));
        }
        assert_snapshot!(result);
    }

    #[test]
    fn test_grapheme_len() {
        let title = "Thé Ünivęrsïty ôf ẞheƒƒiėld";
        assert_eq!(grapheme_len(title), 27);
    }
}

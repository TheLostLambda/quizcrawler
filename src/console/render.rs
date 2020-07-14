use super::{data::*, util::*};
use crate::core::{
    data::Section,
    quiz::{QuizProgress, QuizRef},
};
use tui::{
    style::{Color, Modifier, StyleDiff},
    symbols::line,
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
// FIXME: Good lord, this file needs some cleaning...

impl Quizcrawler {
    pub fn render(&self, f: &mut Frame) {
        match self.state_stack.last() {
            Some(State::TreeView(s)) => tree_view(&self.tree, &s, f),
            Some(State::AskQuestion(s)) => question_view(&s.quiz, &s.progress, None, f),
            Some(State::AnswerQuestion(s, r)) => question_view(&s.quiz, &s.progress, Some(r), f),
            _ => {}
        }
    }
}

fn tree_view(section: &Section, state: &TreeState, f: &mut Frame) {
    let size = f.size();
    let node = section.child_at_path(&state.path).unwrap();
    let child_names: Vec<_> = node
        .children
        .iter()
        .map(|x| ListItem::new(vec![compact_title(&x.name, size.width as usize - 3).into()]))
        .collect();
    let selected_node = &node.children[state.get_selected()]; // FIXME: This panics
    let mut list_state = ListState::default();
    list_state.select(Some(state.get_selected()));
    let title = tree_titlebar(&section.name, &state.path, selected_node, size.width);
    let list = List::new(child_names)
        .block(titled_block(&title))
        .highlight_symbol(">");
    f.render_stateful_widget(list, size, &mut list_state);
}

fn question_view(
    quiz: &QuizRef,
    progress: &QuizProgress,
    result: Option<&(bool, String)>,
    f: &mut Frame,
) {
    let size = f.size();
    let title = progress_titlebar(progress, size.width);
    let mut text = print_context(&quiz);
    text.extend(print_question(&quiz));
    if let Some(&(correct, ref answer)) = result {
        text.extend(print_answer(correct, answer));
    } else {
        text.extend(print_choices(&quiz))
    }
    let list = Paragraph::new(vec![Spans::from(text)])
        .block(titled_block(&title))
        .wrap(Wrap { trim: false });
    f.render_widget(list, size);
}

fn print_context(quiz: &QuizRef) -> Vec<Span> {
    let style = StyleDiff::default().modifier(Modifier::ITALIC);
    let path = quiz.borrow().get_context().path.join(" > ");
    vec![Span::styled(format!("{}\n", path), style)]
}

fn print_question(quiz: &QuizRef) -> Vec<Span> {
    let style = StyleDiff::default().modifier(Modifier::BOLD);
    vec![Span::styled(format!("{}\n\n", quiz.borrow().ask()), style)]
}

fn print_choices(quiz: &QuizRef) -> Vec<Span> {
    quiz.borrow()
        .get_choices()
        .iter()
        .enumerate()
        .map(|(i, q)| Span::raw(format!("{}) {}\n", i + 1, q)))
        .collect()
}

fn print_answer(correct: bool, answer: &str) -> Vec<Span> {
    let answer_string = format!(", the answer is: {}", answer);
    let continue_string = "SPACE to continue";
    let correct_style = StyleDiff::default()
        .modifier(Modifier::BOLD)
        .fg(Color::Green);
    let wrong_style = correct_style.fg(Color::Red);
    if correct {
        vec![
            Span::styled(format!("Well done{}\n", answer_string), correct_style),
            Span::raw(format!("{}...", continue_string)),
        ]
    } else {
        vec![
            Span::styled(format!("Sorry{}\n", answer_string), wrong_style),
            Span::raw(format!("{}, 'o' for manual override...", continue_string)),
        ]
    }
}

fn tree_titlebar(root: &str, rest: &[String], selected: &Section, width: u16) -> String {
    let mut path = vec![root.to_owned()];
    path.extend(rest.to_vec());
    let children = selected.children.len();
    let questions = selected.questions.len();
    let info = format!(
        "{} Child{}, {} Question{}",
        children,
        if children == 1 { "" } else { "ren" },
        questions,
        if questions == 1 { "" } else { "s" }
    );
    // The 3 is from both corners plus the spacer between right and left
    let target_len = (width as usize)
        .checked_sub(grapheme_len(&info) + 3)
        .unwrap_or_default();
    let path = compact_path(&path[..], "/", target_len);
    render_titlebar(path, line::HORIZONTAL, info, width)
}

fn progress_titlebar(progress: &QuizProgress, width: u16) -> String {
    let learned = format!("Learned {} of {}", progress.learned, progress.questions);
    let score = progress
        .score
        .map_or(String::new(), |s| format!("Your score is {:.2}%", s));
    render_titlebar(learned, line::HORIZONTAL, score, width)
}

fn titled_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}

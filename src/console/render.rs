use super::{data::*, util::*};
use crate::core::{
    data::Section,
    // FIXME: Do I want this in this file? Maybe I need a type synonym file...
    quiz::{Progress, QuizRef},
};
use tui::{
    style::{Color, Modifier, Style},
    symbols::line,
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Text},
};
// FIXME: Good lord, this file needs some cleaning...

impl Quizcrawler {
    pub fn render(&self, f: &mut Frame) {
        match self.state_stack.last() {
            Some(State::TreeView(s)) => tree_view(&self.tree, &s, f),
            Some(State::AskQuestion(s)) => question_view(&s.quiz, &s.progress, None, f),
            Some(State::AnswerQuestion(s, r)) => {
                question_view(&s.quiz, &s.progress, Some(r.clone()), f) // FIXME: This result stuff is shit
            }
            _ => {}
        }
    }
}

fn tree_view(section: &Section, state: &TreeState, f: &mut Frame) {
    let size = f.size();
    let node = section.child_at_path(&state.path).unwrap();
    let child_names = node.children.iter().map(|x| Text::raw(&x.name));
    let selected_node = &node.children[state.get_selected()]; // FIXME: This panics
    let mut list_state = ListState::default();
    list_state.select(Some(state.get_selected()));
    let title = tree_titlebar(&section.name, &state.path, selected_node, size.width);
    let list = List::new(child_names)
        .block(titled_block(&title))
        .highlight_symbol(">");
    f.render_stateful_widget(list, size, &mut list_state);
}

// FIXME: Should that answer type be a struct?
fn question_view(
    quiz: &QuizRef,
    progress: &Progress,
    result: Option<(bool, String)>,
    f: &mut Frame,
) {
    let size = f.size();
    // FIXME: These messages need some refining
    let title = progress_titlebar(progress, size.width);
    let mut text = print_question(&quiz);
    if let Some((correct, ref answer)) = result {
        text.extend(print_answer(correct, answer));
    } else {
        text.extend(print_choices(&quiz))
    }
    let list = Paragraph::new(text.iter())
        .block(titled_block(&title))
        .wrap(true);
    f.render_widget(list, size);
}

fn print_question(quiz: &QuizRef) -> Vec<Text> {
    vec![Text::raw(format!("{}\n\n", quiz.borrow().ask()))]
}

fn print_choices(quiz: &QuizRef) -> Vec<Text> {
    quiz.borrow()
        .get_choices()
        .into_iter()
        .enumerate()
        .map(|(i, q)| Text::raw(format!("{}) {}\n", i + 1, q)))
        .collect()
}

fn print_answer(correct: bool, answer: &str) -> Vec<Text> {
    let answer_string = format!(", the answer is: {}", answer);
    let correct_style = Style::default().modifier(Modifier::BOLD).fg(Color::Green);
    let wrong_style = correct_style.fg(Color::Red);
    if correct {
        vec![Text::styled(
            format!("Well done{}", answer_string),
            correct_style,
        )]
    } else {
        vec![Text::styled(format!("Sorry{}", answer_string), wrong_style)]
    }
}

// FIXME: This should also shorten the path when it gets too long
fn tree_titlebar(root: &str, rest: &Vec<String>, selected: &Section, width: u16) -> String {
    let mut path = vec![root.to_owned()];
    path.extend(rest.clone());
    let path = path.join("/");
    // FIXME: This should remove the plural for 1 item (or just not use words)
    let info = format!(
        "{} Children, {} Questions",
        selected.children.len(),
        selected.questions.len()
    );
    render_titlebar(path, line::HORIZONTAL, info, width)
}

fn progress_titlebar(progress: &Progress, width: u16) -> String {
    let learned = format!("Learned {} of {}", progress.learned, progress.questions);
    let score = if progress.score >= 0.0 {
        format!("Your score is {:.2}%", progress.score)
    } else {
        String::new()
    };
    render_titlebar(learned, line::HORIZONTAL, score, width)
}

fn titled_block(title: &str) -> Block {
    Block::default()
        .title(&title)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
}

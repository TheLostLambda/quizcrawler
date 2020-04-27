use super::{data::*, util::*};
use crate::core::{
    data::Section,
    // FIXME: Do I want this in this file? Maybe I need a type synonym file...
    quiz::{Progress, QuizRef},
};
use tui::{
    symbols::line,
    widgets::{Block, BorderType, Borders, List, ListState, Paragraph, Text},
};
// FIXME: Good lord, this file needs some cleaning...

impl Quizcrawler {
    pub fn render(&self, f: &mut Frame) {
        match self.state_stack.last() {
            Some(State::TreeView(s)) => self.tree_view(&s, f),
            Some(State::AskQuestion(s)) => self.ask_question(&s.quiz, &s.progress, f),
            _ => {}
        }
    }

    fn tree_view(&self, state: &TreeState, f: &mut Frame) {
        let size = f.size();
        let node = &self.tree.child_at_path(&state.path).unwrap();
        let child_names = node.children.iter().map(|x| Text::raw(&x.name));
        let selected_node = &node.children[state.get_selected()]; // FIXME: This panics
        let mut list_state = ListState::default();
        list_state.select(Some(state.get_selected()));
        let title = render_title(&self.tree.name, &state.path, selected_node, size.width);
        let list = List::new(child_names)
            .block(
                Block::default()
                    .title(&title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .highlight_symbol(">");
        f.render_stateful_widget(list, size, &mut list_state);
    }

    fn ask_question(&self, quiz: &QuizRef, progress: &Progress, f: &mut Frame) {
        let size = f.size();
        // FIXME: These messages need some refining
        let title = render_titlebar(
            format!("{} questions to go!", progress.0),
            line::HORIZONTAL,
            if progress.1 > 0.0 {
                format!("Your score is {:.2}%", progress.1)
            } else {
                String::new()
            },
            size.width,
        );
        let mut text = vec![Text::raw(format!("{}\n\n", quiz.borrow().ask()))];
        let choices = quiz
            .borrow()
            .get_choices()
            .into_iter()
            .enumerate()
            .map(|(i, q)| Text::raw(format!("{}) {}\n", i + 1, q)));
        text.extend(choices);
        let list = Paragraph::new(text.iter())
            .block(
                Block::default()
                    .title(&title)
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded),
            )
            .wrap(true);
        f.render_widget(list, size);
    }
}

// FIXME: Not sure where this belongs... Should be TreeView specific
// This should also shorten the path when it gets too long
fn render_title(root: &str, rest: &Vec<String>, selected: &Section, width: u16) -> String {
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

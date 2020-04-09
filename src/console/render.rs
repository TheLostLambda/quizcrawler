use crate::console::data::*;
use crate::console::util::*;
use crate::core::data::Section;
use tui::{
    symbols::line,
    widgets::{Block, BorderType, Borders, List, ListState, Text},
};

// FIXME: Good lord, this file needs some cleaning...

impl Quizcrawler {
    pub fn render(&self, f: &mut Frame) {
        match self.state_stack.last() {
            Some(State::TreeView(state)) => self.tree_view(&state, f),
            _ => {}
        }
    }

    fn tree_view(&self, state: &TreeState, f: &mut Frame) {
        let size = f.size();
        let node = &self.tree.child_at_path(&state.path).unwrap();
        let child_names = node.children.iter().map(|x| Text::raw(&x.name));
        let selected_node = &node.children[state.get_selected()];
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
}

// FIXME: Not sure where this belongs...
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
    // The -2 comes from each corner taking up one char
    let padding = if width as usize > path.len() + info.len() + 2 {
        width as usize - path.len() - info.len() - 2
    } else {
        0
    };
    let spacer = line::HORIZONTAL.repeat(padding);
    [path, spacer, info].concat()
}

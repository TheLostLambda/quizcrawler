use crate::console::data::*;
use crate::console::util::*;
use crate::core::data::Section;
use tui::{
    symbols::line,
    widgets::{Block, Borders, SelectableList, Widget},
};

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
        let child_names: Vec<_> = node.children.iter().map(|x| &x.name[..]).collect();
        let selected_node = &node.children[state.get_selected()];
        SelectableList::default()
            .block(
                Block::default()
                    .title(&render_title(&self.tree.name, &state.path, selected_node, size.width))
                    .borders(Borders::ALL),
            )
            .items(&child_names)
            .select(Some(state.get_selected()))
            .highlight_symbol(">")
            .render(f, size);
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
    let spacer = line::HORIZONTAL.repeat(width as usize - path.len() - info.len() - 2);
    [path, spacer, info].concat()
}

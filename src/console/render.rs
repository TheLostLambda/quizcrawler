use crate::console::data::*;
use crate::console::util::*;
use crate::core::data::Section;
use tui::widgets::{Block, Borders, SelectableList, Widget};

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
        let children: Vec<_> = node.children.iter().map(|x| &x.name[..]).collect();
        SelectableList::default()
            .block(Block::default()
                   .title(&render_path(&self.tree.name, &state.path))
                   .borders(Borders::ALL))
            .items(&children)
            .select(Some(state.get_selected()))
            .highlight_symbol(">")
            .render(f, size);
    }
}

// FIXME: Not sure where this belongs...
fn render_path(root: &str, rest: &Vec<String>) -> String {
    let mut path = vec![root.to_owned()];
    path.extend(rest.clone());
    path.join("/")
}

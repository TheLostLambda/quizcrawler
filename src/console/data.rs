use crate::core::data::Section;
use crate::core::games::Game;
use crate::core::games::*;
use crossterm::event::KeyCode;
use std::collections::HashMap;

// Trim back things that don't need to be public

#[derive(Default)]
pub struct QCOptions {
    flipped: bool,
}

pub struct Quizcrawler {
    pub tree: Section,
    pub options: QCOptions,
    pub state_stack: Vec<State>,
    pub games: Vec<Box<dyn Game>>, // FIXME: Does this actually serve any purpose?
}

#[derive(Default)]
pub struct TreeState {
    pub path: Vec<String>,
    pub selection_history: HashMap<Vec<String>, usize>,
}

impl TreeState {
    pub fn get_selected_mut(&mut self) -> &mut usize {
        self.selection_history
            .entry(self.path.clone())
            .or_insert(0)
    }
    
    pub fn get_selected(&self) -> usize {
        *self.selection_history.get(&self.path).unwrap_or(&0)
    }
}

pub enum State {
    TreeView(TreeState),
}

impl Quizcrawler {
    pub fn new(options: QCOptions, tree: Section) -> Self {
        Self {
            tree,
            options,
            games: Vec::new(),
            state_stack: vec![State::TreeView(Default::default())],
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        match self.state_stack.last_mut() {
            Some(State::TreeView(state)) => {
                // FIXME: Maybe move this to a function?
                let node = self.tree.child_at_path(&state.path).unwrap(); // FIXME: Unwrap
                let child_names: Vec<_> = node.children.iter().map(|x| &x.name).collect();
                let limit = child_names.len() - 1;
                // This gives me an entry, a mutable reference which is updated in the match
                let current = state.get_selected();
                let selected = state.get_selected_mut();
                match key {
                    KeyCode::Up if current > 0 =>
                        *selected -= 1,
                    KeyCode::Down if current < limit =>
                        *selected += 1,
                    KeyCode::Right if node.children[current].is_parent() =>
                        state.path.push(child_names[current].to_owned()),
                    KeyCode::Left => { state.path.pop(); }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}

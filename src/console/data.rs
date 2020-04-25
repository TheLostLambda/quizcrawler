use crate::core::data::Section;
use crate::core::quiz::{MultipleChoice, Progress, QuizDispatcher, QuizRef};
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
}

#[derive(Default)]
pub struct TreeState {
    pub path: Vec<String>,
    pub selection_history: HashMap<Vec<String>, usize>,
}

impl TreeState {
    pub fn get_selected_mut(&mut self) -> &mut usize {
        self.selection_history.entry(self.path.clone()).or_insert(0)
    }

    pub fn get_selected(&self) -> usize {
        *self.selection_history.get(&self.path).unwrap_or(&0)
    }
}

// FIXME: This could use some more thought
pub enum State {
    TreeView(TreeState),
    Dispatch(QuizDispatcher),
    AskQuestion(QuizRef, Progress),
}

impl Quizcrawler {
    pub fn new(options: QCOptions, tree: Section) -> Self {
        Self {
            tree,
            options,
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
                let selector = state.get_selected_mut();
                match key {
                    KeyCode::Up if current > 0 => *selector -= 1,
                    KeyCode::Down if current < limit => *selector += 1,
                    KeyCode::Right if node.children[current].is_parent() => {
                        state.path.push(child_names[current].to_owned())
                    }
                    // FIXME: This feels a tad muddled
                    KeyCode::Right => {
                        let mut path = state.path.clone();
                        path.push(child_names[current].to_owned());
                        // FIXME: unwrap is likely a bad idea here
                        let questions = &self.tree.child_at_path(&path).unwrap().questions;
                        let mut dispatcher = QuizDispatcher::new(questions.to_vec());
                        dispatcher.register_quiz(MultipleChoice::default());
                        self.state_stack.push(State::Dispatch(dispatcher))
                    }
                    KeyCode::Left => {
                        state.path.pop();
                    }
                    _ => {}
                }
            }
            // FIXME: Move this to a non-input loop
            Some(State::Dispatch(dispatcher)) => {
                match dispatcher.next() {
                    Some(quiz) => {
                        let progress = dispatcher.progress();
                        self.state_stack.push(State::AskQuestion(quiz, progress));
                    }
                    None => {
                        self.state_stack.pop();
                    }
                }
            }
            _ => {}
        }
    }
}

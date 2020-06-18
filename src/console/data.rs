use crate::core::{
    data::Section,
    quiz::{MultipleChoice, QuizDispatcher, QuizProgress, QuizRef},
};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;

// Trim back things that don't need to be public

#[derive(Default)]
pub struct QCSettings {}

pub struct Quizcrawler {
    pub tree: Section,
    pub settings: QCSettings,
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

#[derive(Clone)]
pub struct QuestionState {
    pub quiz: QuizRef,
    pub progress: QuizProgress,
}

// FIXME: This could use some more thought
pub enum State {
    TreeView(TreeState),
    Dispatch(QuizDispatcher),
    AskQuestion(QuestionState),
    AnswerQuestion(QuestionState, (bool, String)), // FIXME: Ew
}

impl Quizcrawler {
    pub fn new(settings: QCSettings, tree: Section) -> Self {
        Self {
            tree,
            settings,
            state_stack: vec![State::TreeView(TreeState::default())],
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.state_stack.last_mut() {
            Some(State::TreeView(state)) => {
                // FIXME: Maybe move this to a function?
                let node = self.tree.child_at_path(&state.path).unwrap(); // FIXME: Unwrap
                let child_names: Vec<_> = node.children.iter().map(|x| &x.name).collect();
                let limit = child_names.len() - 1;
                let current = state.get_selected();
                // This gives me an entry, a mutable reference which is updated in the match
                let selector = state.get_selected_mut();
                match key.code {
                    KeyCode::Char('q') => {
                        self.state_stack.pop();
                    }
                    KeyCode::Up if current > 0 => *selector -= 1,
                    KeyCode::Down if current < limit => *selector += 1,
                    KeyCode::Right if node.children[current].is_parent() => {
                        state.path.push(child_names[current].to_owned())
                    }
                    // FIXME: This feels a tad muddled
                    KeyCode::Char(' ') => {
                        let mut path = state.path.clone();
                        path.push(child_names[current].to_owned());
                        // FIXME: unwrap is likely a bad idea here
                        let questions = &self
                            .tree
                            .child_at_path(&path)
                            .unwrap()
                            .get_questions(key.modifiers.contains(KeyModifiers::CONTROL));
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
            Some(State::AskQuestion(state)) => {
                if !state.quiz.borrow().get_choices().is_empty() {
                    match key.code {
                        KeyCode::Char('q') => {
                            // FIXME: This double-pop is hacky and is needed to get past the dispatcher
                            // Write a rewind function to pop the stack back until some condition is met,
                            // like reaching something isn't Dispatch. Maybe the states could have a
                            // transient flag and all of those are popped off.
                            self.state_stack.pop();
                            self.state_stack.pop();
                        }
                        KeyCode::Char('h') => state.quiz.borrow_mut().get_hint(),
                        KeyCode::Char(c) => {
                            let result = state.quiz.borrow_mut().answer(&c.to_string());
                            if let Some(result) = result {
                                let state = state.clone();
                                self.state_stack.pop();
                                self.state_stack.push(State::AnswerQuestion(state, result))
                            }
                        }
                        _ => {}
                    }
                }
            }
            Some(State::AnswerQuestion(state, (correct, _))) => match key.code {
                KeyCode::Char('q') => {
                    // FIXME: Hacky double-pop
                    self.state_stack.pop();
                    self.state_stack.pop();
                }
                KeyCode::Char('o') if !*correct => {
                    state.quiz.borrow_mut().i_was_right();
                    *correct = true;
                }
                KeyCode::Char(' ') => {
                    self.state_stack.pop();
                }
                _ => {}
            },
            _ => {}
        }
    }

    pub fn tick(&mut self) -> bool {
        match self.state_stack.last_mut() {
            Some(State::Dispatch(dispatcher)) => {
                if let Some(quiz) = dispatcher.next() {
                    let progress = dispatcher.progress();
                    let state = QuestionState { quiz, progress };
                    self.state_stack.push(State::AskQuestion(state));
                } else {
                    self.state_stack.pop();
                }
                true
            }
            Some(_) => true,
            None => false,
        }
    }
}

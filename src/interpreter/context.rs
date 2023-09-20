use crate::ast::Ast;
use crate::interpreter::state::State;
use std::collections::HashMap;

/// A context for the interpreter. This can hold more than what just a
/// State can -- it also holds definitions, which are programs that are
/// bound to identifiers.
pub struct Context {
    pub state: State,
    pub definitions: HashMap<String, Ast>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            state: State::new(),
            definitions: HashMap::new(),
        }
    }

    pub fn add_definition(&mut self, name: String, definition: Ast) {
        self.definitions.insert(name, definition);
    }

    pub fn get_definition(&self, name: &str) -> Option<&Ast> {
        self.definitions.get(name)
    }

    pub fn set_variable(&mut self, name: String, value: i32) {
        self.state.set(name, value);
    }

    pub fn get_variable(&self, name: &str) -> i32 {
        self.state.get(name)
    }
}

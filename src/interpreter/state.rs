use std::collections::HashMap;

/// A representation of the mathematical notion of state in While. It's a wrapper over a HashMap.
///
/// Provides a Display impl and total equality checking.
///
/// If a value doesn't exist, 0 is returned.
///
/// Explicit settings of values to 0 show up in the end Display, but gets on their own, don't.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    mappings: HashMap<String, i32>,
}

impl State {
    pub fn new() -> Self {
        Self {
            mappings: HashMap::new(),
        }
    }

    /// Retrieve a value from the State.
    ///
    /// If the key doesn't exist, 0 is returned.
    pub fn get(&self, ident: &str) -> i32 {
        match self.mappings.get(ident) {
            None => 0,
            Some(&x) => x,
        }
    }

    /// Performs an assignment for a given ident.
    pub fn set(&mut self, ident: String, val: i32) {
        self.mappings.insert(ident, val);
    }
}

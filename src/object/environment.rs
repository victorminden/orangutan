//! Environment
//! 
//! `environment` contains a simple struct representing the environment of the Monkey interpreter.
use std::collections::HashMap;
use crate::object::Object;

/// Represents the environment of objects already recognized by the interpreter.
/// 
/// Such objects are known about due to the interpretation of prior statements.
#[derive(Default, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get(&self, name: &str) -> Option<&Object> {
        self.store.get(name)
    }

    pub fn set(&mut self, name: &str, val: Object) {
        self.store.insert(name.to_string(), val);
    }
}

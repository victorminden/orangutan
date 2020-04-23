use std::collections::HashMap;
use std::mem;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolScope {
    Global,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Symbol {
    pub scope: SymbolScope,
    pub index: u16,
}

#[derive(Debug)]
pub enum SymbolError {
    NotFound,
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    num_definitions: u16,
}

impl SymbolTable {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn define(&mut self, name: &String) -> &Symbol {
        self.store.insert(name.clone(), Symbol {
            scope: SymbolScope::Global,
            index: self.num_definitions,
        });
        self.num_definitions += 1;
        &self.store[name]
    }

    pub fn resolve(&self, name: &String) -> Result<Symbol, SymbolError> {
        match self.store.get(name) {
            Some(value) => Ok(*value),
            None => Err(SymbolError::NotFound),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_test() {
        let expected = vec![
            Symbol { scope: SymbolScope::Global, index: 0 },
            Symbol { scope: SymbolScope::Global, index: 1 },
        ];
        let mut global = SymbolTable::new();
        let a = global.define(&String::from("a"));
        assert_eq!(a, &expected[0]);
        let b = global.define(&String::from("b"));
        assert_eq!(b, &expected[1]);
    }

    #[test]
    fn resolve_global_test() {
        let expected = vec![
            Symbol { scope: SymbolScope::Global, index: 0 },
            Symbol { scope: SymbolScope::Global, index: 1 },
        ];
        let mut global = SymbolTable::new();
        global.define(&String::from("a"));
        let a_hat = global.resolve(&String::from("a")).unwrap();
        assert_eq!(expected[0], a_hat);
        global.define(&String::from("b"));
        let b_hat = global.resolve(&String::from("b")).unwrap();
        assert_eq!(expected[1], b_hat);
    }
}
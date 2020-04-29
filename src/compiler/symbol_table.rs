use crate::object::BuiltIn;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolScope {
    Global,
    Local,
    BuiltIn,
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
struct SymbolStore {
    store: HashMap<String, Symbol>,
    pub num_definitions: u16,
}

impl SymbolStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn define_with_scope(
        &mut self,
        name: &String,
        scope: SymbolScope,
        index: Option<u16>,
    ) -> &Symbol {
        let idx = match index {
            Some(idx) => idx,
            None => {
                self.num_definitions += 1;
                self.num_definitions - 1
            }
        };

        self.store
            .insert(name.clone(), Symbol { scope, index: idx });
        &self.store[name]
    }

    pub fn resolve(&self, name: &String) -> Result<Symbol, SymbolError> {
        match self.store.get(name) {
            Some(value) => Ok(*value),
            None => Err(SymbolError::NotFound),
        }
    }
}

#[derive(Default, Debug)]
pub struct SymbolTable {
    stores: Vec<SymbolStore>,
    store_index: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            stores: vec![SymbolStore::new()],
            store_index: 1,
        }
    }

    pub fn new_with_builtins() -> Self {
        let mut sym_table = SymbolTable::new();
        for b in BuiltIn::all() {
            let idx: u8 = b.clone().into();
            sym_table.define_builtin(&b.name(), idx as u16);
        }
        sym_table
    }

    fn define_builtin(&mut self, name: &String, index: u16) -> &Symbol {
        self.stores[0].define_with_scope(name, SymbolScope::BuiltIn, Some(index))
    }

    pub fn num_definitions(&self) -> usize {
        self.stores[self.store_index - 1].num_definitions as usize
    }

    pub fn enter_scope(&mut self) {
        self.stores.push(SymbolStore::new());
        self.store_index += 1;
    }

    pub fn leave_scope(&mut self) {
        self.stores.pop();
        self.store_index -= 1;
    }

    pub fn define(&mut self, name: &String) -> &Symbol {
        let scope = if self.store_index > 1 {
            SymbolScope::Local
        } else {
            SymbolScope::Global
        };
        self.stores[self.store_index - 1].define_with_scope(name, scope, None)
    }

    pub fn resolve(&self, name: &String) -> Result<Symbol, SymbolError> {
        self.resolve_with_index(name, self.store_index - 1)
    }

    fn resolve_with_index(&self, name: &String, index: usize) -> Result<Symbol, SymbolError> {
        match self.stores[index].resolve(name) {
            Err(error) => {
                if index > 0 {
                    // Recursively look in the outer scopes.
                    self.resolve_with_index(name, index - 1)
                } else {
                    Err(error)
                }
            }
            good_result => good_result,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn define_test() {
        let expected = vec![
            Symbol {
                scope: SymbolScope::Global,
                index: 0,
            },
            Symbol {
                scope: SymbolScope::Global,
                index: 1,
            },
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
            Symbol {
                scope: SymbolScope::Global,
                index: 0,
            },
            Symbol {
                scope: SymbolScope::Global,
                index: 1,
            },
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

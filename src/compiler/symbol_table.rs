use crate::object::BuiltIn;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SymbolScope {
    Global,
    Local,
    BuiltIn,
    Free,
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
    pub free_symbols: Vec<Symbol>,
}

impl SymbolStore {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn define_free(&mut self, name: &String, original: &Symbol) -> &Symbol {
        self.free_symbols.push(original.clone());
        let symbol = Symbol {
            scope: SymbolScope::Free,
            index: (self.free_symbols.len() - 1) as u16,
        };
        self.store.insert(name.clone(), symbol);
        self.store.get(name).unwrap()
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

    pub fn resolve(&mut self, name: &String) -> Result<Symbol, SymbolError> {
        let current_index = self.store_index - 1;
        match self.resolve_with_index(name, current_index) {
            Ok((sym, index)) => {
                if index == current_index || sym.scope != SymbolScope::Local {
                    return Ok(sym);
                }
                // Define the symbol as free in the current scope.
                // TODO: May need to do this for all scopes between current scope and found scope.
                return Ok(self.stores[index as usize].define_free(name, &sym).clone());
            }
            Err(error) => Err(error),
        }
    }

    fn resolve_with_index(
        &self,
        name: &String,
        index: usize,
    ) -> Result<(Symbol, usize), SymbolError> {
        match self.stores[index].resolve(name) {
            Err(error) => {
                if index > 0 {
                    // Recursively look in the outer scopes.
                    self.resolve_with_index(name, index - 1)
                } else {
                    Err(error)
                }
            }
            Ok(good_result) => Ok((good_result, index)),
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

    #[test]
    fn resolve_free_test() {
        let mut tbl = SymbolTable::new();
        tbl.define(&String::from("a"));
        tbl.define(&String::from("b"));
        tbl.enter_scope();
        tbl.define(&String::from("c"));
        tbl.define(&String::from("d"));

        let mut test = tbl.resolve(&String::from("a")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Global,
                index: 0,
            }
        );
        test = tbl.resolve(&String::from("b")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Global,
                index: 1,
            }
        );

        test = tbl.resolve(&String::from("c")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Local,
                index: 0,
            }
        );

        test = tbl.resolve(&String::from("d")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Local,
                index: 1,
            }
        );

        tbl.enter_scope();
        tbl.define(&String::from("e"));
        tbl.define(&String::from("f"));

        test = tbl.resolve(&String::from("a")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Global,
                index: 0,
            }
        );
        test = tbl.resolve(&String::from("b")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Global,
                index: 1,
            }
        );
        test = tbl.resolve(&String::from("c")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Free,
                index: 0,
            }
        );
        test = tbl.resolve(&String::from("d")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Free,
                index: 1,
            }
        );
        test = tbl.resolve(&String::from("e")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Local,
                index: 0,
            }
        );
        test = tbl.resolve(&String::from("f")).unwrap();
        assert_eq!(
            test,
            Symbol {
                scope: SymbolScope::Local,
                index: 1,
            }
        );
        let out = tbl.resolve(&String::from("does_not_exist"));
        assert!(out.is_err());
    }
}

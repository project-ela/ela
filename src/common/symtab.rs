use std::{cell::RefCell, collections::HashMap};

use crate::frontend::ast::Parameter;

use super::types::Type;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);
impl NodeId {
    pub fn new() -> Self {
        thread_local! {
            static CURRENT_ID: RefCell<usize> = RefCell::new(0);
        }

        CURRENT_ID.with(|c| {
            *c.borrow_mut() += 1;
            Self(*c.borrow())
        })
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable {
    pub scopes: HashMap<NodeId, SymbolScope>,
}

#[derive(Debug, Default)]
pub struct SymbolScope {
    pub variables: HashMap<String, SigVar>,
    pub functions: HashMap<String, SigFunc>,
    pub parent_node: Option<NodeId>,
}

#[derive(Debug, Clone)]
// typ, is_const
pub struct SigVar(pub Type, pub bool);

#[derive(Debug, Clone)]
// params, ret_typ
pub struct SigFunc(pub Vec<Parameter>, pub Type);

impl SymbolScope {
    fn new(parent_node: Option<NodeId>) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            parent_node,
        }
    }
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            scopes: HashMap::new(),
        }
    }

    pub fn add_scope(&mut self, node: NodeId, parent_node: Option<NodeId>) {
        self.scopes.insert(node, SymbolScope::new(parent_node));
    }

    pub fn add_variable(&mut self, node: NodeId, name: String, sig: SigVar) {
        let scope = self.scopes.get_mut(&node).unwrap();
        scope.variables.insert(name, sig);
    }

    pub fn find_variable(&mut self, node: NodeId, name: &String) -> Option<SigVar> {
        let mut cur_scope = self.scopes.get(&node).unwrap();
        loop {
            if let Some(sig) = cur_scope.variables.get(name) {
                return Some(sig.clone());
            }

            match cur_scope.parent_node {
                Some(parent_node) => cur_scope = self.scopes.get(&parent_node).unwrap(),
                None => break,
            }
        }

        None
    }

    pub fn is_defined_here(&mut self, node: NodeId, name: &String) -> bool {
        let scope = self.scopes.get(&node).unwrap();
        scope.variables.contains_key(name)
    }

    pub fn add_function(&mut self, node: NodeId, name: String, sig: SigFunc) {
        let scope = self.scopes.get_mut(&node).unwrap();
        scope.functions.insert(name, sig);
    }

    pub fn find_function(&mut self, node: NodeId, name: &String) -> Option<SigFunc> {
        let mut cur_scope = self.scopes.get(&node).unwrap();
        loop {
            if let Some(sig) = cur_scope.functions.get(name) {
                return Some(sig.clone());
            }

            match cur_scope.parent_node {
                Some(parent_node) => cur_scope = self.scopes.get(&parent_node).unwrap(),
                None => break,
            }
        }

        None
    }
}

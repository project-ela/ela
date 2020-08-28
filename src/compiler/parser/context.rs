use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    pub functions: HashMap<String, FunctionSig>,
    pub variables: HashMap<String, Variable>,
    pub cur_offset: u32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
            cur_offset: 0,
        }
    }

    pub fn add_function(&mut self, name: &String) {
        self.functions
            .insert(name.clone(), FunctionSig { name: name.clone() });
    }

    pub fn find_function(&self, name: &String) -> Option<&FunctionSig> {
        self.functions.get(name)
    }

    pub fn add_variable(&mut self, name: &String) {
        self.cur_offset += 4; // TODO
        self.variables.insert(
            name.clone(),
            Variable {
                name: name.clone(),
                offset: self.cur_offset,
            },
        );
    }

    pub fn find_variable(&self, name: &String) -> Option<&Variable> {
        self.variables.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub offset: u32,
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    pub functions: HashMap<String, FunctionSig>,
    pub variables: HashMap<String, Variable>,
    pub types: HashMap<String, Type>,
    pub cur_offset: u32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
            types: HashMap::new(),
            cur_offset: 0,
        }
    }

    pub fn add_function(&mut self, name: &String, ret_type: &Type) {
        self.functions.insert(
            name.clone(),
            FunctionSig {
                name: name.clone(),
                ret_type: ret_type.clone(),
            },
        );
    }

    pub fn find_function(&self, name: &String) -> Option<&FunctionSig> {
        self.functions.get(name)
    }

    pub fn add_variable(&mut self, name: &String, typ: &Type) {
        self.cur_offset += 4; // TODO
        self.variables.insert(
            name.clone(),
            Variable {
                name: name.clone(),
                typ: typ.clone(),
                offset: self.cur_offset,
            },
        );
    }

    pub fn find_variable(&self, name: &String) -> Option<&Variable> {
        self.variables.get(name)
    }

    pub fn add_type(&mut self, name: &String, typ: Type) {
        self.types.insert(name.clone(), typ);
    }

    pub fn find_type(&self, name: &String) -> Option<&Type> {
        self.types.get(name)
    }
}

#[derive(Debug, Clone)]
pub struct FunctionSig {
    pub name: String,
    pub ret_type: Type,
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub typ: Type,
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Bool,
}

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Context {
    pub variables: HashMap<String, Variable>,
    pub cur_offset: u32,
}

impl Context {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            cur_offset: 0,
        }
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
pub struct Variable {
    pub name: String,
    pub offset: u32,
}

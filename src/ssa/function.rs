use super::Instruction;

#[derive(Debug)]
pub struct Function {
    pub name: String,

    pub instructions: Vec<Instruction>,
}

impl Function {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            instructions: Vec::new(),
        }
    }

    pub fn add_instruction(&mut self, instruction: Instruction) {
        self.instructions.push(instruction)
    }
}

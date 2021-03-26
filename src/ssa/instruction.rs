use super::Value;

#[derive(Debug, Clone)]
pub enum Instruction {
    Add(Box<Value>, Box<Value>),
}

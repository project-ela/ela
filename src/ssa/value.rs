use super::Instruction;

#[derive(Debug, Clone)]
pub enum Value {
    Immediate(u32),
    Instruction(Instruction),
}

use super::InstructionId;

#[derive(Debug, Clone, Copy)]
pub enum Value {
    Immediate(u32),
    Instruction(InstructionId),
}

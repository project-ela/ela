pub mod immediate;
pub mod memory;
pub mod offset;
pub mod register;

use immediate::Immediate;
use memory::Memory;
use offset::Offset;
use register::Register;

#[derive(Debug)]
pub enum Operand {
    Immediate(Immediate),
    Register(Register),
    Memory(Memory),
    Offset(Offset),
}

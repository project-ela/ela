use x86asm::instruction::operand::register;

#[derive(Debug)]
pub enum Value {
    Value8(u8),
    Value32(u32),
    Value64(u64),
}

impl Value {
    pub fn size(&self) -> register::Size {
        match self {
            Value::Value8(_) => register::Size::Byte,
            Value::Value32(_) => register::Size::DWord,
            Value::Value64(_) => register::Size::QWord,
        }
    }

    pub fn as_u64(&self) -> u64 {
        match self {
            Value::Value8(value) => *value as u64,
            Value::Value32(value) => *value as u64,
            Value::Value64(value) => *value,
        }
    }
}

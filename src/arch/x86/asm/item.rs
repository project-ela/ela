use super::Instruction;

#[derive(Debug)]
pub enum AssemblyItem {
    Instruction(Instruction),
    PseudoOp(PseudoOp),
    Label(String),
}

#[derive(Debug)]
pub enum PseudoOp {
    Global(String),
}

impl AssemblyItem {
    pub fn stringify(&self) -> String {
        use self::AssemblyItem::*;

        match self {
            Instruction(inst) => inst.stringify(),
            PseudoOp(op) => op.stringify(),
            Label(name) => format!("{}:", name),
        }
    }
}

impl PseudoOp {
    pub fn stringify(&self) -> String {
        use self::PseudoOp::*;

        match self {
            Global(name) => format!(".global {}", name),
        }
    }
}

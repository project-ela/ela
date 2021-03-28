use super::{AssemblyItem, Instruction, PseudoOp};

#[derive(Debug)]
pub struct Assembly {
    pub items: Vec<AssemblyItem>,
}

impl Assembly {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn add_inst(&mut self, inst: Instruction) {
        self.items.push(AssemblyItem::Instruction(inst));
    }

    pub fn add_pseudo_op(&mut self, op: PseudoOp) {
        self.items.push(AssemblyItem::PseudoOp(op));
    }

    pub fn add_label<S: Into<String>>(&mut self, name: S) {
        self.items.push(AssemblyItem::Label(name.into()));
    }

    pub fn stringify(&self) -> String {
        let mut s = String::new();

        s.push_str(".intel_syntax noprefix\n");

        for item in &self.items {
            s.push_str(&item.stringify());
            s.push_str("\n");
        }

        s
    }
}

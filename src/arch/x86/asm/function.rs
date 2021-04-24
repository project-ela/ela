use super::{AssemblyItem, Instruction, PseudoOp};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub items: Vec<AssemblyItem>,
}

impl Function {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
        }
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

        s.push_str(&format!(".global {}\n", self.name));
        s.push_str(&format!("{}:\n", self.name));
        for item in &self.items {
            s.push_str(&item.stringify());
            s.push('\n');
        }

        s
    }
}

use std::fmt::Write;

use super::{AssemblyItem, Instruction, Printer};

#[derive(Debug)]
pub struct Block {
    pub name: String,
    pub items: Vec<AssemblyItem>,
}

impl Block {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            items: Vec::new(),
        }
    }

    pub fn add_inst(&mut self, inst: Instruction) {
        self.items.push(AssemblyItem::Instruction(inst));
    }

    pub fn add_label<S: Into<String>>(&mut self, name: S) {
        self.items.push(AssemblyItem::Label(name.into()));
    }
}

impl Printer for Block {
    fn print(&self, buf: &mut String) -> super::Result {
        writeln!(buf, "{}:", self.name)?;
        for item in &self.items {
            item.print(buf)?;
        }
        Ok(())
    }
}

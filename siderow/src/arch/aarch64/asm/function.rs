use std::fmt::Write;

use super::{AssemblyItem, Instruction, Printer, Result};

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

    pub fn add_label<S: Into<String>>(&mut self, name: S) {
        self.items.push(AssemblyItem::Label(name.into()));
    }
}

impl Printer for Function {
    fn print(&self, buf: &mut String) -> Result {
        writeln!(buf, ".global {}", self.name)?;
        writeln!(buf, "{}:", self.name)?;
        for item in &self.items {
            item.print(buf)?;
        }
        Ok(())
    }
}

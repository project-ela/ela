use std::fmt::Write;

use super::{AssemblyItem, Printer};

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

    pub fn add_label<S: Into<String>>(&mut self, name: S) {
        self.items.push(AssemblyItem::Label(name.into()));
    }
}

impl Printer for Function {
    fn print(&self, buf: &mut String) {
        writeln!(buf, ".global {}", self.name);
        writeln!(buf, "{}:", self.name);
        for item in &self.items {
            item.print(buf);
        }
    }
}

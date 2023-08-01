use std::fmt::Write;

use super::Printer;

#[derive(Debug)]
pub struct Function {
    pub name: String,
}

impl Function {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self { name: name.into() }
    }
}

impl Printer for Function {
    fn print(&self, buf: &mut String) {
        writeln!(buf, ".global {}", self.name);
        writeln!(buf, "{}:", self.name);
    }
}

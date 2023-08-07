use std::fmt::Write;

use super::{Function, Printer, Result};

#[derive(Debug)]
pub struct Assembly {
    pub text: TextSection,
}

impl Assembly {
    pub fn new() -> Self {
        Self {
            text: TextSection::new(),
        }
    }
}

impl Printer for Assembly {
    fn print(&self, buf: &mut String) -> Result {
        self.text.print(buf)
    }
}

#[derive(Debug)]
pub struct TextSection {
    pub functions: Vec<Function>,
}

impl TextSection {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.functions.push(function);
    }
}

impl Printer for TextSection {
    fn print(&self, buf: &mut String) -> Result {
        writeln!(buf, ".text")?;
        for func in &self.functions {
            func.print(buf)?;
            writeln!(buf)?;
        }
        Ok(())
    }
}

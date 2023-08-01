use std::fmt::Write;

use super::{Function, Printer};

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
    fn print(&self, buf: &mut String) {
        writeln!(buf, ".intel_syntax noprefix");
        self.text.print(buf);
    }
}

#[derive(Debug)]
pub struct TextSection {
    pub function: Vec<Function>,
}

impl TextSection {
    pub fn new() -> Self {
        Self {
            function: Vec::new(),
        }
    }

    pub fn add_function(&mut self, function: Function) {
        self.function.push(function);
    }
}

impl Printer for TextSection {
    fn print(&self, buf: &mut String) {
        writeln!(buf, ".text");
        for func in &self.function {
            func.print(buf);
            writeln!(buf);
        }
    }
}

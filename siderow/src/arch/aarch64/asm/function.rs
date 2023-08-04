use std::fmt::Write;

use super::{Block, Printer, Result};

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub blocks: Vec<Block>,
}

impl Function {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            blocks: Vec::new(),
        }
    }

    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
}

impl Printer for Function {
    fn print(&self, buf: &mut String) -> Result {
        writeln!(buf, ".global {}", self.name)?;
        writeln!(buf, "{}:", self.name)?;
        for block in &self.blocks {
            block.print(buf)?;
        }
        Ok(())
    }
}

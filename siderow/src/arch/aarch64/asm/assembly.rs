use std::fmt::Write;

#[derive(Debug)]
pub struct Assembly {}

impl Assembly {
    pub fn new() -> Self {
        Self {}
    }
}

impl Printer for Assembly {
    fn print(&self, buf: &mut String) {
        writeln!(buf, ".intel_syntax noprefix");
        self.text.print(buf);
    }
}

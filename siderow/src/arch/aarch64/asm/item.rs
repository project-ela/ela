use std::fmt::Write;

use super::Printer;

#[derive(Debug)]
pub enum AssemblyItem {
    Label(String),
}

impl Printer for AssemblyItem {
    fn print(&self, buf: &mut String) {
        use self::AssemblyItem::*;

        match self {
            Label(name) => {
                writeln!(buf, "{}:", name);
            }
        }
    }
}

pub type Result = std::fmt::Result;

pub trait Printer {
    fn print(&self, buf: &mut String) -> Result;
}

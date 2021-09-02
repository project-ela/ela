use std::fmt;

#[derive(Debug, Clone, Default)]
pub struct Pos {
    pub filename: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.filename, self.line, self.column)
    }
}

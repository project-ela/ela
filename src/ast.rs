#[derive(Debug)]
pub enum AST {
    Integer { value: u32 },
    Add { lhs: Box<AST>, rhs: Box<AST> },
    Sub { lhs: Box<AST>, rhs: Box<AST> },
}

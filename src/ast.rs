#[derive(Debug)]
pub enum AST {
    Function { name: String, body: Box<AST> },
    Return { value: Box<AST> },
    Integer { value: u32 },
    Add { lhs: Box<AST>, rhs: Box<AST> },
    Sub { lhs: Box<AST>, rhs: Box<AST> },
    Mul { lhs: Box<AST>, rhs: Box<AST> },
    Div { lhs: Box<AST>, rhs: Box<AST> },
}

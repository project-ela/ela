use crate::ast::AST;
use crate::token::Token;
pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

pub fn parse(tokens: Vec<Token>) -> Result<AST, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<AST, String> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<AST, String> {
        match self.consume() {
            Token::IntLiteral { value } => Ok(AST::Integer { value: *value }),
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn consume(&mut self) -> &Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        token
    }
}

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
        self.parse_add()
    }

    fn parse_add(&mut self) -> Result<AST, String> {
        let mut node = self.parse_mul()?;
        loop {
            match self.peek() {
                Token::Plus => {
                    self.consume();
                    node = AST::Add {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_mul()?),
                    }
                }
                Token::Minus => {
                    self.consume();
                    node = AST::Sub {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_mul()?),
                    }
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_mul(&mut self) -> Result<AST, String> {
        let mut node = self.parse_primary()?;
        loop {
            match self.peek() {
                Token::Asterisk => {
                    self.consume();
                    node = AST::Mul {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_primary()?),
                    }
                }
                Token::Slash => {
                    self.consume();
                    node = AST::Div {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_primary()?),
                    }
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<AST, String> {
        match self.consume() {
            Token::IntLiteral { value } => Ok(AST::Integer { value: *value }),
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn consume(&mut self) -> &Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        token
    }
}

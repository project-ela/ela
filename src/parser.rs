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
        self.parse_function()
    }

    fn parse_function(&mut self) -> Result<AST, String> {
        self.expect(&Token::Func)?;
        let name = self.consume_ident()?;
        self.expect(&Token::LParen)?;
        self.expect(&Token::RParen)?;
        let body = self.parse_statement()?;
        Ok(AST::Function {
            name,
            body: Box::new(body),
        })
    }

    fn parse_statement(&mut self) -> Result<AST, String> {
        match self.consume() {
            Token::LBrace => {
                let mut stmts = Vec::new();
                loop {
                    if self.peek() == &Token::RBrace {
                        self.consume();
                        break;
                    }
                    stmts.push(self.parse_statement()?);
                }
                Ok(AST::Block { stmts })
            }
            Token::Return => {
                let value = self.parse_expression()?;
                Ok(AST::Return {
                    value: Box::new(value),
                })
            }
            Token::If => {
                let cond = self.parse_expression()?;
                let then = self.parse_statement()?;
                let els = if self.peek() == &Token::Else {
                    self.consume();
                    let els = self.parse_statement()?;
                    Some(Box::new(els))
                } else {
                    None
                };
                Ok(AST::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    els,
                })
            }
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn parse_expression(&mut self) -> Result<AST, String> {
        self.parse_equal()
    }

    fn parse_equal(&mut self) -> Result<AST, String> {
        let mut node = self.parse_relation()?;
        loop {
            match self.peek() {
                Token::Equal => {
                    self.consume();
                    node = AST::Equal {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_relation()?),
                    }
                }
                Token::NotEqual => {
                    self.consume();
                    node = AST::NotEqual {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_relation()?),
                    }
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_relation(&mut self) -> Result<AST, String> {
        let mut node = self.parse_add()?;
        loop {
            match self.peek() {
                Token::Lt => {
                    self.consume();
                    node = AST::Lt {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_add()?),
                    }
                }
                Token::Lte => {
                    self.consume();
                    node = AST::Lte {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_add()?),
                    }
                }
                Token::Gt => {
                    self.consume();
                    node = AST::Gt {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_add()?),
                    }
                }
                Token::Gte => {
                    self.consume();
                    node = AST::Gte {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_add()?),
                    }
                }
                _ => break,
            }
        }

        Ok(node)
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
        let mut node = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Asterisk => {
                    self.consume();
                    node = AST::Mul {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_unary()?),
                    }
                }
                Token::Slash => {
                    self.consume();
                    node = AST::Div {
                        lhs: Box::new(node),
                        rhs: Box::new(self.parse_unary()?),
                    }
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<AST, String> {
        match self.peek() {
            Token::Plus => {
                self.consume();
                Ok(AST::Add {
                    lhs: Box::new(AST::Integer { value: 0 }),
                    rhs: Box::new(self.parse_unary()?),
                })
            }
            Token::Minus => {
                self.consume();
                Ok(AST::Sub {
                    lhs: Box::new(AST::Integer { value: 0 }),
                    rhs: Box::new(self.parse_unary()?),
                })
            }
            _ => Ok(self.parse_primary()?),
        }
    }

    fn parse_primary(&mut self) -> Result<AST, String> {
        match self.consume() {
            Token::IntLiteral { value } => Ok(AST::Integer { value: *value }),
            Token::LParen => {
                let expr = self.parse_add()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn expect(&mut self, token: &Token) -> Result<&Token, String> {
        let next_token = self.consume();
        if next_token == token {
            Ok(next_token)
        } else {
            Err(format!("expected {:?}, but got {:?}", token, next_token))
        }
    }

    fn peek(&mut self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn consume_ident(&mut self) -> Result<String, String> {
        let next_token = self.consume();
        if let Token::Ident { name } = next_token {
            Ok(name.to_string())
        } else {
            Err(format!("expected identifier, but got {:?}", next_token))
        }
    }

    fn consume(&mut self) -> &Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        token
    }
}

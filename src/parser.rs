use crate::ast::*;
use crate::token::Token;
pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

macro_rules! new_binop {
    ($self: expr, $op: expr, $lhs: expr, $rhs: expr) => {{
        $self.consume();
        AstExpression::BinaryOp {
            op: $op,
            lhs: Box::new($lhs),
            rhs: Box::new($rhs),
        }
    }};
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<Program, String> {
        let mut program = Program::new();
        while !self.is_eof() {
            program.functions.push(self.parse_function()?);
        }
        Ok(program)
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(Token::Func)?;
        let name = self.consume_ident()?;
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        let body = self.parse_statement()?;
        Ok(Function { name, body })
    }

    fn parse_statement(&mut self) -> Result<AstStatement, String> {
        match self.consume() {
            Token::LBrace => {
                let mut stmts = Vec::new();
                loop {
                    if self.peek() == Token::RBrace {
                        self.consume();
                        break;
                    }
                    stmts.push(self.parse_statement()?);
                }
                Ok(AstStatement::Block { stmts })
            }
            Token::Return => {
                let value = self.parse_expression()?;
                Ok(AstStatement::Return {
                    value: Box::new(value),
                })
            }
            Token::If => {
                let cond = self.parse_expression()?;
                let then = self.parse_statement()?;
                let els = if self.peek() == Token::Else {
                    self.consume();
                    let els = self.parse_statement()?;
                    Some(Box::new(els))
                } else {
                    None
                };
                Ok(AstStatement::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    els,
                })
            }
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn parse_expression(&mut self) -> Result<AstExpression, String> {
        self.parse_equal()
    }

    fn parse_equal(&mut self) -> Result<AstExpression, String> {
        let mut node = self.parse_relation()?;
        loop {
            match self.peek() {
                Token::Equal => {
                    node = new_binop!(self, Operator::Equal, node, self.parse_relation()?)
                }
                Token::NotEqual => {
                    node = new_binop!(self, Operator::NotEqual, node, self.parse_relation()?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_relation(&mut self) -> Result<AstExpression, String> {
        let mut node = self.parse_add()?;
        loop {
            match self.peek() {
                Token::Lt => node = new_binop!(self, Operator::Lt, node, self.parse_add()?),
                Token::Lte => node = new_binop!(self, Operator::Lte, node, self.parse_add()?),
                Token::Gt => node = new_binop!(self, Operator::Gt, node, self.parse_add()?),
                Token::Gte => node = new_binop!(self, Operator::Gte, node, self.parse_add()?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_add(&mut self) -> Result<AstExpression, String> {
        let mut node = self.parse_mul()?;
        loop {
            match self.peek() {
                Token::Plus => node = new_binop!(self, Operator::Add, node, self.parse_mul()?),
                Token::Minus => node = new_binop!(self, Operator::Sub, node, self.parse_mul()?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_mul(&mut self) -> Result<AstExpression, String> {
        let mut node = self.parse_unary()?;
        loop {
            match self.peek() {
                Token::Asterisk => {
                    node = new_binop!(self, Operator::Mul, node, self.parse_unary()?)
                }
                Token::Slash => node = new_binop!(self, Operator::Div, node, self.parse_unary()?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<AstExpression, String> {
        match self.peek() {
            Token::Plus => Ok(new_binop!(
                self,
                Operator::Add,
                AstExpression::Integer { value: 0 },
                self.parse_unary()?
            )),
            Token::Minus => Ok(new_binop!(
                self,
                Operator::Sub,
                AstExpression::Integer { value: 0 },
                self.parse_unary()?
            )),
            _ => Ok(self.parse_primary()?),
        }
    }

    fn parse_primary(&mut self) -> Result<AstExpression, String> {
        match self.consume() {
            Token::IntLiteral { value } => Ok(AstExpression::Integer { value: value }),
            Token::LParen => {
                let expr = self.parse_add()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn expect(&mut self, token: Token) -> Result<Token, String> {
        let next_token = self.consume();
        if next_token == token {
            Ok(next_token)
        } else {
            Err(format!("expected {:?}, but got {:?}", token, next_token))
        }
    }

    fn is_eof(&self) -> bool {
        self.peek() == Token::EOF
    }

    fn peek(&self) -> Token {
        if self.pos >= self.tokens.len() {
            Token::EOF
        } else {
            self.tokens.get(self.pos).unwrap().clone()
        }
    }

    fn consume_ident(&mut self) -> Result<String, String> {
        let next_token = self.consume();
        if let Token::Ident { name } = next_token {
            Ok(name.to_string())
        } else {
            Err(format!("expected identifier, but got {:?}", next_token))
        }
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        token.clone()
    }
}

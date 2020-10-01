pub mod ast;

use crate::{
    common::{
        error::{Error, ErrorKind},
        operator::{BinaryOperator, UnaryOperator},
        types::Type,
    },
    frontend::{
        lexer::token::TokenKind,
        parser::ast::{AstExpression, AstStatement, Function, Program},
    },
};

struct Parser {
    pos: usize,
    tokens: Vec<TokenKind>,
}

pub fn parse(tokens: Vec<TokenKind>) -> Result<Program, Error> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

macro_rules! new_unop {
    ($self: expr, $op: expr, $expr: expr) => {{
        $self.consume();
        AstExpression::UnaryOp {
            op: $op,
            expr: Box::new($expr),
        }
    }};
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
    fn new(tokens: Vec<TokenKind>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<Program, Error> {
        let mut program = Program::default();
        while !self.is_eof() {
            program.functions.push(self.parse_function()?);
        }
        Ok(program)
    }

    fn parse_function(&mut self) -> Result<Function, Error> {
        self.expect(TokenKind::Func)?;
        let name = self.consume_ident()?;
        self.expect(TokenKind::LParen)?;
        self.expect(TokenKind::RParen)?;
        let ret_typ = match self.peek() {
            TokenKind::Colon => {
                self.consume();
                self.consume_type()?
            }
            _ => Type::Void,
        };
        let body = self.parse_statement()?;
        Ok(Function {
            name,
            ret_typ,
            body,
        })
    }

    fn parse_statement(&mut self) -> Result<AstStatement, Error> {
        match self.consume() {
            TokenKind::LBrace => {
                let mut stmts = Vec::new();
                while self.peek() != TokenKind::RBrace {
                    stmts.push(self.parse_statement()?);
                }
                self.consume();
                Ok(AstStatement::Block { stmts })
            }
            token @ TokenKind::Var | token @ TokenKind::Val => {
                let name = self.consume_ident()?;
                self.expect(TokenKind::Colon)?;
                let typ = self.consume_type()?;
                self.expect(TokenKind::Assign)?;
                let value = self.parse_expression()?;
                match token {
                    TokenKind::Var => Ok(AstStatement::Var {
                        name,
                        typ,
                        value: Box::new(value),
                    }),
                    TokenKind::Val => Ok(AstStatement::Val {
                        name,
                        typ,
                        value: Box::new(value),
                    }),
                    _ => unreachable!(),
                }
            }
            TokenKind::Ident { name } => match self.peek() {
                TokenKind::Assign => {
                    self.consume();
                    let value = self.parse_expression()?;
                    Ok(AstStatement::Assign {
                        name,
                        value: Box::new(value),
                    })
                }
                TokenKind::LParen => {
                    self.consume();
                    self.expect(TokenKind::RParen)?;
                    Ok(AstStatement::Call { name })
                }
                x => Err(Error::new(ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x,
                })),
            },
            TokenKind::Return => Ok(AstStatement::Return {
                value: match self.parse_expression() {
                    Ok(expr) => Some(Box::new(expr)),
                    Err(_) => {
                        self.pos -= 1;
                        None
                    }
                },
            }),
            TokenKind::If => {
                let cond = self.parse_expression()?;
                let then = self.parse_statement()?;
                let els = match self.peek() {
                    TokenKind::Else => {
                        self.consume();
                        let els = self.parse_statement()?;
                        Some(Box::new(els))
                    }
                    _ => None,
                };
                Ok(AstStatement::If {
                    cond: Box::new(cond),
                    then: Box::new(then),
                    els,
                })
            }
            TokenKind::While => {
                let cond = self.parse_expression()?;
                let body = self.parse_statement()?;
                Ok(AstStatement::While {
                    cond: Box::new(cond),
                    body: Box::new(body),
                })
            }
            x => Err(Error::new(ErrorKind::UnexpectedToken {
                expected: None,
                actual: x,
            })),
        }
    }

    fn parse_expression(&mut self) -> Result<AstExpression, Error> {
        self.parse_bitor()
    }

    fn parse_bitor(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_bitxor()?;
        while let TokenKind::Or = self.peek() {
            node = new_binop!(self, BinaryOperator::Or, node, self.parse_bitxor()?)
        }

        Ok(node)
    }

    fn parse_bitxor(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_bitand()?;
        while let TokenKind::Xor = self.peek() {
            node = new_binop!(self, BinaryOperator::Xor, node, self.parse_bitand()?)
        }

        Ok(node)
    }

    fn parse_bitand(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_equal()?;
        while let TokenKind::And = self.peek() {
            node = new_binop!(self, BinaryOperator::And, node, self.parse_equal()?)
        }

        Ok(node)
    }

    fn parse_equal(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_relation()?;
        loop {
            match self.peek() {
                TokenKind::Equal => {
                    node = new_binop!(self, BinaryOperator::Equal, node, self.parse_relation()?)
                }
                TokenKind::NotEqual => {
                    node = new_binop!(self, BinaryOperator::NotEqual, node, self.parse_relation()?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_relation(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_add()?;
        loop {
            match self.peek() {
                TokenKind::Lt => {
                    node = new_binop!(self, BinaryOperator::Lt, node, self.parse_add()?)
                }
                TokenKind::Lte => {
                    node = new_binop!(self, BinaryOperator::Lte, node, self.parse_add()?)
                }
                TokenKind::Gt => {
                    node = new_binop!(self, BinaryOperator::Gt, node, self.parse_add()?)
                }
                TokenKind::Gte => {
                    node = new_binop!(self, BinaryOperator::Gte, node, self.parse_add()?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_add(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_mul()?;
        loop {
            match self.peek() {
                TokenKind::Plus => {
                    node = new_binop!(self, BinaryOperator::Add, node, self.parse_mul()?)
                }
                TokenKind::Minus => {
                    node = new_binop!(self, BinaryOperator::Sub, node, self.parse_mul()?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_mul(&mut self) -> Result<AstExpression, Error> {
        let mut node = self.parse_unary()?;
        loop {
            match self.peek() {
                TokenKind::Asterisk => {
                    node = new_binop!(self, BinaryOperator::Mul, node, self.parse_unary()?)
                }
                TokenKind::Slash => {
                    node = new_binop!(self, BinaryOperator::Div, node, self.parse_unary()?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<AstExpression, Error> {
        match self.peek() {
            TokenKind::Plus => Ok(new_binop!(
                self,
                BinaryOperator::Add,
                AstExpression::Integer { value: 0 },
                self.parse_unary()?
            )),
            TokenKind::Minus => Ok(new_binop!(
                self,
                BinaryOperator::Sub,
                AstExpression::Integer { value: 0 },
                self.parse_unary()?
            )),
            TokenKind::Not => Ok(new_unop!(self, UnaryOperator::Not, self.parse_unary()?)),
            _ => Ok(self.parse_primary()?),
        }
    }

    fn parse_primary(&mut self) -> Result<AstExpression, Error> {
        match self.consume() {
            TokenKind::IntLiteral { value } => Ok(AstExpression::Integer { value }),
            TokenKind::False => Ok(AstExpression::Bool { value: false }),
            TokenKind::True => Ok(AstExpression::Bool { value: true }),
            TokenKind::Ident { name } => match self.peek() {
                TokenKind::LParen => {
                    self.consume();
                    self.expect(TokenKind::RParen)?;
                    Ok(AstExpression::Call { name })
                }
                _ => Ok(AstExpression::Ident { name }),
            },
            TokenKind::LParen => {
                let expr = self.parse_add()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            x => Err(Error::new(ErrorKind::UnexpectedToken {
                expected: None,
                actual: x,
            })),
        }
    }

    fn expect(&mut self, token: TokenKind) -> Result<TokenKind, Error> {
        let next_token = self.consume();
        if next_token == token {
            Ok(next_token)
        } else {
            Err(Error::new(ErrorKind::UnexpectedToken {
                expected: Some(token),
                actual: next_token,
            }))
        }
    }

    fn is_eof(&self) -> bool {
        self.peek() == TokenKind::EOF
    }

    fn peek(&self) -> TokenKind {
        if self.pos >= self.tokens.len() {
            TokenKind::EOF
        } else {
            self.tokens.get(self.pos).unwrap().clone()
        }
    }

    fn consume_ident(&mut self) -> Result<String, Error> {
        let next_token = self.consume();
        if let TokenKind::Ident { name } = next_token {
            Ok(name)
        } else {
            Err(Error::new(ErrorKind::ExpectedIdent { actual: next_token }))
        }
    }

    fn consume_type(&mut self) -> Result<Type, Error> {
        let typ_name = self.consume_ident()?;
        match typ_name.as_str() {
            "int" => Ok(Type::Int),
            "bool" => Ok(Type::Bool),
            x => Err(Error::new(ErrorKind::NotTypeName { name: x.into() })),
        }
    }

    fn consume(&mut self) -> TokenKind {
        let token = match self.tokens.get(self.pos) {
            // skip comment
            Some(TokenKind::Comment { .. }) => {
                self.pos += 1;
                return self.consume();
            }
            Some(token) => token,
            None => &TokenKind::EOF,
        };
        self.pos += 1;
        token.clone()
    }
}

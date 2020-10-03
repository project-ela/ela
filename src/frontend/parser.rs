pub mod ast;

use crate::{
    common::{
        error::{Error, ErrorKind},
        operator::{BinaryOperator, UnaryOperator},
        types::Type,
    },
    frontend::{
        lexer::token::{Token, TokenKind},
        parser::ast::{Expression, ExpressionKind, Function, Program, Statement, StatementKind},
    },
};

struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

macro_rules! new_unop {
    ($self: expr, $op: expr, $expr: expr) => {{
        let token = $self.consume();
        Expression::new(
            ExpressionKind::UnaryOp {
                op: $op,
                expr: Box::new($expr),
            },
            token.pos,
        )
    }};
}

macro_rules! new_binop {
    ($self: expr, $op: expr, $lhs: expr, $rhs: expr) => {{
        let token = $self.consume();
        Expression::new(
            ExpressionKind::BinaryOp {
                op: $op,
                lhs: Box::new($lhs),
                rhs: Box::new($rhs),
            },
            token.pos,
        )
    }};
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
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
        let pos = self.expect(TokenKind::Func)?.pos;
        let name = self.consume_ident()?;
        self.expect(TokenKind::LParen)?;
        self.expect(TokenKind::RParen)?;
        let ret_typ = match self.peek().kind {
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
            pos,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        let token = self.consume();
        match token.kind {
            TokenKind::LBrace => {
                let mut stmts = Vec::new();
                while self.peek().kind != TokenKind::RBrace {
                    stmts.push(self.parse_statement()?);
                }
                self.consume();
                Ok(Statement::new(StatementKind::Block { stmts }, token.pos))
            }
            kind @ TokenKind::Var | kind @ TokenKind::Val => {
                let name = self.consume_ident()?;
                self.expect(TokenKind::Colon)?;
                let typ = self.consume_type()?;
                self.expect(TokenKind::Assign)?;
                let value = self.parse_expression()?;
                match kind {
                    TokenKind::Var => Ok(Statement::new(
                        StatementKind::Var {
                            name,
                            typ,
                            value: Box::new(value),
                        },
                        token.pos,
                    )),
                    TokenKind::Val => Ok(Statement::new(
                        StatementKind::Val {
                            name,
                            typ,
                            value: Box::new(value),
                        },
                        token.pos,
                    )),
                    _ => unreachable!(),
                }
            }
            TokenKind::Ident { name } => match self.peek().kind {
                TokenKind::Assign => {
                    self.consume();
                    let value = self.parse_expression()?;
                    Ok(Statement::new(
                        StatementKind::Assign {
                            name,
                            value: Box::new(value),
                        },
                        token.pos,
                    ))
                }
                TokenKind::LParen => {
                    self.consume();
                    self.expect(TokenKind::RParen)?;
                    Ok(Statement::new(StatementKind::Call { name }, token.pos))
                }
                x => Err(Error::new(
                    token.pos,
                    ErrorKind::UnexpectedToken {
                        expected: None,
                        actual: x,
                    },
                )),
            },
            TokenKind::Return => Ok(Statement::new(
                StatementKind::Return {
                    value: match self.parse_expression() {
                        Ok(expr) => Some(Box::new(expr)),
                        Err(_) => {
                            self.pos -= 1;
                            None
                        }
                    },
                },
                token.pos,
            )),
            TokenKind::If => {
                let cond = self.parse_expression()?;
                let then = self.parse_statement()?;
                let els = match self.peek().kind {
                    TokenKind::Else => {
                        self.consume();
                        let els = self.parse_statement()?;
                        Some(Box::new(els))
                    }
                    _ => None,
                };
                Ok(Statement::new(
                    StatementKind::If {
                        cond: Box::new(cond),
                        then: Box::new(then),
                        els,
                    },
                    token.pos,
                ))
            }
            TokenKind::While => {
                let cond = self.parse_expression()?;
                let body = self.parse_statement()?;
                Ok(Statement::new(
                    StatementKind::While {
                        cond: Box::new(cond),
                        body: Box::new(body),
                    },
                    token.pos,
                ))
            }
            x => Err(Error::new(
                token.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x,
                },
            )),
        }
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_bitor()
    }

    fn parse_bitor(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_bitxor()?;
        while let TokenKind::Or = self.peek().kind {
            node = new_binop!(self, BinaryOperator::Or, node, self.parse_bitxor()?)
        }

        Ok(node)
    }

    fn parse_bitxor(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_bitand()?;
        while let TokenKind::Xor = self.peek().kind {
            node = new_binop!(self, BinaryOperator::Xor, node, self.parse_bitand()?)
        }

        Ok(node)
    }

    fn parse_bitand(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_equal()?;
        while let TokenKind::And = self.peek().kind {
            node = new_binop!(self, BinaryOperator::And, node, self.parse_equal()?)
        }

        Ok(node)
    }

    fn parse_equal(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_relation()?;
        loop {
            match self.peek().kind {
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

    fn parse_relation(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_add()?;
        loop {
            match self.peek().kind {
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

    fn parse_add(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_mul()?;
        loop {
            match self.peek().kind {
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

    fn parse_mul(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_unary()?;
        loop {
            match self.peek().kind {
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

    fn parse_unary(&mut self) -> Result<Expression, Error> {
        let token = self.peek();
        match token.kind {
            TokenKind::Plus => Ok(new_binop!(
                self,
                BinaryOperator::Add,
                Expression::new(ExpressionKind::Integer { value: 0 }, token.pos),
                self.parse_unary()?
            )),
            TokenKind::Minus => Ok(new_binop!(
                self,
                BinaryOperator::Sub,
                Expression::new(ExpressionKind::Integer { value: 0 }, token.pos),
                self.parse_unary()?
            )),
            TokenKind::Not => Ok(new_unop!(self, UnaryOperator::Not, self.parse_unary()?)),
            _ => Ok(self.parse_primary()?),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, Error> {
        let token = self.consume();
        match token.kind {
            TokenKind::IntLiteral { value } => Ok(Expression::new(
                ExpressionKind::Integer { value },
                token.pos,
            )),
            TokenKind::False => Ok(Expression::new(
                ExpressionKind::Bool { value: false },
                token.pos,
            )),
            TokenKind::True => Ok(Expression::new(
                ExpressionKind::Bool { value: true },
                token.pos,
            )),
            TokenKind::Ident { name } => match self.peek().kind {
                TokenKind::LParen => {
                    self.consume();
                    self.expect(TokenKind::RParen)?;
                    Ok(Expression::new(ExpressionKind::Call { name }, token.pos))
                }
                _ => Ok(Expression::new(ExpressionKind::Ident { name }, token.pos)),
            },
            TokenKind::LParen => {
                let expr = self.parse_add()?;
                self.expect(TokenKind::RParen)?;
                Ok(expr)
            }
            x => Err(Error::new(
                token.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x,
                },
            )),
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, Error> {
        let next_token = self.consume();
        if next_token.kind == kind {
            Ok(next_token)
        } else {
            Err(Error::new(
                next_token.pos,
                ErrorKind::UnexpectedToken {
                    expected: Some(kind),
                    actual: next_token.kind,
                },
            ))
        }
    }

    fn is_eof(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).unwrap().clone()
    }

    fn consume_ident(&mut self) -> Result<String, Error> {
        let next_token = self.consume();
        if let TokenKind::Ident { name } = next_token.kind {
            Ok(name)
        } else {
            Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedIdent {
                    actual: next_token.kind,
                },
            ))
        }
    }

    fn consume_type(&mut self) -> Result<Type, Error> {
        let next_token = self.consume();
        if let TokenKind::Ident { name } = next_token.kind {
            match name.as_str() {
                "int" => Ok(Type::Int),
                "bool" => Ok(Type::Bool),
                x => Err(Error::new(
                    next_token.pos,
                    ErrorKind::NotTypeName { name: x.into() },
                )),
            }
        } else {
            Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedIdent {
                    actual: next_token.kind,
                },
            ))
        }
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens.get(self.pos).unwrap();
        if let TokenKind::Comment { .. } = token.kind {
            self.pos += 1;
            return self.consume();
        }

        if self.pos < self.tokens.len() {
            self.pos += 1;
        }

        token.clone()
    }
}

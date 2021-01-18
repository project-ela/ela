pub mod ast;

use crate::{
    common::{
        error::{Error, ErrorKind},
        operator::{BinaryOperator, UnaryOperator},
        pos::Pos,
        types::Type,
    },
    frontend::{
        lexer::token::{Token, TokenKind},
        parser::ast::{
            Expression, ExpressionKind, Function, GlobalDef, Parameter, Program, Statement,
            StatementKind,
        },
    },
};

use super::lexer::token::{Keyword, Symbol};

struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

type FuncCall = (String, Vec<Expression>);

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

macro_rules! op_assign {
    ($self: expr, $op: expr, $dst: expr) => {{
        let pos = $dst.pos.clone();
        $self.consume();
        let value = $self.parse_expression()?;
        Ok(Statement::new(
            StatementKind::Assign {
                dst: Box::new($dst.clone()),
                value: Box::new(Expression::new(
                    ExpressionKind::BinaryOp {
                        op: $op,
                        lhs: Box::new($dst),
                        rhs: Box::new(value),
                    },
                    pos.clone(),
                )),
            },
            pos,
        ))
    }};
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<Program, Error> {
        let mut program = Program::default();
        while !self.is_eof() {
            self.parse_toplevel(&mut program)?;
        }
        Ok(program)
    }

    fn parse_toplevel(&mut self, program: &mut Program) -> Result<(), Error> {
        let token = self.peek();
        match token.kind {
            TokenKind::Keyword(Keyword::Func) => program.functions.push(self.parse_function()?),
            TokenKind::Keyword(Keyword::Var) => {
                self.consume();
                if let StatementKind::Var { name, typ, .. } =
                    self.parse_var_statement(token.pos)?.kind
                {
                    program.global_defs.push(GlobalDef {
                        name,
                        typ,
                        is_const: false,
                    })
                }
            }
            TokenKind::Keyword(Keyword::Val) => {
                self.consume();
                if let StatementKind::Val { name, typ, .. } =
                    self.parse_val_statement(token.pos)?.kind
                {
                    program.global_defs.push(GlobalDef {
                        name,
                        typ,
                        is_const: true,
                    })
                }
            }
            x => {
                return Err(Error::new(
                    token.pos,
                    ErrorKind::UnexpectedToken {
                        expected: None,
                        actual: x,
                    },
                ))
            }
        }

        Ok(())
    }

    fn parse_function(&mut self) -> Result<Function, Error> {
        let pos = self.expect(TokenKind::Keyword(Keyword::Func))?.pos;
        let name = self.consume_ident()?;
        self.expect(TokenKind::Symbol(Symbol::LParen))?;
        let params = self.parse_function_parameters()?;
        self.expect(TokenKind::Symbol(Symbol::RParen))?;
        let ret_typ = match self.peek().kind {
            TokenKind::Symbol(Symbol::Colon) => {
                self.consume();
                self.consume_type()?
            }
            _ => Type::Void,
        };
        let mut body = None;
        if self.peek().kind == TokenKind::Symbol(Symbol::LBrace) {
            body = Some(self.parse_statement()?);
        }
        Ok(Function {
            name,
            params,
            ret_typ,
            body,
            pos,
        })
    }

    fn parse_function_parameters(&mut self) -> Result<Vec<Parameter>, Error> {
        let mut params = Vec::new();
        if self.peek().kind != TokenKind::Symbol(Symbol::RParen) {
            params.push(self.parse_function_parameter()?);
        }
        while self.peek().kind != TokenKind::Symbol(Symbol::RParen) {
            self.expect(TokenKind::Symbol(Symbol::Comma))?;
            params.push(self.parse_function_parameter()?);
        }
        Ok(params)
    }

    fn parse_function_parameter(&mut self) -> Result<Parameter, Error> {
        let param_name = self.consume_ident()?;
        self.expect(TokenKind::Symbol(Symbol::Colon))?;
        let param_typ = self.consume_type()?;

        Ok(Parameter {
            name: param_name,
            typ: param_typ,
        })
    }

    fn parse_statement(&mut self) -> Result<Statement, Error> {
        let token = self.consume();
        match token.kind {
            TokenKind::Symbol(Symbol::LBrace) => self.parse_block_statement(token.pos),
            TokenKind::Keyword(Keyword::Var) => self.parse_var_statement(token.pos),
            TokenKind::Keyword(Keyword::Val) => self.parse_val_statement(token.pos),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_statement(token.pos),
            TokenKind::Keyword(Keyword::If) => self.parse_if_statement(token.pos),
            TokenKind::Keyword(Keyword::While) => self.parse_while_statement(token.pos),
            TokenKind::Comment(_) => self.parse_statement(),
            _ => {
                self.pos -= 1;
                let expr = self.parse_unary()?;
                if let ExpressionKind::Call { name, args } = expr.kind {
                    return Ok(Statement::new(StatementKind::Call { name, args }, expr.pos));
                }
                match self.peek().kind {
                    TokenKind::Symbol(Symbol::Assign) => {
                        self.parse_assign_statement(expr, token.pos)
                    }
                    TokenKind::Symbol(Symbol::PlusAssign) => {
                        op_assign!(self, BinaryOperator::Add, expr)
                    }
                    TokenKind::Symbol(Symbol::MinusAssign) => {
                        op_assign!(self, BinaryOperator::Sub, expr)
                    }
                    TokenKind::Symbol(Symbol::AsteriskAssign) => {
                        op_assign!(self, BinaryOperator::Mul, expr)
                    }
                    TokenKind::Symbol(Symbol::SlashAssign) => {
                        op_assign!(self, BinaryOperator::Div, expr)
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
        }
    }

    fn parse_block_statement(&mut self, pos: Pos) -> Result<Statement, Error> {
        let mut stmts = Vec::new();
        while self.peek().kind != TokenKind::Symbol(Symbol::RBrace) {
            stmts.push(self.parse_statement()?);
        }
        self.consume();
        Ok(Statement::new(StatementKind::Block { stmts }, pos))
    }

    fn parse_var_statement(&mut self, pos: Pos) -> Result<Statement, Error> {
        let name = self.consume_ident()?;
        self.expect(TokenKind::Symbol(Symbol::Colon))?;
        let typ = self.consume_type()?;
        let mut value = None;
        if self.peek().kind == TokenKind::Symbol(Symbol::Assign) {
            self.consume();
            value = Some(Box::new(self.parse_expression()?));
        }
        Ok(Statement::new(StatementKind::Var { name, typ, value }, pos))
    }

    fn parse_val_statement(&mut self, pos: Pos) -> Result<Statement, Error> {
        let name = self.consume_ident()?;
        self.expect(TokenKind::Symbol(Symbol::Colon))?;
        let typ = self.consume_type()?;
        let mut value = None;
        if self.peek().kind == TokenKind::Symbol(Symbol::Assign) {
            self.consume();
            value = Some(Box::new(self.parse_expression()?));
        }
        Ok(Statement::new(StatementKind::Val { name, typ, value }, pos))
    }

    fn parse_assign_statement(&mut self, dst: Expression, pos: Pos) -> Result<Statement, Error> {
        self.expect(TokenKind::Symbol(Symbol::Assign))?;
        let value = self.parse_expression()?;
        Ok(Statement::new(
            StatementKind::Assign {
                dst: Box::new(dst),
                value: Box::new(value),
            },
            pos,
        ))
    }

    fn parse_return_statement(&mut self, pos: Pos) -> Result<Statement, Error> {
        Ok(Statement::new(
            StatementKind::Return {
                value: match self.parse_expression() {
                    Ok(expr) => Some(Box::new(expr)),
                    Err(_) => {
                        self.pos -= 1;
                        None
                    }
                },
            },
            pos,
        ))
    }

    fn parse_if_statement(&mut self, pos: Pos) -> Result<Statement, Error> {
        let cond = self.parse_expression()?;
        let then = self.parse_statement()?;
        let els = match self.peek().kind {
            TokenKind::Keyword(Keyword::Else) => {
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
            pos,
        ))
    }

    fn parse_while_statement(&mut self, pos: Pos) -> Result<Statement, Error> {
        let cond = self.parse_expression()?;
        let body = self.parse_statement()?;
        Ok(Statement::new(
            StatementKind::While {
                cond: Box::new(cond),
                body: Box::new(body),
            },
            pos,
        ))
    }

    fn parse_expression(&mut self) -> Result<Expression, Error> {
        self.parse_bitor()
    }

    fn parse_bitor(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_bitxor()?;
        while let TokenKind::Symbol(Symbol::Or) = self.peek().kind {
            node = new_binop!(self, BinaryOperator::Or, node, self.parse_bitxor()?)
        }

        Ok(node)
    }

    fn parse_bitxor(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_bitand()?;
        while let TokenKind::Symbol(Symbol::Xor) = self.peek().kind {
            node = new_binop!(self, BinaryOperator::Xor, node, self.parse_bitand()?)
        }

        Ok(node)
    }

    fn parse_bitand(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_equal()?;
        while let TokenKind::Symbol(Symbol::And) = self.peek().kind {
            node = new_binop!(self, BinaryOperator::And, node, self.parse_equal()?)
        }

        Ok(node)
    }

    fn parse_equal(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_relation()?;
        loop {
            match self.peek().kind {
                TokenKind::Symbol(Symbol::Equal) => {
                    node = new_binop!(self, BinaryOperator::Equal, node, self.parse_relation()?)
                }
                TokenKind::Symbol(Symbol::NotEqual) => {
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
                TokenKind::Symbol(Symbol::Lt) => {
                    node = new_binop!(self, BinaryOperator::Lt, node, self.parse_add()?)
                }
                TokenKind::Symbol(Symbol::Lte) => {
                    node = new_binop!(self, BinaryOperator::Lte, node, self.parse_add()?)
                }
                TokenKind::Symbol(Symbol::Gt) => {
                    node = new_binop!(self, BinaryOperator::Gt, node, self.parse_add()?)
                }
                TokenKind::Symbol(Symbol::Gte) => {
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
                TokenKind::Symbol(Symbol::Plus) => {
                    node = new_binop!(self, BinaryOperator::Add, node, self.parse_mul()?)
                }
                TokenKind::Symbol(Symbol::Minus) => {
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
                TokenKind::Symbol(Symbol::Asterisk) => {
                    // TODO
                    if let ExpressionKind::UnaryOp {
                        op: UnaryOperator::Addr,
                        ..
                    } = node.kind
                    {
                        break;
                    }
                    node = new_binop!(self, BinaryOperator::Mul, node, self.parse_unary()?)
                }
                TokenKind::Symbol(Symbol::Slash) => {
                    node = new_binop!(self, BinaryOperator::Div, node, self.parse_unary()?)
                }
                TokenKind::Symbol(Symbol::Percent) => {
                    node = new_binop!(self, BinaryOperator::Mod, node, self.parse_unary()?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self) -> Result<Expression, Error> {
        let token = self.peek();
        match token.kind {
            TokenKind::Symbol(Symbol::Plus) => Ok(new_binop!(
                self,
                BinaryOperator::Add,
                Expression::new(ExpressionKind::Integer { value: 0 }, token.pos),
                self.parse_unary()?
            )),
            TokenKind::Symbol(Symbol::Minus) => Ok(new_binop!(
                self,
                BinaryOperator::Sub,
                Expression::new(ExpressionKind::Integer { value: 0 }, token.pos),
                self.parse_unary()?
            )),
            TokenKind::Symbol(Symbol::Not) => {
                Ok(new_unop!(self, UnaryOperator::Not, self.parse_unary()?))
            }
            TokenKind::Symbol(Symbol::And) => {
                Ok(new_unop!(self, UnaryOperator::Addr, self.parse_unary()?))
            }
            TokenKind::Symbol(Symbol::Asterisk) => {
                Ok(new_unop!(self, UnaryOperator::Load, self.parse_unary()?))
            }
            _ => Ok(self.parse_postfix()?),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expression, Error> {
        let mut node = self.parse_primary()?;

        if self.peek().kind == TokenKind::Symbol(Symbol::LBracket) {
            let pos = self.consume().pos;
            node = Expression::new(
                ExpressionKind::Index {
                    lhs: Box::new(node),
                    index: Box::new(self.parse_expression()?),
                },
                pos,
            );
            self.expect(TokenKind::Symbol(Symbol::RBracket))?;
        }

        Ok(node)
    }

    fn parse_primary(&mut self) -> Result<Expression, Error> {
        let token = self.consume();
        let kind = match token.kind {
            TokenKind::Char(value) => ExpressionKind::Char { value },
            TokenKind::Integer(value) => ExpressionKind::Integer { value },
            TokenKind::Keyword(Keyword::False) => ExpressionKind::Bool { value: false },
            TokenKind::Keyword(Keyword::True) => ExpressionKind::Bool { value: true },
            TokenKind::Ident(name) => match self.peek().kind {
                TokenKind::Symbol(Symbol::LParen) => {
                    let (name, args) = self.parse_call(name)?;
                    ExpressionKind::Call { name, args }
                }
                _ => ExpressionKind::Ident { name },
            },

            TokenKind::Symbol(Symbol::LParen) => {
                let expr = self.parse_add()?;
                self.expect(TokenKind::Symbol(Symbol::RParen))?;
                return Ok(expr);
            }
            x => {
                return Err(Error::new(
                    token.pos,
                    ErrorKind::UnexpectedToken {
                        expected: None,
                        actual: x,
                    },
                ));
            }
        };

        Ok(Expression::new(kind, token.pos))
    }

    fn parse_call(&mut self, name: String) -> Result<FuncCall, Error> {
        self.consume();
        let args = self.parse_call_arguments()?;
        self.expect(TokenKind::Symbol(Symbol::RParen))?;
        Ok((name, args))
    }

    fn parse_call_arguments(&mut self) -> Result<Vec<Expression>, Error> {
        let mut args = Vec::new();
        if self.peek().kind != TokenKind::Symbol(Symbol::RParen) {
            args.push(self.parse_expression()?);
        }
        while self.peek().kind != TokenKind::Symbol(Symbol::RParen) {
            self.expect(TokenKind::Symbol(Symbol::Comma))?;
            args.push(self.parse_expression()?);
        }
        Ok(args)
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

    fn consume_ident(&mut self) -> Result<String, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::Ident(name) => Ok(name),
            _ => Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedIdent {
                    actual: next_token.kind,
                },
            )),
        }
    }

    fn consume_int(&mut self) -> Result<i32, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::Integer(value) => Ok(value),
            _ => Err(Error::new(
                next_token.pos,
                ErrorKind::UnexpectedToken {
                    actual: next_token.kind,
                    expected: Some(TokenKind::Integer(0)),
                },
            )),
        }
    }

    fn consume_type(&mut self) -> Result<Type, Error> {
        if self.peek().kind == TokenKind::Symbol(Symbol::Asterisk) {
            self.consume();
            return Ok(self.consume_type()?.pointer_to());
        }

        let next_token_pos = self.peek().pos;
        let mut typ = match self.consume_ident()?.as_str() {
            "byte" => Type::Byte,
            "int" => Type::Int,
            "bool" => Type::Bool,
            x => {
                return Err(Error::new(
                    next_token_pos,
                    ErrorKind::NotTypeName { name: x.into() },
                ))
            }
        };

        if self.peek().kind == TokenKind::Symbol(Symbol::LBracket) {
            self.consume();
            let len = self.consume_int()? as u32;
            self.expect(TokenKind::Symbol(Symbol::RBracket))?;

            typ = Type::Array {
                elm_type: Box::new(typ),
                len,
            }
        }

        Ok(typ)
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

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).unwrap().clone()
    }

    fn is_eof(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }
}

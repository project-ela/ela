pub mod ast;
pub mod context;

use crate::frontend::{
    lexer::token::Token,
    parser::{
        ast::{AstExpression, AstStatement, Function, Operator, Program},
        context::{Context, Type},
    },
};

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
        let mut ctx = Context::new();
        ctx.add_type(&"int".to_string(), Type::Int);
        ctx.add_type(&"bool".to_string(), Type::Bool);
        while !self.is_eof() {
            program.functions.push(self.parse_function(&mut ctx)?);
        }
        Ok(program)
    }

    fn parse_function(&mut self, ctx: &mut Context) -> Result<Function, String> {
        self.expect(Token::Func)?;
        let name = self.consume_ident()?;
        self.expect(Token::LParen)?;
        self.expect(Token::RParen)?;
        self.expect(Token::Colon)?;
        let typ = self.consume_type(ctx)?;
        let body = self.parse_statement(ctx)?;
        ctx.add_function(&name, &typ);
        Ok(Function {
            name,
            body,
            ctx: ctx.clone(),
        })
    }

    fn parse_statement(&mut self, ctx: &mut Context) -> Result<AstStatement, String> {
        match self.consume() {
            Token::LBrace => {
                let mut stmts = Vec::new();
                loop {
                    if self.peek() == Token::RBrace {
                        self.consume();
                        break;
                    }
                    stmts.push(self.parse_statement(ctx)?);
                }
                Ok(AstStatement::Block { stmts })
            }
            Token::Var => {
                let name = self.consume_ident()?;
                self.expect(Token::Colon)?;
                let typ = self.consume_type(ctx)?;
                self.expect(Token::Assign)?;
                let value = self.parse_expression(ctx)?;
                ctx.add_variable(&name, &typ);
                Ok(AstStatement::Declare {
                    name,
                    typ,
                    value: Box::new(value),
                })
            }
            Token::Ident { name } => {
                if ctx.find_variable(&name).is_none() {
                    return Err(format!("undefined variable: {}", name));
                }

                self.expect(Token::Assign)?;
                let value = self.parse_expression(ctx)?;
                Ok(AstStatement::Assign {
                    name,
                    value: Box::new(value),
                })
            }
            Token::Return => {
                let value = self.parse_expression(ctx)?;
                Ok(AstStatement::Return {
                    value: Box::new(value),
                })
            }
            Token::If => {
                let cond = self.parse_expression(ctx)?;
                let then = self.parse_statement(ctx)?;
                let els = if self.peek() == Token::Else {
                    self.consume();
                    let els = self.parse_statement(ctx)?;
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

    fn parse_expression(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        self.parse_bitor(ctx)
    }

    fn parse_bitor(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_bitxor(ctx)?;
        loop {
            match self.peek() {
                Token::Or => node = new_binop!(self, Operator::Or, node, self.parse_bitxor(ctx)?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_bitxor(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_bitand(ctx)?;
        loop {
            match self.peek() {
                Token::Xor => node = new_binop!(self, Operator::Xor, node, self.parse_bitand(ctx)?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_bitand(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_equal(ctx)?;
        loop {
            match self.peek() {
                Token::And => node = new_binop!(self, Operator::And, node, self.parse_equal(ctx)?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_equal(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_relation(ctx)?;
        loop {
            match self.peek() {
                Token::Equal => {
                    node = new_binop!(self, Operator::Equal, node, self.parse_relation(ctx)?)
                }
                Token::NotEqual => {
                    node = new_binop!(self, Operator::NotEqual, node, self.parse_relation(ctx)?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_relation(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_add(ctx)?;
        loop {
            match self.peek() {
                Token::Lt => node = new_binop!(self, Operator::Lt, node, self.parse_add(ctx)?),
                Token::Lte => node = new_binop!(self, Operator::Lte, node, self.parse_add(ctx)?),
                Token::Gt => node = new_binop!(self, Operator::Gt, node, self.parse_add(ctx)?),
                Token::Gte => node = new_binop!(self, Operator::Gte, node, self.parse_add(ctx)?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_add(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_mul(ctx)?;
        loop {
            match self.peek() {
                Token::Plus => node = new_binop!(self, Operator::Add, node, self.parse_mul(ctx)?),
                Token::Minus => node = new_binop!(self, Operator::Sub, node, self.parse_mul(ctx)?),
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_mul(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        let mut node = self.parse_unary(ctx)?;
        loop {
            match self.peek() {
                Token::Asterisk => {
                    node = new_binop!(self, Operator::Mul, node, self.parse_unary(ctx)?)
                }
                Token::Slash => {
                    node = new_binop!(self, Operator::Div, node, self.parse_unary(ctx)?)
                }
                _ => break,
            }
        }

        Ok(node)
    }

    fn parse_unary(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        match self.peek() {
            Token::Plus => Ok(new_binop!(
                self,
                Operator::Add,
                AstExpression::Integer { value: 0 },
                self.parse_unary(ctx)?
            )),
            Token::Minus => Ok(new_binop!(
                self,
                Operator::Sub,
                AstExpression::Integer { value: 0 },
                self.parse_unary(ctx)?
            )),
            _ => Ok(self.parse_primary(ctx)?),
        }
    }

    fn parse_primary(&mut self, ctx: &mut Context) -> Result<AstExpression, String> {
        match self.consume() {
            Token::IntLiteral { value } => Ok(AstExpression::Integer { value: value }),
            Token::Ident { name } => match ctx.find_variable(&name) {
                Some(_) => Ok(AstExpression::Ident { name }),
                None => Err(format!("undefined variable: {}", name)),
            },
            Token::LParen => {
                let expr = self.parse_add(ctx)?;
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

    fn consume_type(&mut self, ctx: &mut Context) -> Result<Type, String> {
        let typ_name = self.consume_ident()?;
        match ctx.find_type(&typ_name) {
            Some(typ) => Ok(typ.clone()),
            None => Err(format!("undefined type: {}", typ_name)),
        }
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens.get(self.pos).unwrap_or(&Token::EOF);
        self.pos += 1;
        token.clone()
    }
}

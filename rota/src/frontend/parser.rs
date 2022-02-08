pub mod node;

use x86asm::instruction::mnemonic;

use crate::{
    common::error::{Error, ErrorKind},
    frontend::{
        lexer::token::{Keyword, Symbol, Token, TokenKind},
        parser::node::{
            DispNode, InstructionNode, MemoryNode, OperandNode, Program, PseudoOp, PseudoOpArg,
        },
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

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<Program, Error> {
        let mut insts = Vec::new();
        loop {
            if self.is_eof() {
                break;
            }

            if matches!(self.peek().kind, TokenKind::Comment(_)) {
                self.consume();
                continue;
            }

            if !matches!(self.peek().kind, TokenKind::Ident(_)) {
                insts.push(self.parse_inst()?);
                continue;
            }

            let ident_token = self.peek().clone();
            let ident = self.consume_ident()?;

            if self.peek().kind == TokenKind::Symbol(Symbol::Colon) {
                self.consume();
                insts.push(InstructionNode::Label(ident));
                continue;
            }

            if ident.starts_with('.') {
                insts.push(self.parse_pseudop(ident_token)?);
                continue;
            }

            return Err(unexpected(ident_token));
        }
        Ok(Program { insts })
    }

    fn parse_inst(&mut self) -> Result<InstructionNode, Error> {
        let token = self.consume().clone();
        match token.kind {
            TokenKind::Mnemonic(mnemonic) => match mnemonic.typ() {
                mnemonic::Type::Nullary => Ok(InstructionNode::NullaryOp(mnemonic)),
                mnemonic::Type::Unary => {
                    let operand1 = self.parse_operand()?;
                    Ok(InstructionNode::UnaryOp(mnemonic, operand1))
                }
                mnemonic::Type::Binary => {
                    let operand1 = self.parse_operand()?;
                    self.expect(TokenKind::Symbol(Symbol::Comma))?;
                    let operand2 = self.parse_operand()?;
                    Ok(InstructionNode::BinaryOp(mnemonic, operand1, operand2))
                }
            },
            _ => Err(unexpected(token)),
        }
    }

    fn parse_operand(&mut self) -> Result<OperandNode, Error> {
        let token = self.consume();
        match token.kind {
            TokenKind::Symbol(Symbol::Plus) => Ok(OperandNode::Immidiate(self.consume_integer()?)),
            TokenKind::Symbol(Symbol::Minus) => {
                Ok(OperandNode::Immidiate(-self.consume_integer()?))
            }
            TokenKind::Integer(value) => Ok(OperandNode::Immidiate(value)),
            TokenKind::Ident(name) => Ok(OperandNode::Label(name.to_owned())),
            TokenKind::Register(reg) => Ok(OperandNode::Register(reg.to_owned())),
            TokenKind::Symbol(Symbol::LBracket) => self.parse_operand_address(),
            // TODO
            TokenKind::Keyword(Keyword::Byte) => {
                self.expect(TokenKind::Keyword(Keyword::Ptr))?;
                self.expect(TokenKind::Symbol(Symbol::LBracket))?;
                self.parse_operand_address()
            }
            _ => Err(unexpected(token)),
        }
    }

    fn parse_operand_address(&mut self) -> Result<OperandNode, Error> {
        let token = self.consume();
        let base = match token.kind {
            TokenKind::Register(reg) => reg.clone(),
            _ => return Err(unexpected(token)),
        };

        // TODO
        let token = self.peek();
        let disp = match token.kind {
            TokenKind::Symbol(Symbol::RBracket) => None,
            TokenKind::Symbol(Symbol::Minus) => {
                self.consume();
                Some(DispNode::Immediate(-self.consume_integer()?))
            }
            TokenKind::Symbol(Symbol::Plus) => {
                self.consume();
                let token = self.consume();
                match token.kind {
                    TokenKind::Integer(value) => Some(DispNode::Immediate(value)),
                    TokenKind::Ident(name) => Some(DispNode::Label(name)),
                    _ => return Err(unexpected(token)),
                }
            }
            _ => return Err(unexpected(token)),
        };

        self.expect(TokenKind::Symbol(Symbol::RBracket))?;
        Ok(OperandNode::Memory(MemoryNode { base, disp }))
    }

    fn parse_pseudop(&mut self, ident_token: Token) -> Result<InstructionNode, Error> {
        let op = find_pseudoop(ident_token)?;
        let args = match op {
            PseudoOp::Tse => {
                let mut args = Vec::new();
                args.push(PseudoOpArg::Integer(self.consume_signed_integer()?));
                self.expect(TokenKind::Symbol(Symbol::Comma))?;
                args.push(PseudoOpArg::Integer(self.consume_signed_integer()?));
                self.expect(TokenKind::Symbol(Symbol::Comma))?;
                args.push(PseudoOpArg::Integer(self.consume_signed_integer()?));
                args
            }
            PseudoOp::IntelSyntax | PseudoOp::Global => {
                vec![PseudoOpArg::String(self.consume_ident()?)]
            }
            PseudoOp::Zero => vec![PseudoOpArg::Integer(self.consume_integer()?)],
            PseudoOp::Ascii => vec![PseudoOpArg::String(self.consume_string()?)],
            _ => vec![],
        };

        Ok(InstructionNode::PseudoOp(op, args))
    }

    fn expect(&mut self, token: TokenKind) -> Result<Token, Error> {
        let next_token = self.consume();
        if next_token.kind == token {
            Ok(next_token)
        } else {
            Err(Error::new(
                next_token.pos,
                ErrorKind::UnexpectedToken {
                    expected: Some(token),
                    actual: next_token.kind,
                },
            ))
        }
    }

    fn consume_integer(&mut self) -> Result<i32, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::Integer(value) => Ok(value),
            x => Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedInteger { actual: x },
            )),
        }
    }

    fn consume_signed_integer(&mut self) -> Result<i32, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::Symbol(Symbol::Minus) => Ok(-self.consume_integer()?),
            TokenKind::Integer(value) => Ok(value),
            x => Err(Error::new(
                next_token.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x,
                },
            )),
        }
    }

    fn consume_string(&mut self) -> Result<String, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::String(value) => Ok(value),
            x => Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedString { actual: x },
            )),
        }
    }

    fn consume_ident(&mut self) -> Result<String, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::Ident(name) => Ok(name),
            x => Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedIdent { actual: x },
            )),
        }
    }

    fn consume(&mut self) -> Token {
        let token = self.tokens.get(self.pos).unwrap();

        if self.pos < self.tokens.len() {
            self.pos += 1;
        }

        token.clone()
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.pos).unwrap().clone()
    }

    fn is_eof(&mut self) -> bool {
        self.peek().kind == TokenKind::EOF
    }
}

fn find_pseudoop(ident: Token) -> Result<PseudoOp, Error> {
    let name = match ident.kind {
        TokenKind::Ident(name) => name,
        _ => return Err(unexpected(ident)),
    };

    match name.as_str() {
        ".global" => Ok(PseudoOp::Global),
        ".intel_syntax" => Ok(PseudoOp::IntelSyntax),
        ".data" => Ok(PseudoOp::Data),
        ".text" => Ok(PseudoOp::Text),
        ".zero" => Ok(PseudoOp::Zero),
        ".ascii" => Ok(PseudoOp::Ascii),
        ".tse" => Ok(PseudoOp::Tse),
        x => Err(Error::new(
            ident.pos,
            ErrorKind::UnknownPseudoOp {
                name: x.to_string(),
            },
        )),
    }
}

fn unexpected(token: Token) -> Error {
    Error::new(
        token.pos,
        ErrorKind::UnexpectedToken {
            expected: None,
            actual: token.kind,
        },
    )
}

pub mod node;

use x86asm::instruction::mnemonic;

use crate::{
    common::error::{Error, ErrorKind},
    frontend::{
        lexer::token::{Keyword, Symbol, Token, TokenKind},
        parser::node::{
            InstructionNode, MemoryNode, OperandNode, Program, PseudoOp, PseudoOpParam,
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
                insts.push(InstructionNode::Label { name: ident });
                continue;
            }

            if ident.starts_with('.') {
                let op = find_pseudoop(ident_token)?;
                let arg = match op {
                    PseudoOp::IntelSyntax | PseudoOp::Global => {
                        PseudoOpParam::String(self.consume_ident()?)
                    }
                    PseudoOp::Zero => PseudoOpParam::Integer(self.consume_integer()?),
                    _ => PseudoOpParam::None,
                };

                insts.push(InstructionNode::PseudoOp(op, arg));
                continue;
            }

            return Err(Error::new(
                ident_token.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: ident_token.kind,
                },
            ));
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
            x => Err(Error::new(
                token.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x,
                },
            )),
        }
    }

    fn parse_operand(&mut self) -> Result<OperandNode, Error> {
        let token = self.consume();
        match token.kind {
            TokenKind::Integer(value) => Ok(OperandNode::Immidiate { value }),
            TokenKind::Ident(name) => Ok(OperandNode::Label {
                name: name.to_owned(),
            }),
            TokenKind::Register(reg) => Ok(OperandNode::Register {
                reg: reg.to_owned(),
            }),
            TokenKind::Symbol(Symbol::LBracket) => self.parse_operand_address(),
            // TODO
            TokenKind::Keyword(Keyword::Byte) => {
                self.expect(TokenKind::Keyword(Keyword::Ptr))?;
                self.expect(TokenKind::Symbol(Symbol::LBracket))?;
                self.parse_operand_address()
            }
            x => Err(Error::new(
                token.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x.clone(),
                },
            )),
        }
    }

    fn parse_operand_address(&mut self) -> Result<OperandNode, Error> {
        let token = self.consume();
        let base = match token.kind {
            TokenKind::Register(reg) => reg.clone(),
            x => {
                return Err(Error::new(
                    token.pos,
                    ErrorKind::UnexpectedToken {
                        expected: None,
                        actual: x.clone(),
                    },
                ))
            }
        };

        let disp = match self.peek().kind {
            TokenKind::Symbol(Symbol::Plus) => {
                self.consume();
                Some(self.consume_integer()? as i32)
            }
            TokenKind::Symbol(Symbol::Minus) => {
                self.consume();
                Some(-(self.consume_integer()? as i32))
            }
            _ => None,
        };

        self.expect(TokenKind::Symbol(Symbol::RBracket))?;
        Ok(OperandNode::Memory(MemoryNode { base, disp }))
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

    fn consume_integer(&mut self) -> Result<u32, Error> {
        let next_token = self.consume();
        match next_token.kind {
            TokenKind::Integer(value) => Ok(value),
            x => Err(Error::new(
                next_token.pos,
                ErrorKind::ExpectedInteger { actual: x },
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
        x => {
            return Err(Error::new(
                ident.pos,
                ErrorKind::UnexpectedToken {
                    expected: None,
                    actual: x,
                },
            ))
        }
    };

    match name.as_str() {
        ".global" => Ok(PseudoOp::Global),
        ".intel_syntax" => Ok(PseudoOp::IntelSyntax),
        ".data" => Ok(PseudoOp::Data),
        ".text" => Ok(PseudoOp::Text),
        ".zero" => Ok(PseudoOp::Zero),
        x => Err(Error::new(
            ident.pos,
            ErrorKind::UnknownPseudoOp {
                name: x.to_string(),
            },
        )),
    }
}

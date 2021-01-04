pub mod node;

use node::{InstructionNode, MemoryNode, OperandNode, PseudoOp};
use x86asm::instruction::mnemonic;

use crate::frontend::lexer::token::{Symbol, Token};

use super::lexer::token::Keyword;

struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<InstructionNode>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<Vec<InstructionNode>, String> {
        let mut insts = Vec::new();
        loop {
            if self.is_eof() {
                break;
            }

            if matches!(self.peek(), Token::Comment(_)) {
                self.consume();
                continue;
            }

            if !matches!(self.peek(), Token::Ident(_)) {
                insts.push(self.parse_inst()?);
                continue;
            }

            let ident = self.consume_ident()?;

            if self.peek() == &Token::Symbol(Symbol::Colon) {
                self.consume();
                insts.push(InstructionNode::Label { name: ident });
                continue;
            }

            if ident.starts_with('.') {
                let op = find_pseudoop(&ident).ok_or(format!("unknown pseudo-op: {}", ident))?;
                let arg = self.consume_ident()?;
                insts.push(InstructionNode::PseudoOp(op, arg));
                continue;
            }

            return Err(format!("unexpected token: {}", ident));
        }
        Ok(insts)
    }

    fn parse_inst(&mut self) -> Result<InstructionNode, String> {
        let token = self.consume().clone();
        match token {
            Token::Mnemonic(mnemonic) => match mnemonic.typ() {
                mnemonic::Type::Nullary => Ok(InstructionNode::NullaryOp(mnemonic)),
                mnemonic::Type::Unary => {
                    let operand1 = self.parse_operand()?;
                    Ok(InstructionNode::UnaryOp(mnemonic, operand1))
                }
                mnemonic::Type::Binary => {
                    let operand1 = self.parse_operand()?;
                    self.expect(&Token::Symbol(Symbol::Comma))?;
                    let operand2 = self.parse_operand()?;
                    Ok(InstructionNode::BinaryOp(mnemonic, operand1, operand2))
                }
            },
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn parse_operand(&mut self) -> Result<OperandNode, String> {
        match self.consume() {
            Token::Integer(value) => Ok(OperandNode::Immidiate { value: *value }),
            Token::Ident(name) => Ok(OperandNode::Label {
                name: name.to_owned(),
            }),
            Token::Register(reg) => Ok(OperandNode::Register {
                reg: reg.to_owned(),
            }),
            Token::Symbol(Symbol::LBracket) => self.parse_operand_address(),
            // TODO
            Token::Keyword(Keyword::Byte) => {
                self.expect(&Token::Keyword(Keyword::Ptr))?;
                self.expect(&Token::Symbol(Symbol::LBracket))?;
                self.parse_operand_address()
            }
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn parse_operand_address(&mut self) -> Result<OperandNode, String> {
        let base = match self.consume() {
            Token::Register(reg) => reg.clone(),
            x => return Err(format!("unexpected token: {:?}", x)),
        };

        let disp = match self.peek() {
            Token::Symbol(Symbol::Plus) => {
                self.consume();
                Some(self.consume_integer()? as i32)
            }
            Token::Symbol(Symbol::Minus) => {
                self.consume();
                Some(-(self.consume_integer()? as i32))
            }
            _ => None,
        };

        self.expect(&Token::Symbol(Symbol::RBracket))?;
        Ok(OperandNode::Memory(MemoryNode { base, disp }))
    }

    fn expect(&mut self, token: &Token) -> Result<&Token, String> {
        let next_token = self.consume();
        if next_token == token {
            Ok(next_token)
        } else {
            Err(format!("expected {:?}, but got {:?}", token, next_token))
        }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
    }

    fn consume_integer(&mut self) -> Result<u32, String> {
        let next_token = self.consume();
        if let Token::Integer(value) = next_token {
            Ok(*value)
        } else {
            Err(format!("expected integer, but got {:?}", next_token))
        }
    }

    fn consume_ident(&mut self) -> Result<String, String> {
        let next_token = self.consume();
        if let Token::Ident(name) = next_token {
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

    fn is_eof(&mut self) -> bool {
        self.peek() == &Token::EOF
    }
}

fn find_pseudoop(ident: &str) -> Option<PseudoOp> {
    match ident {
        ".global" => Some(PseudoOp::Global),
        ".intel_syntax" => Some(PseudoOp::IntelSyntax),
        _ => None,
    }
}

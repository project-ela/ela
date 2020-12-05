use crate::instruction::{Instruction, MnemonicType, Operand};
use crate::token::{Symbol, Token};

struct Parser {
    pos: usize,
    tokens: Vec<Token>,
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Instruction>, String> {
    let mut parser = Parser::new(tokens);
    parser.parse()
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { pos: 0, tokens }
    }

    fn parse(&mut self) -> Result<Vec<Instruction>, String> {
        let mut insts = Vec::new();
        loop {
            if self.is_eof() {
                break;
            }

            if !matches!(self.peek(), Token::Ident(_)) {
                insts.push(self.parse_inst()?);
                continue;
            }

            let ident = self.consume_ident()?;

            if self.peek() == &Token::Symbol(Symbol::Colon) {
                self.consume();
                insts.push(Instruction::Label { name: ident });
                continue;
            }

            if ident.chars().next().unwrap() == '.' {
                let arg = self.consume_ident()?;
                insts.push(Instruction::PseudoOp { name: ident, arg });
                continue;
            }

            return Err(format!("unexpected token: {}", ident));
        }
        Ok(insts)
    }

    fn parse_inst(&mut self) -> Result<Instruction, String> {
        let token = self.consume().clone();
        match token {
            Token::Mnemonic(mnemonic) => match mnemonic.typ() {
                MnemonicType::Nullary => Ok(Instruction::NullaryOp(mnemonic)),
                MnemonicType::Unary => {
                    let operand1 = self.parse_operand()?;
                    Ok(Instruction::UnaryOp(mnemonic, operand1))
                }
                MnemonicType::Binary => {
                    let operand1 = self.parse_operand()?;
                    self.expect(&Token::Symbol(Symbol::Comma))?;
                    let operand2 = self.parse_operand()?;
                    Ok(Instruction::BinaryOp(mnemonic, operand1, operand2))
                }
            },
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, String> {
        match self.consume() {
            Token::Integer(value) => Ok(Operand::Immidiate { value: *value }),
            Token::Ident(name) => Ok(Operand::Label {
                name: name.to_owned(),
            }),
            Token::Register(reg) => Ok(Operand::Register {
                reg: reg.to_owned(),
            }),
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

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::EOF)
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

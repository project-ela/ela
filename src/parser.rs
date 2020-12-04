use crate::instruction::{Instruction, Opcode, Operand, Register};
use crate::token::Token;

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

            match self.peek() {
                Token::Ident { name: _ } => {}
                _ => {
                    insts.push(self.parse_inst()?);
                    continue;
                }
            }

            let ident = self.consume_ident()?;

            if self.peek() == &Token::Colon {
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
        let token = self.consume();
        let opcode = token_to_opcode(token)?;
        match token {
            Token::Ret => Ok(Instruction::NullaryOp(opcode)),
            Token::Push
            | Token::Pop
            | Token::IDiv
            | Token::Jmp
            | Token::Sete
            | Token::Je
            | Token::Setne
            | Token::Setl
            | Token::Setle
            | Token::Setg
            | Token::Setge
            | Token::Call => {
                let operand1 = self.parse_operand()?;
                Ok(Instruction::UnaryOp(opcode, operand1))
            }
            Token::Add
            | Token::Sub
            | Token::IMul
            | Token::Xor
            | Token::Mov
            | Token::And
            | Token::Or
            | Token::Cmp => {
                let operand1 = self.parse_operand()?;
                self.expect(&Token::Commna)?;
                let operand2 = self.parse_operand()?;
                Ok(Instruction::BinaryOp(opcode, operand1, operand2))
            }
            x => Err(format!("unexpected token: {:?}", x)),
        }
    }

    fn parse_operand(&mut self) -> Result<Operand, String> {
        match self.consume() {
            Token::Integer { value } => Ok(Operand::Immidiate { value: *value }),
            Token::Ident { name } => Ok(Operand::Label {
                name: name.to_owned(),
            }),

            Token::Rax => Ok(Operand::Register { reg: Register::Rax }),
            Token::Rcx => Ok(Operand::Register { reg: Register::Rcx }),
            Token::Rdx => Ok(Operand::Register { reg: Register::Rdx }),
            Token::Rbx => Ok(Operand::Register { reg: Register::Rbx }),
            Token::Rsp => Ok(Operand::Register { reg: Register::Rsp }),
            Token::Rbp => Ok(Operand::Register { reg: Register::Rbp }),
            Token::Rsi => Ok(Operand::Register { reg: Register::Rsi }),
            Token::Rdi => Ok(Operand::Register { reg: Register::Rdi }),
            Token::R8 => Ok(Operand::Register { reg: Register::R8 }),
            Token::R9 => Ok(Operand::Register { reg: Register::R9 }),
            Token::R10 => Ok(Operand::Register { reg: Register::R10 }),
            Token::R11 => Ok(Operand::Register { reg: Register::R11 }),
            Token::R12 => Ok(Operand::Register { reg: Register::R12 }),
            Token::R13 => Ok(Operand::Register { reg: Register::R13 }),
            Token::R14 => Ok(Operand::Register { reg: Register::R14 }),
            Token::R15 => Ok(Operand::Register { reg: Register::R15 }),

            Token::Eax => Ok(Operand::Register { reg: Register::Eax }),
            Token::Ecx => Ok(Operand::Register { reg: Register::Ecx }),
            Token::Edx => Ok(Operand::Register { reg: Register::Edx }),
            Token::Ebx => Ok(Operand::Register { reg: Register::Ebx }),
            Token::Esp => Ok(Operand::Register { reg: Register::Esp }),
            Token::Ebp => Ok(Operand::Register { reg: Register::Ebp }),
            Token::Esi => Ok(Operand::Register { reg: Register::Esi }),
            Token::Edi => Ok(Operand::Register { reg: Register::Edi }),

            Token::Al => Ok(Operand::Register { reg: Register::Al }),
            Token::Cl => Ok(Operand::Register { reg: Register::Cl }),
            Token::Dl => Ok(Operand::Register { reg: Register::Dl }),
            Token::Bl => Ok(Operand::Register { reg: Register::Bl }),

            Token::R8b => Ok(Operand::Register { reg: Register::R8b }),
            Token::R9b => Ok(Operand::Register { reg: Register::R9b }),
            Token::R10b => Ok(Operand::Register {
                reg: Register::R10b,
            }),
            Token::R11b => Ok(Operand::Register {
                reg: Register::R11b,
            }),
            Token::R12b => Ok(Operand::Register {
                reg: Register::R12b,
            }),
            Token::R13b => Ok(Operand::Register {
                reg: Register::R13b,
            }),
            Token::R14b => Ok(Operand::Register {
                reg: Register::R14b,
            }),
            Token::R15b => Ok(Operand::Register {
                reg: Register::R15b,
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

    fn is_eof(&mut self) -> bool {
        self.peek() == &Token::EOF
    }
}

fn token_to_opcode(token: &Token) -> Result<Opcode, String> {
    match token {
        Token::Push => Ok(Opcode::Push),
        Token::Pop => Ok(Opcode::Pop),
        Token::Add => Ok(Opcode::Add),
        Token::Sub => Ok(Opcode::Sub),
        Token::IMul => Ok(Opcode::IMul),
        Token::IDiv => Ok(Opcode::IDiv),
        Token::Xor => Ok(Opcode::Xor),
        Token::Ret => Ok(Opcode::Ret),
        Token::Mov => Ok(Opcode::Mov),
        Token::Jmp => Ok(Opcode::Jmp),
        Token::And => Ok(Opcode::And),
        Token::Or => Ok(Opcode::Or),
        Token::Cmp => Ok(Opcode::Cmp),
        Token::Sete => Ok(Opcode::Sete),
        Token::Je => Ok(Opcode::Je),
        Token::Setne => Ok(Opcode::Setne),
        Token::Setl => Ok(Opcode::Setl),
        Token::Setle => Ok(Opcode::Setle),
        Token::Setg => Ok(Opcode::Setg),
        Token::Setge => Ok(Opcode::Setge),
        Token::Call => Ok(Opcode::Call),
        x => Err(format!("unexpected token: {:?}", x)),
    }
}

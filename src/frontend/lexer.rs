pub mod token;

use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

use crate::frontend::lexer::token::{Symbol, Token};

struct Lexer {
    pos: usize,
    source: String,
}

pub fn tokenize(source: String) -> Result<Vec<Token>, String> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

impl Lexer {
    fn new(source: String) -> Self {
        Self { pos: 0, source }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            tokens.push(self.next_token()?);
        }
        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.consume_whitespace();
        if self.is_eof() {
            return Ok(Token::EOF);
        }

        let token = match self.peek_char() {
            ',' => Token::Symbol(Symbol::Comma),
            ':' => Token::Symbol(Symbol::Colon),
            ';' => {
                self.consume_char();
                return Ok(Token::Comment(self.consume_comment()));
            }
            '[' => Token::Symbol(Symbol::LBracket),
            ']' => Token::Symbol(Symbol::RBracket),
            '+' => Token::Symbol(Symbol::Plus),
            '-' => Token::Symbol(Symbol::Minus),
            x if x.is_digit(10) => {
                let value = self.consume_number();
                return Ok(Token::Integer(value));
            }
            x if self.is_ident(x) => {
                let name = self.consume_ident();
                return match find_keyword(&name) {
                    Some(token) => Ok(token),
                    None => Ok(Token::Ident(name)),
                };
            }
            x => return Err(format!("unexpected char: {}", x)),
        };
        self.consume_char();
        Ok(token)
    }

    fn consume_comment(&mut self) -> String {
        let mut result = String::new();
        while !self.is_eof() && self.peek_char() != '\n' {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_whitespace(&mut self) {
        while !self.is_eof() && self.peek_char().is_whitespace() {
            self.consume_char();
        }
    }

    fn consume_number(&mut self) -> u32 {
        let mut result = String::new();
        while !self.is_eof() && self.peek_char().is_digit(10) {
            result.push(self.consume_char());
        }
        result.parse().unwrap()
    }

    fn consume_ident(&mut self) -> String {
        let mut result = String::new();
        loop {
            let cur_char = self.peek_char();
            if self.is_eof() {
                break;
            }
            if !(self.is_ident(cur_char) || cur_char.is_digit(10)) {
                break;
            }
            result.push(self.consume_char());
        }
        result
    }

    fn is_ident(&self, c: char) -> bool {
        match c {
            '.' | '_' => true,
            x if x.is_alphabetic() => true,
            _ => false,
        }
    }

    fn peek_char(&mut self) -> char {
        self.source[self.pos..].chars().next().unwrap_or(' ')
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.source[self.pos..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));
        self.pos += next_pos;
        cur_char
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }
}

fn find_keyword(ident: &str) -> Option<Token> {
    match ident {
        "add" => Some(Token::Mnemonic(Mnemonic::Add)),
        "and" => Some(Token::Mnemonic(Mnemonic::And)),
        "call" => Some(Token::Mnemonic(Mnemonic::Call)),
        "cmp" => Some(Token::Mnemonic(Mnemonic::Cmp)),
        "hlt" => Some(Token::Mnemonic(Mnemonic::Hlt)),
        "idiv" => Some(Token::Mnemonic(Mnemonic::IDiv)),
        "imul" => Some(Token::Mnemonic(Mnemonic::IMul)),
        "je" => Some(Token::Mnemonic(Mnemonic::Je)),
        "jmp" => Some(Token::Mnemonic(Mnemonic::Jmp)),
        "lea" => Some(Token::Mnemonic(Mnemonic::Lea)),
        "mov" => Some(Token::Mnemonic(Mnemonic::Mov)),
        "or" => Some(Token::Mnemonic(Mnemonic::Or)),
        "pop" => Some(Token::Mnemonic(Mnemonic::Pop)),
        "push" => Some(Token::Mnemonic(Mnemonic::Push)),
        "ret" => Some(Token::Mnemonic(Mnemonic::Ret)),
        "sete" => Some(Token::Mnemonic(Mnemonic::Sete)),
        "setg" => Some(Token::Mnemonic(Mnemonic::Setg)),
        "setge" => Some(Token::Mnemonic(Mnemonic::Setge)),
        "setl" => Some(Token::Mnemonic(Mnemonic::Setl)),
        "setle" => Some(Token::Mnemonic(Mnemonic::Setle)),
        "setne" => Some(Token::Mnemonic(Mnemonic::Setne)),
        "sub" => Some(Token::Mnemonic(Mnemonic::Sub)),
        "syscall" => Some(Token::Mnemonic(Mnemonic::Syscall)),
        "xor" => Some(Token::Mnemonic(Mnemonic::Xor)),

        "rax" => Some(Token::Register(Register::Rax)),
        "rcx" => Some(Token::Register(Register::Rcx)),
        "rdx" => Some(Token::Register(Register::Rdx)),
        "rbx" => Some(Token::Register(Register::Rbx)),
        "rsp" => Some(Token::Register(Register::Rsp)),
        "rbp" => Some(Token::Register(Register::Rbp)),
        "rsi" => Some(Token::Register(Register::Rsi)),
        "rdi" => Some(Token::Register(Register::Rdi)),
        "r8" => Some(Token::Register(Register::R8)),
        "r9" => Some(Token::Register(Register::R9)),
        "r10" => Some(Token::Register(Register::R10)),
        "r11" => Some(Token::Register(Register::R11)),
        "r12" => Some(Token::Register(Register::R12)),
        "r13" => Some(Token::Register(Register::R13)),
        "r14" => Some(Token::Register(Register::R14)),
        "r15" => Some(Token::Register(Register::R15)),

        "eax" => Some(Token::Register(Register::Eax)),
        "ecx" => Some(Token::Register(Register::Ecx)),
        "edx" => Some(Token::Register(Register::Edx)),
        "ebx" => Some(Token::Register(Register::Ebx)),
        "esp" => Some(Token::Register(Register::Esp)),
        "ebp" => Some(Token::Register(Register::Ebp)),
        "esi" => Some(Token::Register(Register::Esi)),
        "edi" => Some(Token::Register(Register::Edi)),

        "al" => Some(Token::Register(Register::Al)),
        "cl" => Some(Token::Register(Register::Cl)),
        "dl" => Some(Token::Register(Register::Dl)),
        "bl" => Some(Token::Register(Register::Bl)),

        "r8b" => Some(Token::Register(Register::R8b)),
        "r9b" => Some(Token::Register(Register::R9b)),
        "r10b" => Some(Token::Register(Register::R10b)),
        "r11b" => Some(Token::Register(Register::R11b)),
        "r12b" => Some(Token::Register(Register::R12b)),
        "r13b" => Some(Token::Register(Register::R13b)),
        "r14b" => Some(Token::Register(Register::R14b)),
        "r15b" => Some(Token::Register(Register::R15b)),

        _ => None,
    }
}

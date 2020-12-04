use crate::token::Token;

struct Tokenizer {
    pos: usize,
    source: String,
}

pub fn tokenize(source: String) -> Result<Vec<Token>, String> {
    let mut tokenizer = Tokenizer::new(source);
    tokenizer.tokenize()
}

impl Tokenizer {
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
            ',' => Token::Commna,
            ':' => Token::Colon,
            x if x.is_digit(10) => {
                let value = self.consume_number();
                return Ok(Token::Integer { value });
            }
            x if self.is_ident(x) => {
                let name = self.consume_ident();
                return match find_keyword(&name) {
                    Some(token) => Ok(token),
                    None => Ok(Token::Ident { name }),
                };
            }
            x => return Err(format!("unexpected char: {}", x)),
        };
        self.consume_char();
        Ok(token)
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

fn find_keyword(ident: &String) -> Option<Token> {
    match ident.as_str() {
        "push" => Some(Token::Push),
        "pop" => Some(Token::Pop),
        "add" => Some(Token::Add),
        "sub" => Some(Token::Sub),
        "imul" => Some(Token::IMul),
        "idiv" => Some(Token::IDiv),
        "xor" => Some(Token::Xor),
        "ret" => Some(Token::Ret),
        "mov" => Some(Token::Mov),
        "jmp" => Some(Token::Jmp),
        "and" => Some(Token::And),
        "or" => Some(Token::Or),
        "cmp" => Some(Token::Cmp),
        "sete" => Some(Token::Sete),
        "je" => Some(Token::Je),
        "setne" => Some(Token::Setne),
        "setl" => Some(Token::Setl),
        "setle" => Some(Token::Setle),
        "setg" => Some(Token::Setg),
        "setge" => Some(Token::Setge),
        "call" => Some(Token::Call),

        "rax" => Some(Token::Rax),
        "rcx" => Some(Token::Rcx),
        "rdx" => Some(Token::Rdx),
        "rbx" => Some(Token::Rbx),
        "rsp" => Some(Token::Rsp),
        "rbp" => Some(Token::Rbp),
        "rsi" => Some(Token::Rsi),
        "rdi" => Some(Token::Rdi),
        "r8" => Some(Token::R8),
        "r9" => Some(Token::R9),
        "r10" => Some(Token::R10),
        "r11" => Some(Token::R11),
        "r12" => Some(Token::R12),
        "r13" => Some(Token::R13),
        "r14" => Some(Token::R14),
        "r15" => Some(Token::R15),

        "eax" => Some(Token::Eax),
        "ecx" => Some(Token::Ecx),
        "edx" => Some(Token::Edx),
        "ebx" => Some(Token::Ebx),
        "esp" => Some(Token::Esp),
        "ebp" => Some(Token::Ebp),
        "esi" => Some(Token::Esi),
        "edi" => Some(Token::Edi),

        "al" => Some(Token::Al),
        "cl" => Some(Token::Cl),
        "dl" => Some(Token::Dl),
        "bl" => Some(Token::Bl),

        "r8b" => Some(Token::R8b),
        "r9b" => Some(Token::R9b),
        "r10b" => Some(Token::R10b),
        "r11b" => Some(Token::R11b),
        "r12b" => Some(Token::R12b),
        "r13b" => Some(Token::R13b),
        "r14b" => Some(Token::R14b),
        "r15b" => Some(Token::R15b),

        _ => None,
    }
}

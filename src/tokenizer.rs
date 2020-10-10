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

        "eax" => Some(Token::Eax),
        "ecx" => Some(Token::Ecx),
        "edx" => Some(Token::Edx),
        "ebx" => Some(Token::Ebx),
        "esp" => Some(Token::Esp),
        "ebp" => Some(Token::Ebp),
        "esi" => Some(Token::Esi),
        "edi" => Some(Token::Edi),

        _ => None,
    }
}

use crate::token::Token;

pub struct Tokenizer {
    pos: usize,
    source: String,
}

pub fn tokenize(source: String) -> Result<Vec<Token>, String> {
    let mut tokenizer = Tokenizer::new(source);
    tokenizer.tokenize()
}

impl Tokenizer {
    fn new(source: String) -> Tokenizer {
        Tokenizer { pos: 0, source }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();

        loop {
            if self.is_eof() {
                break;
            }

            match self.next_token() {
                Ok(token) => tokens.push(token),
                Err(err) => return Err(err),
            }
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        if self.is_eof() {
            return Ok(Token::EOF);
        }

        self.consume_whitespace();
        match self.peek_char() {
            x if x.is_digit(10) => Ok(Token::IntLiteral {
                value: self.consume_number(),
            }),
            x => Err(format!("unexpected char: {}", x)),
        }
    }

    fn consume_number(&mut self) -> u32 {
        let mut result = String::new();
        while !self.is_eof() && self.peek_char().is_digit(10) {
            result.push(self.consume_char());
        }
        result.parse().unwrap()
    }

    fn consume_whitespace(&mut self) {
        while !self.is_eof() && self.peek_char().is_whitespace() {
            self.consume_char();
        }
    }

    fn peek_char(&mut self) -> char {
        self.source[self.pos..].chars().next().unwrap()
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

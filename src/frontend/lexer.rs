pub mod token;

use crate::frontend::lexer::token::Token;

struct Tokenizer {
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
        self.consume_whitespace();
        if self.is_eof() {
            return Ok(Token::EOF);
        }

        let token = match self.peek_char() {
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Asterisk),
            '/' => Ok(Token::Slash),
            '&' => Ok(Token::And),
            '|' => Ok(Token::Or),
            '^' => Ok(Token::Xor),
            ':' => Ok(Token::Colon),
            '=' => {
                self.consume_char();
                match self.peek_char() {
                    '=' => {
                        self.consume_char();
                        Ok(Token::Equal)
                    }
                    _ => return Ok(Token::Assign),
                }
            }
            '!' => {
                self.consume_char();
                match self.peek_char() {
                    '=' => {
                        self.consume_char();
                        Ok(Token::NotEqual)
                    }
                    x => Err(format!("expected '=', but got '{}'", x)),
                }
            }
            '<' => {
                self.consume_char();
                if self.peek_char() == '=' {
                    self.consume_char();
                    Ok(Token::Lte)
                } else {
                    Ok(Token::Lt)
                }
            }
            '>' => {
                self.consume_char();
                if self.peek_char() == '=' {
                    self.consume_char();
                    Ok(Token::Gte)
                } else {
                    Ok(Token::Gt)
                }
            }
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            x if x.is_digit(10) => {
                return Ok(Token::IntLiteral {
                    value: self.consume_number(),
                })
            }
            x if x.is_alphabetic() => {
                let ident = self.consume_ident();
                return match find_keyword(&ident) {
                    Some(token) => Ok(token),
                    None => Ok(Token::Ident { name: ident }),
                };
            }
            x => return Err(format!("unexpected char: {}", x)),
        };
        self.consume_char();
        token
    }

    fn consume_ident(&mut self) -> String {
        let mut result = String::new();
        while !self.is_eof() && self.peek_char().is_alphabetic() {
            result.push(self.consume_char());
        }
        result
    }

    fn consume_number(&mut self) -> i32 {
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

fn find_keyword(ident: &String) -> Option<Token> {
    match ident.as_str() {
        "func" => Some(Token::Func),
        "var" => Some(Token::Var),
        "return" => Some(Token::Return),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "false" => Some(Token::False),
        "true" => Some(Token::True),
        _ => None,
    }
}

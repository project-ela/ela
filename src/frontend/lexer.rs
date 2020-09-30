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
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Asterisk),
            '/' => {
                self.consume_char();
                match self.peek_char() {
                    '/' => {
                        self.consume_char();
                        Ok(Token::Comment {
                            content: self.consume_line_comment(),
                        })
                    }
                    '*' => {
                        self.consume_char();
                        Ok(Token::Comment {
                            content: self.consume_block_comment(),
                        })
                    }
                    _ => Ok(Token::Slash),
                }
            }
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
                    _ => return Ok(Token::Not),
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
        let mut ident = String::new();
        while !self.is_eof() && self.peek_char().is_alphabetic() {
            ident.push(self.consume_char());
        }
        ident
    }

    fn consume_number(&mut self) -> i32 {
        let mut digits = String::new();
        while !self.is_eof() && self.peek_char().is_digit(10) {
            digits.push(self.consume_char());
        }
        digits.parse().unwrap()
    }

    fn consume_whitespace(&mut self) {
        while !self.is_eof() && self.peek_char().is_whitespace() {
            self.consume_char();
        }
    }

    fn consume_line_comment(&mut self) -> String {
        let mut content = String::new();
        while !self.is_eof() && self.peek_char() != '\n' {
            content.push(self.consume_char());
        }
        content
    }

    fn consume_block_comment(&mut self) -> String {
        let mut content = String::new();
        while !self.is_eof() {
            match (self.consume_char(), self.consume_char()) {
                ('*', '/') => break,
                (cur_char, next_char) => {
                    content.push(cur_char);
                    content.push(next_char);
                }
            }
        }
        content
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

fn find_keyword(ident: &str) -> Option<Token> {
    match ident {
        "func" => Some(Token::Func),
        "var" => Some(Token::Var),
        "return" => Some(Token::Return),
        "if" => Some(Token::If),
        "else" => Some(Token::Else),
        "false" => Some(Token::False),
        "true" => Some(Token::True),
        "while" => Some(Token::While),
        _ => None,
    }
}

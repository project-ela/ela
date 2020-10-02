pub mod token;

use crate::{
    common::{
        error::{Error, ErrorKind},
        pos::Pos,
    },
    frontend::lexer::token::{Token, TokenKind},
};

struct Tokenizer {
    source: SourceFile,
    source_index: usize,

    pos: Pos,
}

pub struct SourceFile {
    pub filename: String,
    pub content: String,
}

pub fn tokenize(source: SourceFile) -> Result<Vec<Token>, Error> {
    let mut tokenizer = Tokenizer::new(source);
    tokenizer.tokenize()
}

impl Tokenizer {
    fn new(source: SourceFile) -> Tokenizer {
        let pos = Pos {
            filename: source.filename.to_owned(),
            line: 1,
            column: 1,
        };

        Tokenizer {
            source_index: 0,
            source,
            pos,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            tokens.push(Token {
                kind: self.next_token()?,
                pos: self.pos.clone(),
            });
        }

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<TokenKind, Error> {
        self.consume_whitespace();
        if self.is_eof() {
            return Ok(TokenKind::EOF);
        }

        let token = match self.peek_char() {
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Asterisk),
            '/' => {
                self.consume_char();
                match self.peek_char() {
                    '/' => {
                        self.consume_char();
                        Ok(TokenKind::Comment {
                            content: self.consume_line_comment(),
                        })
                    }
                    '*' => {
                        self.consume_char();
                        Ok(TokenKind::Comment {
                            content: self.consume_block_comment(),
                        })
                    }
                    _ => Ok(TokenKind::Slash),
                }
            }
            '&' => Ok(TokenKind::And),
            '|' => Ok(TokenKind::Or),
            '^' => Ok(TokenKind::Xor),
            ':' => Ok(TokenKind::Colon),
            '=' => {
                self.consume_char();
                match self.peek_char() {
                    '=' => {
                        self.consume_char();
                        Ok(TokenKind::Equal)
                    }
                    _ => return Ok(TokenKind::Assign),
                }
            }
            '!' => {
                self.consume_char();
                match self.peek_char() {
                    '=' => {
                        self.consume_char();
                        Ok(TokenKind::NotEqual)
                    }
                    _ => return Ok(TokenKind::Not),
                }
            }
            '<' => {
                self.consume_char();
                if self.peek_char() == '=' {
                    self.consume_char();
                    Ok(TokenKind::Lte)
                } else {
                    Ok(TokenKind::Lt)
                }
            }
            '>' => {
                self.consume_char();
                if self.peek_char() == '=' {
                    self.consume_char();
                    Ok(TokenKind::Gte)
                } else {
                    Ok(TokenKind::Gt)
                }
            }
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            x if x.is_digit(10) => {
                return Ok(TokenKind::IntLiteral {
                    value: self.consume_number(),
                })
            }
            x if x.is_alphabetic() => {
                let ident = self.consume_ident();
                return match find_keyword(&ident) {
                    Some(token) => Ok(token),
                    None => Ok(TokenKind::Ident { name: ident }),
                };
            }
            x => {
                return Err(Error::new(
                    self.pos.clone(),
                    ErrorKind::UnexpectedChar { c: x },
                ))
            }
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
        self.source.content[self.source_index..]
            .chars()
            .next()
            .unwrap()
    }

    fn consume_char(&mut self) -> char {
        let mut iter = self.source.content[self.source_index..].char_indices();
        let (_, cur_char) = iter.next().unwrap();
        let (next_pos, _) = iter.next().unwrap_or((1, ' '));

        self.source_index += next_pos;
        self.pos.column = 1;
        if cur_char == '\n' {
            self.pos.line += 1;
            self.pos.column = 1;
        }

        cur_char
    }

    fn is_eof(&self) -> bool {
        self.source_index >= self.source.content.len()
    }
}

fn find_keyword(ident: &str) -> Option<TokenKind> {
    match ident {
        "func" => Some(TokenKind::Func),
        "var" => Some(TokenKind::Var),
        "val" => Some(TokenKind::Val),
        "return" => Some(TokenKind::Return),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "false" => Some(TokenKind::False),
        "true" => Some(TokenKind::True),
        "while" => Some(TokenKind::While),
        _ => None,
    }
}

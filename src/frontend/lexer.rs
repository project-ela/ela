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
            self.consume_whitespace();

            let pos = self.pos.clone();
            tokens.push(Token {
                kind: self.next_token()?,
                pos,
            });
        }

        tokens.push(Token {
            kind: TokenKind::EOF,
            pos: self.pos.clone(),
        });

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<TokenKind, Error> {
        if self.is_eof() {
            return Ok(TokenKind::EOF);
        }

        match self.peek_char() {
            x if x.is_digit(10) => Ok(self.consume_number()),
            x if x.is_alphabetic() => Ok(find_keyword(self.consume_ident())),
            x => self.consume_symbol(x),
        }
    }

    fn consume_symbol(&mut self, c: char) -> Result<TokenKind, Error> {
        self.consume_char();

        match c {
            '+' => Ok(TokenKind::Plus),
            '-' => Ok(TokenKind::Minus),
            '*' => Ok(TokenKind::Asterisk),
            '&' => Ok(TokenKind::And),
            '|' => Ok(TokenKind::Or),
            '^' => Ok(TokenKind::Xor),
            ':' => Ok(TokenKind::Colon),
            '(' => Ok(TokenKind::LParen),
            ')' => Ok(TokenKind::RParen),
            '{' => Ok(TokenKind::LBrace),
            '}' => Ok(TokenKind::RBrace),
            ',' => Ok(TokenKind::Comma),
            '/' => match self.peek_char() {
                '/' => {
                    self.consume_char();
                    Ok(self.consume_line_comment())
                }
                '*' => {
                    self.consume_char();
                    Ok(self.consume_block_comment())
                }
                _ => Ok(TokenKind::Slash),
            },
            '=' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Ok(TokenKind::Equal)
                }
                _ => Ok(TokenKind::Assign),
            },
            '!' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Ok(TokenKind::NotEqual)
                }
                _ => Ok(TokenKind::Not),
            },
            '<' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Ok(TokenKind::Lte)
                }
                _ => Ok(TokenKind::Lt),
            },
            '>' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Ok(TokenKind::Gte)
                }
                _ => Ok(TokenKind::Gt),
            },
            x => Err(Error::new(
                self.pos.clone(),
                ErrorKind::UnexpectedChar { c: x },
            )),
        }
    }

    fn consume_ident(&mut self) -> TokenKind {
        let mut name = String::new();
        while !self.is_eof() && self.peek_char().is_alphabetic() || self.peek_char().is_digit(10) {
            name.push(self.consume_char());
        }

        TokenKind::Ident { name }
    }

    fn consume_number(&mut self) -> TokenKind {
        let mut digits = String::new();
        while !self.is_eof() && self.peek_char().is_digit(10) {
            digits.push(self.consume_char());
        }

        let value = digits.parse().unwrap();
        TokenKind::IntLiteral { value }
    }

    fn consume_whitespace(&mut self) {
        while !self.is_eof() && self.peek_char().is_whitespace() {
            self.consume_char();
        }
    }

    fn consume_line_comment(&mut self) -> TokenKind {
        let mut content = String::new();
        while !self.is_eof() && self.peek_char() != '\n' {
            content.push(self.consume_char());
        }

        TokenKind::Comment { content }
    }

    fn consume_block_comment(&mut self) -> TokenKind {
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

        TokenKind::Comment { content }
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
        self.pos.column += 1;
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

fn find_keyword(ident: TokenKind) -> TokenKind {
    if let TokenKind::Ident { name } = &ident {
        match name.as_str() {
            "func" => TokenKind::Func,
            "var" => TokenKind::Var,
            "val" => TokenKind::Val,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "true" => TokenKind::True,
            "while" => TokenKind::While,
            _ => ident,
        }
    } else {
        ident
    }
}

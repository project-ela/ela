pub mod error;

use crate::{
    common::{error::Error, pos::Pos},
    frontend::token::{Keyword, Symbol, Token, TokenKind},
};
use anyhow::Result;

use self::error::LexerError;

struct Tokenizer {
    source: SourceFile,
    source_index: usize,

    pos: Pos,
}

pub struct SourceFile {
    pub filename: String,
    pub content: String,
}

pub fn tokenize(source: SourceFile) -> Result<Vec<Token>> {
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

    fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        self.consume_whitespace();
        while !self.is_eof() {
            tokens.push(self.next_token()?);
            self.consume_whitespace();
        }

        tokens.push(Token {
            kind: TokenKind::EOF,
            pos: self.pos.clone(),
        });

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token> {
        let pos = self.pos.clone();
        let kind = match self.peek_char() {
            '\'' => self.consume_char_literal()?,
            '"' => self.consume_string_literal()?,
            x if x.is_digit(10) => self.consume_number(),
            x if is_ident(x) => find_keyword(self.consume_ident()),
            _ => self.consume_symbol()?,
        };

        Ok(Token { kind, pos })
    }

    fn consume_symbol(&mut self) -> Result<TokenKind> {
        let symbol = match self.consume_char() {
            '+' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::PlusAssign
                }
                _ => Symbol::Plus,
            },
            '-' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::MinusAssign
                }
                _ => Symbol::Minus,
            },
            '*' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::AsteriskAssign
                }
                _ => Symbol::Asterisk,
            },
            '%' => Symbol::Percent,
            '&' => Symbol::And,
            '|' => Symbol::Or,
            '^' => Symbol::Xor,
            ':' => Symbol::Colon,
            '(' => Symbol::LParen,
            ')' => Symbol::RParen,
            '{' => Symbol::LBrace,
            '}' => Symbol::RBrace,
            '[' => Symbol::LBracket,
            ']' => Symbol::RBracket,
            ',' => Symbol::Comma,
            '/' => match self.peek_char() {
                '/' => {
                    self.consume_char();
                    return Ok(self.consume_line_comment());
                }
                '*' => {
                    self.consume_char();
                    return Ok(self.consume_block_comment());
                }
                '=' => {
                    self.consume_char();
                    Symbol::SlashAssign
                }
                _ => Symbol::Slash,
            },
            '=' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::Equal
                }
                _ => Symbol::Assign,
            },
            '!' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::NotEqual
                }
                _ => Symbol::Not,
            },
            '<' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::Lte
                }
                _ => Symbol::Lt,
            },
            '>' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    Symbol::Gte
                }
                _ => Symbol::Gt,
            },
            x => return Err(Error::new(self.pos.clone(), LexerError::UnexpectedChar(x)).into()),
        };

        Ok(TokenKind::Symbol(symbol))
    }

    fn consume_ident(&mut self) -> TokenKind {
        let mut name = String::new();
        while !self.is_eof() && is_ident(self.peek_char()) || self.peek_char().is_digit(10) {
            name.push(self.consume_char());
        }

        TokenKind::Ident(name)
    }

    fn consume_char_literal(&mut self) -> Result<TokenKind> {
        self.consume_char();
        let value = match self.consume_char() {
            '\\' => self.consume_escape_char(),
            x => x,
        };

        match self.consume_char() {
            '\'' => Ok(TokenKind::Char(value)),
            x => Err(Error::new(self.pos.clone(), LexerError::UnexpectedChar(x)).into()),
        }
    }

    fn consume_string_literal(&mut self) -> Result<TokenKind> {
        self.consume_char();

        let mut value = String::new();
        while !self.is_eof() && self.peek_char() != '"' {
            let c = match self.consume_char() {
                '\\' => self.consume_escape_char(),
                x => x,
            };
            value.push(c);
        }
        self.consume_char();

        Ok(TokenKind::String(value))
    }

    fn consume_escape_char(&mut self) -> char {
        match self.consume_char() {
            'n' => '\n',
            'r' => '\r',
            't' => '\t',
            '\\' => '\\',
            '\'' => '\'',
            '"' => '"',
            '0' => '\0',
            x => x,
        }
    }

    fn consume_number(&mut self) -> TokenKind {
        let mut radix = 10;
        if self.peek_char() == '0' {
            self.consume_char();
            match self.peek_char() {
                'x' => {
                    self.consume_char();
                    radix = 16;
                }
                _ => return TokenKind::Integer(0),
            }
        }

        let mut digits = String::new();
        while !self.is_eof() && self.peek_char().is_digit(radix) {
            digits.push(self.consume_char());
        }

        let value = i32::from_str_radix(&digits, radix).unwrap();
        TokenKind::Integer(value)
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

        TokenKind::Comment(content)
    }

    fn consume_block_comment(&mut self) -> TokenKind {
        let mut content = String::new();
        while !self.is_eof() {
            match self.consume_char() {
                '*' => match self.peek_char() {
                    '/' => {
                        self.consume_char();
                        break;
                    }
                    _ => {
                        content.push('*');
                    }
                },
                x => content.push(x),
            }
        }

        TokenKind::Comment(content)
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

fn is_ident(c: char) -> bool {
    match c {
        '_' => true,
        x if x.is_alphabetic() => true,
        _ => false,
    }
}

fn find_keyword(ident: TokenKind) -> TokenKind {
    let name = match ident {
        TokenKind::Ident(ref name) => name,
        x => return x,
    };

    let keyword = match name.as_str() {
        "else" => Keyword::Else,
        "false" => Keyword::False,
        "func" => Keyword::Func,
        "if" => Keyword::If,
        "return" => Keyword::Return,
        "true" => Keyword::True,
        "val" => Keyword::Val,
        "var" => Keyword::Var,
        "while" => Keyword::While,
        _ => return ident,
    };

    TokenKind::Keyword(keyword)
}

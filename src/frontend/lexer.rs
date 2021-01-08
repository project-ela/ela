pub mod token;

use x86asm::instruction::{mnemonic::Mnemonic, operand::register::Register};

use crate::{
    common::{
        error::{Error, ErrorKind},
        pos::Pos,
    },
    frontend::lexer::token::{Symbol, Token, TokenKind},
};

use self::token::Keyword;

struct Lexer {
    source: SourceFile,
    source_index: usize,

    pos: Pos,
}

pub struct SourceFile {
    pub filename: String,
    pub content: String,
}

pub fn tokenize(source: SourceFile) -> Result<Vec<Token>, Error> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

impl Lexer {
    fn new(source: SourceFile) -> Self {
        let pos = Pos {
            filename: source.filename.clone(),
            line: 1,
            column: 1,
        };

        Self {
            source,
            source_index: 0,
            pos,
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();

        while !self.is_eof() {
            self.consume_whitespace();
            tokens.push(self.next_token()?);
        }

        tokens.push(Token {
            kind: TokenKind::EOF,
            pos: self.pos.clone(),
        });

        Ok(tokens)
    }

    fn next_token(&mut self) -> Result<Token, Error> {
        if self.is_eof() {
            return Ok(Token {
                kind: TokenKind::EOF,
                pos: self.pos.clone(),
            });
        }

        let pos = self.pos.clone();
        let kind = match self.peek_char() {
            x if x.is_digit(10) => self.consume_number(),
            x if is_ident(x) => find_keyword(self.consume_ident()),
            _ => self.consume_symbol()?,
        };

        Ok(Token { kind, pos })
    }

    fn consume_number(&mut self) -> TokenKind {
        let mut result = String::new();
        while !self.is_eof() && self.peek_char().is_digit(10) {
            result.push(self.consume_char());
        }

        let value = result.parse().unwrap();
        TokenKind::Integer(value)
    }

    fn consume_ident(&mut self) -> TokenKind {
        let mut name = String::new();
        while !self.is_eof() && (is_ident(self.peek_char()) || self.peek_char().is_digit(10)) {
            name.push(self.consume_char());
        }

        TokenKind::Ident(name)
    }

    fn consume_symbol(&mut self) -> Result<TokenKind, Error> {
        let symbol = match self.consume_char() {
            ',' => Symbol::Comma,
            ':' => Symbol::Colon,
            '[' => Symbol::LBracket,
            ']' => Symbol::RBracket,
            '+' => Symbol::Plus,
            '-' => Symbol::Minus,
            ';' => {
                self.consume_char();
                return Ok(self.consume_comment());
            }
            x => {
                return Err(Error::new(
                    self.pos.clone(),
                    ErrorKind::UnexpectedChar { actual: x },
                ))
            }
        };

        Ok(TokenKind::Symbol(symbol))
    }

    fn consume_comment(&mut self) -> TokenKind {
        let mut content = String::new();
        while !self.is_eof() && self.peek_char() != '\n' {
            content.push(self.consume_char());
        }

        TokenKind::Comment(content)
    }

    fn consume_whitespace(&mut self) {
        while !self.is_eof() && self.peek_char().is_whitespace() {
            self.consume_char();
        }
    }

    fn peek_char(&self) -> char {
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
        '.' | '_' => true,
        x if x.is_alphabetic() => true,
        _ => false,
    }
}

fn find_keyword(ident: TokenKind) -> TokenKind {
    let name = match ident {
        TokenKind::Ident(ref name) => name,
        x => return x,
    };

    match name.as_str() {
        "byte" => TokenKind::Keyword(Keyword::Byte),
        "ptr" => TokenKind::Keyword(Keyword::Ptr),

        "add" => TokenKind::Mnemonic(Mnemonic::Add),
        "and" => TokenKind::Mnemonic(Mnemonic::And),
        "call" => TokenKind::Mnemonic(Mnemonic::Call),
        "cmp" => TokenKind::Mnemonic(Mnemonic::Cmp),
        "hlt" => TokenKind::Mnemonic(Mnemonic::Hlt),
        "idiv" => TokenKind::Mnemonic(Mnemonic::IDiv),
        "imul" => TokenKind::Mnemonic(Mnemonic::IMul),
        "je" => TokenKind::Mnemonic(Mnemonic::Je),
        "jmp" => TokenKind::Mnemonic(Mnemonic::Jmp),
        "lea" => TokenKind::Mnemonic(Mnemonic::Lea),
        "mov" => TokenKind::Mnemonic(Mnemonic::Mov),
        "movsx" => TokenKind::Mnemonic(Mnemonic::Movsx),
        "or" => TokenKind::Mnemonic(Mnemonic::Or),
        "pop" => TokenKind::Mnemonic(Mnemonic::Pop),
        "push" => TokenKind::Mnemonic(Mnemonic::Push),
        "ret" => TokenKind::Mnemonic(Mnemonic::Ret),
        "sete" => TokenKind::Mnemonic(Mnemonic::Sete),
        "setg" => TokenKind::Mnemonic(Mnemonic::Setg),
        "setge" => TokenKind::Mnemonic(Mnemonic::Setge),
        "setl" => TokenKind::Mnemonic(Mnemonic::Setl),
        "setle" => TokenKind::Mnemonic(Mnemonic::Setle),
        "setne" => TokenKind::Mnemonic(Mnemonic::Setne),
        "sub" => TokenKind::Mnemonic(Mnemonic::Sub),
        "syscall" => TokenKind::Mnemonic(Mnemonic::Syscall),
        "xor" => TokenKind::Mnemonic(Mnemonic::Xor),

        "rax" => TokenKind::Register(Register::Rax),
        "rcx" => TokenKind::Register(Register::Rcx),
        "rdx" => TokenKind::Register(Register::Rdx),
        "rbx" => TokenKind::Register(Register::Rbx),
        "rsp" => TokenKind::Register(Register::Rsp),
        "rbp" => TokenKind::Register(Register::Rbp),
        "rsi" => TokenKind::Register(Register::Rsi),
        "rdi" => TokenKind::Register(Register::Rdi),
        "r8" => TokenKind::Register(Register::R8),
        "r9" => TokenKind::Register(Register::R9),
        "r10" => TokenKind::Register(Register::R10),
        "r11" => TokenKind::Register(Register::R11),
        "r12" => TokenKind::Register(Register::R12),
        "r13" => TokenKind::Register(Register::R13),
        "r14" => TokenKind::Register(Register::R14),
        "r15" => TokenKind::Register(Register::R15),

        "eax" => TokenKind::Register(Register::Eax),
        "ecx" => TokenKind::Register(Register::Ecx),
        "edx" => TokenKind::Register(Register::Edx),
        "ebx" => TokenKind::Register(Register::Ebx),
        "esp" => TokenKind::Register(Register::Esp),
        "ebp" => TokenKind::Register(Register::Ebp),
        "esi" => TokenKind::Register(Register::Esi),
        "edi" => TokenKind::Register(Register::Edi),

        "al" => TokenKind::Register(Register::Al),
        "cl" => TokenKind::Register(Register::Cl),
        "dl" => TokenKind::Register(Register::Dl),
        "bl" => TokenKind::Register(Register::Bl),
        "sil" => TokenKind::Register(Register::Sil),
        "dil" => TokenKind::Register(Register::Dil),
        "spl" => TokenKind::Register(Register::Spl),
        "bpl" => TokenKind::Register(Register::Bpl),
        "r8b" => TokenKind::Register(Register::R8b),
        "r9b" => TokenKind::Register(Register::R9b),
        "r10b" => TokenKind::Register(Register::R10b),
        "r11b" => TokenKind::Register(Register::R11b),
        "r12b" => TokenKind::Register(Register::R12b),
        "r13b" => TokenKind::Register(Register::R13b),
        "r14b" => TokenKind::Register(Register::R14b),
        "r15b" => TokenKind::Register(Register::R15b),

        _ => ident,
    }
}

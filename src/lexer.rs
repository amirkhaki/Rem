use phf::phf_map;
use std::{
    fs::File,
    io::{self, BufRead, BufReader, Error, ErrorKind, Read},
};
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Float {
    integral: u64,
    fractional: u64,
}
#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub enum Token {
    EOF,
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Semicolon,
    While,
    Minus,
    Plus,
    Return,
    Identifier(String),
    DoubleColon,
    Colon,
    Function,
    Integer(i64),
    Float(Float),
    String(String),
    Div,
    Directive(String),
    Comma,
    Hash,
    LessThan,
    ShiftLeft,
    LessThanEqual,
    BitOr,
    Or,
    Not,
    NotEqual,
    BitAnd,
    And,
    Star,
    Arrow,
    Equal,
    Assign,
    LeftBracket,
    RightBracket,
    RightLeft,
    GreaterThan,
    GreaterThanEqual,
    Question,
    BitNot,
    Mod,
    Dot,
    Xor,
    If,
    Else,
    Continue,
    Struct,
    Switch,
    Typedef,
    Case,
    Auto,
    Break,
    Enum,
    Register,
    Extern,
    Union,
    Const,
    For,
    Default,
    Goto,
    Volatile,
    Do,
    Static,
    Character(String),
}

#[derive(Debug)]
pub struct Lexer {
    f: BufReader<File>,
}

use Token as tk;
impl Lexer {
    const KEYWORDS: phf::Map<&'static str, Token> = phf_map! {
        "return" => Token::Return,
        "if" => Token::If,
        "else" => Token::Else,
        "continue" => Token::Continue,
        "break" => Token::Break,
        "struct" => Token::Struct,
        "typedef" => Token::Typedef,
        "switch" => Token::Switch,
        "case" => Token::Case,
        "auto" => Token::Auto,
        "enum" => Token::Enum,
        "register" => Token::Register,
        "extern" => Token::Extern,
        "union" => Token::Union,
        "const" => Token::Const,
        "for" => Token::For,
        "default" => Token::Default,
        "goto" => Token::Goto,
        "volatile" => Token::Volatile,
        "do" => Token::Do,
        "static" => Token::Static,
        "while" => Token::While,
    };
    pub fn from(f: File) -> Self {
        Lexer {
            f: BufReader::new(f),
        }
    }
    fn getch(&mut self) -> Option<u8> {
        let mut ch: [u8; 1] = [0];
        if self.eof().ok()? {
            return None;
        }
        if let Err(e) = self.f.read_exact(&mut ch) {
            eprintln!("error {e}");
            return None;
        }
        Some(ch[0])
    }
    fn eof(&mut self) -> io::Result<bool> {
        Ok(self.f.fill_buf()?.is_empty())
    }
    fn peek(&mut self) -> io::Result<u8> {
        let buf = self.f.fill_buf()?;
        if buf.is_empty() {
            return Err(Error::from(ErrorKind::UnexpectedEof));
        }
        return Ok(buf[0]);
    }
    fn take_while<P>(&mut self, mut initial: String, mut predicate: P) -> Option<String>
    where
        P: FnMut(&&u8) -> bool,
    {
        loop {
            let mut len = 0;
            let Ok(buf) = self.f.fill_buf() else {
                return None;
            };
            let buf_len = buf.len();
            for &ch in buf.into_iter().take_while(&mut predicate) {
                initial.push(ch as char);
                len += 1;
            }
            self.f.consume(len);
            if len != buf_len {
                break;
            }
        }
        Some(initial)
    }

    fn string(&mut self, ch: u8) -> Option<Token> {
        let string = (ch as char).to_string();
        let mut found_end = false;
        let string = self.take_while(string, |&&ch| {
            if found_end {
                return false;
            }
            if ch == b'"' {
                found_end = true;
            }
            true
        })?;
        Some(tk::String(string))
    }

    fn greater_than(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::GreaterThan),
            Some(next) => match next {
                b'>' => {
                    self.f.consume(1);
                    Some(tk::RightLeft)
                }
                b'=' => {
                    self.f.consume(1);
                    Some(tk::GreaterThanEqual)
                }
                _ => Some(tk::GreaterThan),
            },
        }
    }

    fn less_than(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::LessThan),
            Some(next) => match next {
                b'<' => {
                    self.f.consume(1);
                    Some(tk::ShiftLeft)
                }
                b'=' => {
                    self.f.consume(1);
                    Some(tk::LessThanEqual)
                }
                _ => Some(tk::LessThan),
            },
        }
    }

    fn not(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::Not),
            Some(next) => match next {
                b'=' => {
                    self.f.consume(1);
                    Some(tk::NotEqual)
                }
                _ => Some(tk::Not),
            },
        }
    }

    fn and(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::BitAnd),
            Some(next) => match next {
                b'&' => {
                    self.f.consume(1);
                    Some(tk::And)
                }
                _ => Some(tk::BitAnd),
            },
        }
    }

    fn minus(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::Minus),
            Some(next) => match next {
                b'>' => {
                    self.f.consume(1);
                    Some(tk::Arrow)
                }
                _ => Some(tk::Minus),
            },
        }
    }

    fn equal(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::Assign),
            Some(next) => match next {
                b'=' => {
                    self.f.consume(1);
                    Some(tk::Equal)
                }
                _ => Some(tk::Assign),
            },
        }
    }

    fn or(&mut self) -> Option<Token> {
        let next = self.peek().ok();
        match next {
            None => Some(tk::BitOr),
            Some(next) => match next {
                b'|' => {
                    self.f.consume(1);
                    Some(tk::Or)
                }
                _ => Some(tk::BitOr),
            },
        }
    }

    fn hash(&mut self, ch: u8) -> Option<Token> {
        let directive = (ch as char).to_string();
        let ident = self.take_while(directive, |&&ch| {
            (b'A'..=b'Z').contains(&ch)
                || (b'a'..=b'z').contains(&ch)
                || (b'0'..=b'9').contains(&ch)
                || ch == b'_'
        })?;
        Some(tk::Directive(ident))
    }

    fn slash(&mut self) -> Option<Token> {
        let next = self.peek().ok()?;
        match next {
            b'/' => {
                let comment = String::new();
                self.take_while(comment, |&&ch| ch != b'\n')?;
                self.next()
            }
            b'*' => {
                self.f.consume(1);
                let comment = String::new();
                enum CommentState {
                    Looping,
                    Star,
                    Done,
                }
                let mut state = CommentState::Looping;
                self.take_while(comment, |&&ch| match ch {
                    b'*' => match state {
                        CommentState::Looping => {
                            state = CommentState::Star;
                            true
                        }
                        CommentState::Star => true,
                        CommentState::Done => false,
                    },
                    b'/' => match state {
                        CommentState::Looping => true,
                        CommentState::Star => {
                            state = CommentState::Done;
                            true
                        }
                        CommentState::Done => false,
                    },
                    _ => match state {
                        CommentState::Looping => true,
                        CommentState::Star => {
                            state = CommentState::Looping;
                            true
                        }
                        CommentState::Done => false,
                    },
                })?;
                self.next()
            }
            _ => Some(tk::Div),
        }
    }

    fn identifier(&mut self, ch: u8) -> Option<Token> {
        let ident = (ch as char).to_string();
        let ident = self.take_while(ident, |&&ch| {
            (b'A'..=b'Z').contains(&ch)
                || (b'a'..=b'z').contains(&ch)
                || (b'0'..=b'9').contains(&ch)
                || ch == b'_'
        })?;
        Some(
            Lexer::KEYWORDS
                .get(&ident)
                .cloned()
                .unwrap_or(tk::Identifier(ident)),
        )
    }

    fn number(&mut self, ch: u8) -> Option<Token> {
        let number = (ch as char).to_string();
        let mut is_float = false;
        let number = self.take_while(number, |&&ch| {
            if !is_float && ch == b'.' {
                is_float = true;
                return true;
            }
            (b'0'..=b'9').contains(&ch)
        })?;
        self.f.consume(number.len() - 1);
        if is_float {
            let mut parts = number.split('.');
            let integral = parts.next().unwrap();
            let fraction = parts.next().unwrap();
            Some(tk::Float(Float {
                integral: integral.parse().unwrap_or(0),
                fractional: fraction.parse().unwrap_or(0),
            }))
        } else {
            Some(tk::Integer(number.parse().unwrap()))
        }
    }

    fn character(&mut self, ch: u8) -> Option<Token> {
        let string = (ch as char).to_string();
        let mut found_end = false;
        let string = self.take_while(string, |&&ch| {
            if found_end {
                return false;
            }
            if ch == b'\'' {
                found_end = true;
            }
            true
        })?;
        Some(tk::Character(string))
    }
}
impl Iterator for Lexer {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        let ch = self.getch()?;
        match ch {
            b'\'' => self.character(ch),
            b'"' => self.string(ch),
            b'*' => Some(tk::Star),
            b'>' => self.greater_than(),
            b'<' => self.less_than(),

            b'!' => self.not(),
            b'&' => self.and(),
            b'-' => self.minus(),
            b'=' => self.equal(),
            b'|' => self.or(),
            b'#' => self.hash(ch),
            b',' => Some(tk::Comma),
            b'?' => Some(tk::Question),
            b'~' => Some(tk::BitNot),
            b'%' => Some(tk::Mod),
            b'.' => Some(tk::Dot),
            b'^' => Some(tk::Xor),
            b':' => Some(tk::Colon), //TODO: DoubleColon
            b'/' => self.slash(),
            b'+' => Some(tk::Plus),
            b'(' => Some(tk::LeftParen),
            b')' => Some(tk::RightParen),
            b'}' => Some(tk::RightBrace),
            b'{' => Some(tk::LeftBrace),
            b'[' => Some(tk::LeftBracket),
            b']' => Some(tk::RightBracket),
            b';' => Some(tk::Semicolon),
            b'A'..=b'Z' | b'a'..=b'z' | b'_' => self.identifier(ch),
            b'0'..=b'9' => self.number(ch),
            //TODO: properly handle \
            b' ' | b'\n' | b'\t' | 0xC | b'\\' => self.next(), // ignore whitespacs
            _ => {
                eprintln!("char is \"{}\" with code {}", ch as char, ch);
                None
            }
        }
    }
}

use std::fmt::Display;
use crate::prelude::{Result, FhirError};

#[derive(Debug, PartialEq, Eq)]
pub enum Comparator{
    Eq,
    Gt,
    Ge,
    Lt,
    Le,
    Ne,
}

impl Display for Comparator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Comparator::Eq => write!(f, "="),
            Comparator::Gt => write!(f, ">="),
            Comparator::Ge => write!(f, ">"),
            Comparator::Lt => write!(f, "<="),
            Comparator::Le => write!(f, "<"),
            Comparator::Ne => write!(f, "!="),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    WhiteSpace,
    Dot,
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    Symbol(String),
    Text(String),
    DateTime(String),
    Number(String),
    Operator(u8),
    Comparator(Comparator),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub pos:usize,
    pub token_type: TokenType,
}

pub struct Tokenizer<'l>{
    input: &'l [u8],
    index: usize,
}

impl<'l> Tokenizer<'l> {
    
    pub fn new(input: &'l str) -> Self {

        Self {
            input: input.as_bytes(),
            index: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.next_char() {
                Some(ch) => {
                    let token = match ch {
                        b' ' | b'\n' | b'\t' | b'\r' => Token{ pos: self.index, token_type: TokenType::WhiteSpace },
                        b'[' => Token{ pos: self.index, token_type: TokenType::OpenBracket },
                        b']' => Token{ pos: self.index, token_type: TokenType::CloseBracket },
                        b'(' => Token{ pos: self.index, token_type: TokenType::OpenParen },
                        b')' => Token{ pos: self.index, token_type: TokenType::CloseParen },
                        b'.' => Token{ pos: self.index, token_type: TokenType::Dot },
                        b'+' | b'-' | b'*' | b'/' => Token{ pos: self.index, token_type: TokenType::Operator(ch) },
                        b'=' | b'!' | b'>' | b'<' | b'~' => self.parse_equality(ch)?,
                        b'@' => self.parse_datetime(ch)?,
                        b'\"' | b'\'' => self.parse_text(ch)?,
                        b'0'..=b'9' => self.parse_number(ch)?,
                        b'a'..=b'z' | b'A'..=b'Z' => self.parse_symbol(ch)?,
                        _ => return Err(FhirError::Message(format!("表达式中出现了非法的字符[{}]", &ch)))
                    };
                    tokens.push(token);
                }
                None => break,
            }
        }

        Ok(tokens)
    }

    fn next_char(&mut self) -> Option<u8> {
        if self.index < self.input.len() {
            let ch = self.input[self.index];
            self.index += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn eat_char(&mut self) {
        self.index += 1;
    }

    fn peek(&mut self) -> Option<u8> {
        if self.index < self.input.len() {
            Some(self.input[self.index])
        } else {
            None
        }
    }

    // fn parse_escape(&self) {

    // }

    fn parse_datetime(&mut self, _ch: u8) -> Result<Token> {
        Ok(Token{ pos: self.index, token_type: TokenType::Dot })
    }

    fn parse_text(&mut self, _ch: u8) -> Result<Token> {
        let mut scratch = vec![];

        loop {
            match self.peek() {
                Some(ch) => {
                    match ch {
                        b'\'' | b'\"' => {
                            self.eat_char();
                            break;
                        },
                        _ => {
                            scratch.push(ch);
                            self.eat_char();
                        },
                    }
                }
                None => break,
            }
        }

        match String::from_utf8(scratch) {
            Ok(text) => Ok(Token{ pos: self.index, token_type: TokenType::Text(text) }),
            Err(_) => Err(FhirError::error("不是有效的UTF8字符")),
        }
    }

    fn parse_equality(&mut self, ch: u8) -> Result<Token> {
        match ch {
            b'=' | b'~' => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Eq) }),
            b'!' => {
                match self.peek() {
                    Some(next) => {
                        match next {
                            b'=' | b'~' => {
                                self.eat_char();
                                Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Ne) })
                            },
                            _ => Err(FhirError::error("表达式中存在无效的操作符[!]")),
                        }
                    }
                    None => Err(FhirError::error("表达式中存在无效的操作符[!]")),
                }
            },
            b'>' => {
                match self.peek() {
                    Some(next) => {
                        match next {
                            b'=' => {
                                self.eat_char();
                                Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Gt) })
                            },
                            _ => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Ge) }),
                        }
                    }
                    None => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Ge) }),
                }
            }
            b'<' => {
                match self.peek() {
                    Some(next) => {
                        match next {
                            b'=' => {
                                self.eat_char();
                                Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Lt) })
                            },
                            _ => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Le) }),
                        }
                    }
                    None => Ok(Token{ pos: self.index, token_type: TokenType::Comparator(Comparator::Le) }),
                }
            }
            _ => unreachable!()
        }
    }

    fn parse_number(&mut self, ch: u8) -> Result<Token> {
        let mut scratch = vec![ch];

        loop {
            match self.peek() {
                Some(ch) => {
                    match ch {
                        b'0'..=b'9' | b'.' => {
                            scratch.push(ch);
                            self.eat_char();
                        },
                        _ => break,
                    }
                }
                None => break,
            }
        }

        match String::from_utf8(scratch) {
            Ok(number) => Ok(Token{ pos: self.index, token_type: TokenType::Number(number) }),
            Err(_) => Err(FhirError::error("不是有效的UTF8字符")),
        }
    }

    fn parse_symbol(&mut self, ch: u8) -> Result<Token> {
        let mut scratch = vec![ch];
        loop {
            match self.peek() {
                Some(ch) => {
                    match ch {
                        b'a'..=b'z' | b'A'..=b'Z' => {
                            scratch.push(ch);
                            self.eat_char();
                        },
                        _ => break,
                    }
                }
                None => break,
            }
        }
        
        match String::from_utf8(scratch) {
            Ok(symbol) => Ok(Token{ pos: self.index, token_type: TokenType::Symbol(symbol) }),
            Err(_) => Err(FhirError::error("不是有效的UTF8字符")),
        }
    }
}

#[test]
pub fn test_tokenizer() {
    let mut tokenizer = Tokenizer::new("Patient.name[0].given[2] >= 'abc.fgh>10'");

    let list = tokenizer.tokenize().unwrap();

    assert_eq!(list[0].token_type, TokenType::Symbol("Patient".to_string()));
    assert_eq!(list[1].token_type, TokenType::Dot);
    assert_eq!(list[2].token_type, TokenType::Symbol("name".to_string()));
    assert_eq!(list[3].token_type, TokenType::OpenBracket);
    assert_eq!(list[4].token_type, TokenType::Number("0".to_string()));
    assert_eq!(list[5].token_type, TokenType::CloseBracket);
    assert_eq!(list[6].token_type, TokenType::Dot);
    assert_eq!(list[7].token_type, TokenType::Symbol("given".to_string()));
    assert_eq!(list[8].token_type, TokenType::OpenBracket);
    assert_eq!(list[9].token_type, TokenType::Number("2".to_string()));
    assert_eq!(list[10].token_type, TokenType::CloseBracket);
    assert_eq!(list[11].token_type, TokenType::WhiteSpace);
    assert_eq!(list[12].token_type, TokenType::Comparator(Comparator::Gt));
    assert_eq!(list[13].token_type, TokenType::WhiteSpace);
    assert_eq!(list[14].token_type, TokenType::Text("abc.fgh>10".into()));
}

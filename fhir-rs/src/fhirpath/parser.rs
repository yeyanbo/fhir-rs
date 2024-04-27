use std::cell::Cell;
use std::str::FromStr;

use crate::prelude::{Result, FhirError};
use super::*;

pub struct Parser{
    tokens: Vec<Token>,
    current: Cell<usize>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: Cell::new(0),
        }
    }

    pub fn next(&self) -> (Option<&Token>, Option<&Token>) {
        let current = self.current.get() + 1;
        self.current.set(current);
        (self.tokens.get(current - 1), self.tokens.get(current))
    }

    pub fn eat(&self) {
        self.current.set(self.current.get() + 1)
    }

    /// parse tokens to fhirpath expression
    /// 
    /// This is a recursive funtion
    /// 1. Parse whole tokens
    /// 2. Parse tokens in parentheses
    /// 3. Parse tokens for function params
    pub fn parse(&self) -> Result<PathExpression> {
        let mut component = Vec::new();

        // conditon of stop loop：
        // 1. until end of tokens
        // 2. until to close parentheses
        while let (Some(first), second) = self.next() {
            match &first.token_type {
                TokenType::CloseParen => {
                    self.eat();
                    break
                },
                TokenType::Dot => println!("dot"),
                TokenType::Text(text) => component.push(PathComponent::LiteralString(text.clone())),
                TokenType::DateTime(text) => component.push(PathComponent::LiteralDatetime(text.clone())),
                TokenType::Number(text) => component.push(PathComponent::LiteralNumber(text.clone())),
                TokenType::OpenParen => {
                    self.eat();
                    let expr = self.parse()?;
                    component.push(PathComponent::Expression(expr));
                },
                TokenType::Symbol(symbol) => {
                    match second {
                        Some(sec) => {
                            match sec.token_type {
                                TokenType::OpenParen => {
                                    self.eat();
                                    let expr = self.parse()?;
                                    component.push(PathComponent::Function(Function{symbol: symbol.try_into()?, params: expr,}));
                                },
                                TokenType::OpenBracket => {
                                    self.eat();
                                    let comp = self.parse_path_component(symbol)?;
                                    component.push(comp);
                                },
                                _ => component.push(PathComponent::Path(Path::new(symbol.clone()))),
                            }
                        },
                        None => component.push(PathComponent::Path(Path::new(symbol.clone()))),
                    }
                },
                TokenType::Operator(op) => {
                    match second {
                        Some(sec) => {
                            match sec.token_type {
                                TokenType::WhiteSpace => {
                                    // TODO: 根据不同的运算符生成不同的函数
                                },
                                _ => return Err(FhirError::Message(format!("运算符[{op}]右侧应有空白字符作为间隔"))),
                            }
                        },
                        None => return Err(FhirError::Message(format!("运算符[{op}]缺少右侧表达式"))),
                    }
                },
                TokenType::Comparator(op) => {
                    match second {
                        Some(sec) => {
                            match sec.token_type {
                                TokenType::WhiteSpace => {
                                    // TODO: 根据不同的运算符生成不同的函数
                                },
                                _ => return Err(FhirError::Message(format!("比较运算符[{op}]右侧应有空白字符作为间隔"))),
                            }
                        },
                        None => return Err(FhirError::Message(format!("比较运算符[{op}]缺少右侧表达式"))),
                    }
                },
                _ => {},
            }
        }

        let root = match component.first() {
            Some(comp) => {
                match comp {
                    PathComponent::Path(path) => {
                        if Self::is_first_char_upper_case(&path.symbol) {
                            Root::Absolute(path.symbol.clone())
                        } else {
                            Root::Relative
                        }
                    },
                    PathComponent::Expression(expr) => expr.root.clone(),
                    _ => Root::None,
                }
            },
            None => todo!(),
        };

        Ok(PathExpression{
            component, 
            current: 0, 
            branch: 0,
            root,
        })
    }

    fn is_first_char_upper_case(input: &String) -> bool {
        input.chars().next().map_or(false, |c| c.is_uppercase())
    }

    /// 解析路径携带的索引信息
    /// 
    /// 只有[number]才是一个有效的索引信息
    fn parse_path_component(&self, symbol: &String) -> Result<PathComponent> {
        match self.next() {
            (Some(first), Some(second)) => {
                match (&first.token_type, &second.token_type) {
                    (TokenType::Number(number), TokenType::CloseBracket) => {
                        let num: usize = usize::from_str(number.as_str())?;
                        Ok(PathComponent::Path(Path::with_index(symbol.clone(), num)))
                    },
                    (_, _) => Err(FhirError::Message(format!("路径[{}]携带了无效的索引表示", &symbol))),
                }
            },
            _ => Err(FhirError::Message(format!("路径[{}]携带了无效的索引表示", &symbol))),
        }
    }
}
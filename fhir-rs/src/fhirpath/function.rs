use std::fmt::Debug;

use crate::prelude::{Result, FhirError};
use super::*;

#[derive(Debug)]
pub struct InvalidFunction(String);

impl From<InvalidFunction> for FhirError {
    fn from(value: InvalidFunction) -> Self {
        FhirError::Message(value.0)
    }
}

#[derive(Debug, Clone)]
pub enum FunctionParam {
    None,
    String(String),
    Integer(isize),
    Expression(PathExpression),
    Collection,
    Vec(Vec<FunctionParam>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionResponse {
    Collection,
    Bool,
    Number,
    String,
    DateTime,
}

/// 每个函数都要实现该特性
/// 想用每个函数自己来处理表达式的解析
pub trait Function2: Debug {
    fn parse(parser: &mut Parser) -> Result<Self> where Self: Sized;
    fn exec(&self, input: &Collection) -> Result<PathResponse>;
    fn response(&self) -> FunctionResponse;
}

#[derive(Debug)]
pub struct Empty;

impl Function2 for Empty {
    fn parse(parser: &mut Parser) -> Result<Self>  where Self: Sized {
        match parser.next() {
            (Some(first), _) => {
                match &first.token_type {
                    TokenType::CloseParen => Ok(Self),
                    _ => Err(FhirError::error("empty()函数参数应为空")),
                }
            },
            _ => Err(FhirError::error("没有发现函数的结束括号")),
        }
    }

    fn exec(&self, input: &Collection) -> Result<PathResponse> {
        let output = input.empty();
        Ok(PathResponse::Bool(output))
    }

    fn response(&self) -> FunctionResponse {
        FunctionResponse::Bool
    }
}



#[derive(Debug, Clone)]
pub struct Function {
    pub symbol: FunctionName,
    pub params: PathExpression,
}

impl Function {

    pub fn response(&self) -> FunctionResponse {
        FunctionResponse::Collection
    }

    // pub fn is_resource_type_element(&self) -> bool {
    //     match self.definition.function_name() {
    //         FunctionName::Element => {
    //             match &self.params {
    //                 FunctionParam::String(name) if Self::is_first_char_upper_case(name) => true,
    //                 _ => false,
    //             }
    //         },
    //         _ => false,
    //     }
    // }

    // fn is_first_char_upper_case(input: &String) -> bool {
    //     input.chars().next().map_or(false, |c| c.is_uppercase())
    // }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionName {
    Exist,
    Count,
    Empty,
    Single,
    Where,
    Other,
    AllTrue,
    Eq,
}

impl TryFrom<&String> for FunctionName {
    type Error = InvalidFunction;

    fn try_from(value: &String) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.as_str() {
            "empty" => Ok(FunctionName::Empty),
            "exists" => Ok(FunctionName::Exist),
            "count" => Ok(FunctionName::Count),
            "allTrue" => Ok(FunctionName::AllTrue),
            "where" => Ok(FunctionName::Where),
            other => Err(InvalidFunction(format!("无效的或者不支持的函数名称:[{}]", other))),
        }
    }
}
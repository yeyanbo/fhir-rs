use std::{iter::Peekable, vec::IntoIter};
use std::str::FromStr;
use super::*;
 
pub type TokenIter = Peekable<IntoIter<Token>>;

pub struct Parser;

impl Parser {
    pub fn parse_path(input: Vec<Token>) -> Result<Vec<Function>> {
        let mut iter = input.into_iter().peekable();
    
        let mut arr = Vec::new();
        while let Some(token) = iter.next() {
            println!("Main => {:?}", &token);
            match token.token_type {
                TokenType::WhiteSpace => {},
                TokenType::Dot => println!("dot"),
                TokenType::OpenParen => unreachable!(),
                TokenType::CloseParen => unreachable!(),
                TokenType::OpenBracket => {
                    let pt = Parser::parse_indexer(&mut iter)?;
                    arr.push(pt);
                },
                TokenType::CloseBracket => unreachable!(),
                TokenType::Symbol(symbol) => {
                    let func = match iter.peek() {
                        Some(next_token) => {
                            match next_token.token_type {
                                TokenType::OpenParen => Parser::parse_function(symbol, &mut iter)?,
                                _ => Function::from_definition(FunctionDefinition::ELEMENT, FunctionParam::String(symbol)),
                            }
                        },
                        None => Function::from_definition(FunctionDefinition::ELEMENT, FunctionParam::String(symbol)),
                    };
                    arr.push(func);
                },
                TokenType::Text(_) => println!("text"),
                TokenType::DateTime(_) => println!("datetime"),
                TokenType::Number(number) => println!("number: {}", number),
                TokenType::Operator(_) => println!("op"),
                TokenType::Comparator(_) => println!("comparator"),
            }
        };
    
        Ok(arr)
    }
    
    fn parse_function(symbol: String, iter: &mut TokenIter) -> Result<Function> {
        let definition =symbol.try_into()?;

        while let Some(token) = iter.next() {
            println!("Fuction => {:?}", &token);
            match token.token_type {
                TokenType::CloseParen => return Ok(Function { definition, params: FunctionParam::None }),
                _ => {}
            }
        };
    
        Err(FhirError::error("未发现代表结束的圆括号"))
    }
    
    fn parse_indexer(iter: &mut TokenIter) -> Result<Function> {
        let mut index: Option<isize> = None;
    
        while let Some(token) = iter.next() {
            println!("Indexer => {:?}", &token);
            match token.token_type {
                TokenType::CloseBracket => return Ok(Function::from_definition(FunctionDefinition::CHILD, FunctionParam::Integer(index.unwrap()))),
                TokenType::Number(number) => {
                    let num: isize = isize::from_str(number.as_str()).unwrap();
                    index = Some(num)
                },
                _ => return Err(FhirError::error("无效的索引值")),
            }
        };
    
        Err(FhirError::error("未发现代表结束的方括号"))
    }
}


#[test]
pub fn test_parser() -> Result<()> {
    let mut tokenizer = Tokenizer::new("Patient.name[0].given[2]");

    let list = tokenizer.tokenize()?;

    let plist = Parser::parse_path(list)?;

    for pt in plist {
        println!("{:?}", pt);
    };

    Ok(())
}
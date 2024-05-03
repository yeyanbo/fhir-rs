//! 实现了FHIRPath语法解析
//!
//!
mod tokenizer;
mod executor;
mod parser;
mod expression;
mod collection;

use std::ops::BitAnd;
pub use tokenizer::{Token, TokenType, Tokenizer, Operator};
pub use parser::Parser;
pub use collection::Collection;
pub use expression::{Expr};
pub use executor::{Executor, Convert, Compare};

#[test]
pub fn test_parser() -> crate::prelude::Result<()> {
    let mut tokenizer = Tokenizer::new("Patient.name.given[2].where(system = 'abc.cc' and code = 'abc')");

    let list = tokenizer.tokenize()?;
    let parser = Parser::new(list);
    let expr = parser.parse_expr()?;

    println!("{:#?}", expr);

    Ok(())
}

#[test]
pub fn test_expr_eval() -> crate::prelude::Result<()> {
    let mut tokenizer = Tokenizer::new("Patient.name.given[2].where(system = 'abc.cc' and code = 'abc')");

    let list = tokenizer.tokenize()?;
    let parser = Parser::new(list);
    let expr = parser.parse_expr()?;

    println!("{:#?}", expr);

    Ok(())
}
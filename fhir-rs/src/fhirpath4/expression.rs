use std::fmt::{Display, Formatter};
use crate::prelude::{Integer, Result, FhirError};
use super::*;

#[derive(Debug)]
pub enum Expr {
    IntegerExpr(Integer),
    StringExpr(String),
    DateTimeExpr(String),
    PathExpr{symbol: String, index: Option<usize>},
    CallExpr{symbol: String, args: Option<Vec<Expr>>},
    BinOpExpr{lhs: Box<Expr>, rhs: Box<Expr>, op: Operator},
}

impl Expr {
    pub fn parse(input: String) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.tokenize()?;
        Parser::new(tokens).parse_expr()
    }

    pub fn eval(&self, executor: &dyn Executor) -> Result<Collection> {
        match self {
            Expr::IntegerExpr(value) => {
                Ok(Collection::new_integer(*value))
            },
            Expr::StringExpr(value) => {
                Ok(Collection::new_string(value.clone()))
            }
            Expr::DateTimeExpr(value) => {
                Ok(Collection::new_datetime(value.parse()?))
            }
            Expr::PathExpr { symbol, index } => {
                let l_collection = executor.to_collection(&None);
                if executor.type_name() == *symbol {
                    Ok(l_collection)
                } else {
                    l_collection.element(symbol, index)
                }
            },
            Expr::CallExpr { symbol, args: _ } => {
                Err(FhirError::Message(format!("函数[{}]()为非主体表达式，只能出现在右侧。", symbol)))
            },
            Expr::BinOpExpr { lhs, rhs, op } => {
                let l_collection = lhs.eval(executor)?;

                match op {
                    Operator::Add => l_collection + rhs.eval(executor)?,
                    Operator::Sub => l_collection - rhs.eval(executor)?,
                    Operator::Mul => l_collection * rhs.eval(executor)?,
                    Operator::Div => l_collection / rhs.eval(executor)?,
                    Operator::And => {
                        let r_collection = rhs.eval(executor)?;
                        match (l_collection & r_collection)? {
                            None => Ok(Collection::new()),
                            Some(bl) => Ok(Collection::new_boolean(bl))
                        }
                    },
                    Operator::Or => {
                        let r_collection = rhs.eval(executor)?;
                        match (l_collection | r_collection)? {
                            None => Ok(Collection::new()),
                            Some(bl) => Ok(Collection::new_boolean(bl))
                        }
                    },
                    Operator::Xor => {
                        let r_collection = rhs.eval(executor)?;
                        match (l_collection ^ r_collection)? {
                            None => Ok(Collection::new()),
                            Some(bl) => Ok(Collection::new_boolean(bl))
                        }
                    },
                    Operator::As => unimplemented!(),
                    Operator::Is => unimplemented!(),
                    Operator::Eq => l_collection.eq(rhs.eval(executor)?),
                    Operator::Ne => l_collection.eq(rhs.eval(executor)?)?.not(),
                    Operator::Gt => l_collection.gt(rhs.eval(executor)?),
                    Operator::Ge => l_collection.ge(rhs.eval(executor)?),
                    Operator::Lt => l_collection.lt(rhs.eval(executor)?),
                    Operator::Le => l_collection.le(rhs.eval(executor)?),
                    Operator::Dot => {
                        match rhs.as_ref() {
                            Expr::PathExpr {symbol, index} => {
                                l_collection.element(symbol, index)
                            },
                            Expr::CallExpr {symbol, args} => {
                                l_collection.call(symbol, args)
                            },
                            _ => Err(FhirError::Message(format!("点号[.]操作符不支持该右侧表达式。")))
                        }
                    },
                    Operator::Unkown => unreachable!()
                }
            }
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::IntegerExpr(value) => write!(f, "Integer({})", value),
            Expr::StringExpr(value) => write!(f, "Text({})", value),
            Expr::DateTimeExpr(value) => write!(f, "DateTime({})", value),
            Expr::PathExpr { symbol, .. } => write!(f, "Path({})", symbol),
            Expr::CallExpr { symbol, .. } => write!(f, "Function({})", symbol),
            Expr::BinOpExpr { lhs: _, rhs: _, op } => write!(f, "Operator({})", op),
        }
    }
}
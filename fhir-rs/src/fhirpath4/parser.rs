use std::cell::Cell;

use crate::prelude::{Result, FhirError};
use super::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: Cell<usize>,
}
impl Parser {

    // 创建一个语法解析器
    pub fn new(tokens: Vec<Token>) -> Self {
        Self{
            tokens,
            current: Cell::new(0)
        }
    }

    pub fn parse_expr(&self) -> Result<Expr> {
        let expr = self.parse_primary()?;
        self.parse_binary_expr(0, expr)
    }

    fn parse_primary(&self) -> Result<Expr> {
        match self.peek() {
            None => Err(FhirError::error("expect expr")),
            Some(token) => {
                self.eat();
                match &token.token_type {
                    TokenType::Symbol(symbol) => self.parse_symbol_expr(symbol),
                    TokenType::Text(value) => Ok(Expr::StringExpr(value.clone())),
                    TokenType::DateTime(value) => Ok(Expr::DateTimeExpr(value.clone())),
                    TokenType::Number(value) => Ok(Expr::IntegerExpr(value.parse()?)),
                    TokenType::OpenParen => self.parse_paren_expr(),
                    other => Err(FhirError::Message(format!("unknown token: {:#?}", other)))
                }
            }
        }
    }

    /// 处理标识符
    /// 标识符可能有两种：
    /// 1. 路径（路径可能含有索引）
    /// 2. 函数
    fn parse_symbol_expr(&self, symbol: &String) -> Result<Expr> {
        match self.peek() {
            Some(token) => {
                match &token.token_type {
                    TokenType::OpenParen => {
                        self.eat();
                        let args = self.parse_function_args()?;
                        Ok(Expr::CallExpr {symbol: symbol.clone(), args})
                    },
                    TokenType::OpenBracket => {
                        self.eat();  // eat '['
                        let index = self.parse_path_index()?;
                        Ok(Expr::PathExpr { symbol: symbol.clone(), index: Some(index) })
                    }
                    _ => Ok(Expr::PathExpr {symbol: symbol.clone(), index: None}),
                }
            }
            None => Ok(Expr::PathExpr {symbol: symbol.clone(), index: None}),
        }
    }

    fn parse_binary_expr(&self, precedence: usize, lhs: Expr) -> Result<Expr> {
        let mut lhs = lhs;

        loop {
            let token = self.peek();

            match token {
                Some(op_token) => {
                    let (curr_prec, op) = self.token_precedence(op_token);

                    if curr_prec <= precedence {
                        return Ok(lhs)
                    }

                    self.eat();
                    let mut rhs = self.parse_primary()?;

                    // 如果下一个操作符的优先级大于当前的，则优先执行下一个操作
                    let next_prec = self.next_token_precedence();
                    if curr_prec < next_prec {
                        rhs = self.parse_binary_expr(curr_prec, rhs)?;
                    }

                    lhs = Expr::BinOpExpr {
                        lhs: Box::new(lhs),
                        rhs: Box::new(rhs),
                        op: op,
                    }
                },
                None => return Ok(lhs)
            }
        }
    }

    /// 处理圆括号范围内的子表达式
    /// 不处理代表函数的圆括号
    fn parse_paren_expr(&self) -> Result<Expr> {
        self.eat();  // eat '('
        let expr = self.parse_expr();

        match self.peek() {
            Some(token) => {
                match &token.token_type {
                    TokenType::CloseParen => {
                        self.eat();  // eat ')'
                        expr
                    },
                    _ => Err(FhirError::error("expected ')'")),
                }
            },
            None => Err(FhirError::error("expected ')'")),
        }
    }

    /// 处理路径的索引
    fn parse_path_index(&self) -> Result<usize> {
        let index = match self.peek() {
            Some(token) => {
                match &token.token_type {
                    TokenType::Number(value) => {
                        self.eat();   // eat number
                        value.parse()?
                    },
                    _ => return Err(FhirError::error("Expected ']' for index"))
                }
            },
            None => return Err(FhirError::error("Expected ']' for index")),
        };

        match self.peek() {
            Some(token) => {
                match &token.token_type {
                    TokenType::CloseBracket => {
                        self.eat();  //eat ']'
                    },
                    _ => return Err(FhirError::error("Expected ']' for index"))
                }
            },
            None => return Err(FhirError::error("Expected ']' for index"))
        }
        Ok(index)
    }

    /// 处理函数的参数
    fn parse_function_args(&self) -> Result<Option<Vec<Expr>>> {
        let mut args = vec![];

        loop {
            let arg = self.parse_expr()?;
            args.push(arg);

            match self.peek() {
                Some(token) => {
                    match token.token_type {
                        TokenType::Colon => self.eat(),
                        TokenType::CloseParen => {self.eat(); break},
                        _ => return Err(FhirError::error("Expected ')' or ',' in argument list"))
                    }
                },
                None => return Err(FhirError::error("Expected ')' or ',' in argument list")),
            }
        }
        Ok(if args.len() < 1 {None} else {Some(args)})
    }

    fn token_precedence(&self, token: &Token) -> (usize, Operator) {
        match &token.token_type {
            TokenType::Operator(op) => {
                match op {
                    Operator::Or | Operator::Xor => (2, *op),
                    Operator::And => (3, *op),
                    Operator::As | Operator::Is => (8, *op),
                    Operator::Add | Operator::Sub => (9, *op),
                    Operator::Mul | Operator::Div => (10, *op),
                    Operator::Dot => (13, *op),
                    _ => unreachable!()
                }
            }
            TokenType::Comparator(op) => {
                match op {
                    Operator::Eq | Operator::Ne => (5, *op),
                    Operator::Gt | Operator::Ge | Operator::Lt | Operator::Le => (6, *op),
                    _ => unreachable!()
                }
            }
            _ => (0, Operator::Unkown),
        }
    }

    fn next_token_precedence(&self) -> usize {
        match self.peek() {
            Some(token) => {
                let (prec, _) = self.token_precedence(token);
                prec
            },
            None => 0
        }
    }

    fn peek(&self) -> Option<&Token> {
        let current = self.current.get();
        self.tokens.get(current)
    }

    fn eat(&self) {
        let current = self.current.get();
        self.current.set(current + 1);
    }
}
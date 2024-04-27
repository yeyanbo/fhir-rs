use crate::prelude::Result;
use super::*;
use tracing::debug;

#[derive(Debug, Clone)]
pub struct Path {
    pub symbol: String,
    pub index: Option<usize>,
}

impl Path {
    pub fn new(symbol: String) -> Self {
        Self {
            symbol,
            index: None,
        }
    }

    pub fn with_index(symbol: String, index: usize) -> Self {
        Self {
            symbol,
            index: Some(index),
        }
    }
}

#[derive(Debug, Clone)]
pub enum PathComponent {
    Path(Path),
    Function(Function),
    Expression(PathExpression),
    LiteralString(String),
    LiteralDatetime(String),
    LiteralNumber(String),
    LiteralBool(bool),
}

impl PathComponent {
    pub fn response(&self) -> FunctionResponse {
        match self {
            PathComponent::Path(_) => FunctionResponse::Collection,
            PathComponent::Function(func) => func.response(),
            PathComponent::Expression(expr) => expr.response(),
            PathComponent::LiteralString(_) => FunctionResponse::String,
            PathComponent::LiteralDatetime(_) => FunctionResponse::DateTime,
            PathComponent::LiteralNumber(_) => FunctionResponse::Number,
            PathComponent::LiteralBool(_) => FunctionResponse::Bool,
        }
    }
}

#[derive(Debug)]
pub enum PathResponse {
    Collection(Collection),
    Integer(isize),
    String(String),
    Bool(bool)
}

impl PathResponse {
    pub fn exec(self, comp: &PathComponent) -> Result<PathResponse> {
        match self {
            PathResponse::Collection(collection) => collection.exec(comp),
            PathResponse::Integer(value) => value.exec(comp),
            PathResponse::String(value) => value.exec(comp),
            PathResponse::Bool(value) =>  value.exec(comp),
        }
    }
}

/// 记录表达式的根路径是相对路径，还是绝对路径
/// 还有一种与路径无关
#[derive(Debug, Clone)]
pub enum Root {
    None,
    Absolute(String),
    Relative,
}

#[derive(Debug, Clone)]
pub struct PathExpression {
    pub component: Vec<PathComponent>,
    pub current: usize,
    pub branch: usize,
    pub root: Root,
}

impl PathExpression {
    pub fn parse(input: String) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.tokenize()?;
        debug!("Tokens: {:?}", tokens);

        Parser::new(tokens).parse()
    }

    /// 获取下一组表达式项
    /// 每次取两个，如果越界，则返回空
    pub fn next(&mut self) -> (Option<&PathComponent>, Option<&PathComponent>) {
        if self.branch > 0 {
            self.current += self.branch;
            self.branch = 0;
        }

        self.current += 1;
        (self.component.get(self.current-1), self.component.get(self.current))
    }

    pub fn eat(&mut self) {
        self.current += 1;
    }

    /// 返回整个表达式最终的返回值类型
    pub fn response(&self) -> FunctionResponse {
        match self.component.last() {
            Some(comp) => comp.response(),
            None => FunctionResponse::Collection,
        }
    }
}
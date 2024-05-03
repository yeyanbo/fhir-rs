use std::ops::{Add, BitOr, BitXor, Div, Mul, Not, Sub};
use crate::datatype::Boolean;
use crate::prelude::{Result, FhirError, Integer, Decimal, DateTime};
use super::*;

#[derive(Debug)]
pub struct Collection(Vec<Box<dyn Executor>>);

impl Collection {
    /// 创建一个空集合
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn new_string(value: String) -> Self {
        Self(vec![Box::new(value)])
    }

    pub fn new_integer(value: Integer) -> Self {
        Self(vec![Box::new(value)])
    }

    pub fn new_decimal(value: Decimal) -> Self {
        Self(vec![Box::new(value)])
    }

    pub fn new_datetime(value: DateTime) -> Self {
        Self(vec![Box::new(value)])
    }

    pub fn new_boolean(value: Boolean) -> Self {
        Self(vec![Box::new(value)])
    }

    pub fn new_any(value: Box<dyn Executor>) -> Self {
        Self(vec![value])
    }

    pub fn first(&self) -> Option<&Box<dyn Executor>> {
        self.0.get(0)
    }

    pub fn count(&self) -> usize {
        self.0.len()
    }

    pub fn combine(&mut self, other: Collection) {
        self.0.extend(other.0)
    }

    pub fn push(&mut self, value: Box<dyn Executor>) {
        self.0.push(value)
    }

    pub fn empty(&self) -> bool {
        self.count() == 0
    }
    
    pub fn exists(&self) -> bool {
        self.count() > 0
    }

    pub fn to_integer(&self) -> Result<Option<Integer>> {
        match self.count() {
            0 => Ok(None),
            1 => Ok(Some(self.0[0].to_integer()?)),
            _ => Err(FhirError::error("集合内元素数量大于1")),
        }
    }

    pub fn to_boolean(&self) -> Result<Option<bool>> {
        match self.count() {
            0 => Ok(None),
            1 => Ok(Some(self.0[0].to_boolean()?)),
            other => Err(FhirError::Message(format!("集合元素数量[{}]大于1", other)))
        }
    }

    pub fn all_true(&self) -> Result<bool> {
        // for part in &self.0 {
        //     if !part.as_bool()? { return Ok(false)}
        // }
        Ok(true)
    }

    pub fn element(self, symbol: &String, index: &Option<usize>) -> Result<Collection> {
        let mut collection = Collection::new();
        for part in self.0 {
            let children = part.element(symbol, index)?;
            collection.combine(children)
        }

        Ok(collection)
    }

    pub fn filter(self, criteria: &Expr) -> Result<Collection> {
        let mut collection = Collection::new();

        for part in self.0 {
            let col = criteria.eval(part.as_ref())?;
            let bl = col.to_boolean()?;
            let bl = bl.unwrap_or_else(|| false);
            if bl {collection.push(part)}
        }

        Ok(collection)
    }

    pub fn call(self, symbol: &String, args: &Option<Vec<Expr>>) -> Result<Collection> {
        match symbol.as_str() {
            "where" => {
                match args {
                    None => Err(FhirError::error("where()至少拥有一个参数")),
                    Some(arg) => {
                        if arg.len() == 1 {
                            self.filter(&arg[0])
                        } else {
                            Err(FhirError::error("where()至多只能拥有一个参数"))
                        }
                    }
                }
            },
            other => Err(FhirError::Message(format!("这是无效或者未被支持的函数名[{}]", other))),
        }
    }

    pub fn single(self) -> Result<Collection> {
        if self.count() > 1 {
            return Err(FhirError::error("执行single函数时集合内超过一个元素"))
        }

        Ok(self)
    }

    /// 左侧集合与右侧集合相等
    /// 规则：
    /// 1. 两侧任何一侧为空，则结果为空
    /// 2. 如果两侧集合数量不相等，则结果为false
    pub fn eq(self, right: Collection) -> Result<Collection> {
        if self.empty() | right.empty() {
            return Ok(Collection::new())
        }

        if self.count() != right.count() {
            return Ok(Collection::new_boolean(false))
        }

        for (idx, part) in self.0.iter().enumerate() {
            let lhs = part.as_ref();
            let rhs = right.0[idx].as_ref();

            if !lhs.eq(rhs)? {
                return Ok(Collection::new_boolean(false))
            }
        }

        Ok(Collection::new_boolean(true))
    }

    pub fn not(self) -> Result<Collection> {
        match self.to_boolean()? {
            None => Ok(Collection::new()),
            Some(bl) => Ok(Collection::new_boolean(bl.not()))
        }
    }

    pub fn gt(self, _right: Collection) -> Result<Collection> {
        Ok(Collection::new_boolean(true))
    }

    pub fn ge(self, _right: Collection) -> Result<Collection> {
        Ok(Collection::new_boolean(true))
    }

    pub fn lt(self, _right: Collection) -> Result<Collection> {
        Ok(Collection::new_boolean(true))
    }

    pub fn le(self, _right: Collection) -> Result<Collection> {
        Ok(Collection::new_boolean(true))
    }
}

impl Add for Collection {
    type Output = Result<Collection>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.empty() | rhs.empty() {
            return Ok(Collection::new())
        }

        if (self.count() > 1) | (rhs.count() > 1) {
            return Err(FhirError::error("集合内元素数量大于1"))
        }
        // self.first() + rhs.first()
        // 由第一个元素，决定掉第二个元素的to_xxxx() 来实现相加
        Ok(Collection::new())
    }
}

impl Sub for Collection {
    type Output = Result<Collection>;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul for Collection {
    type Output = Result<Collection>;

    fn mul(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Div for Collection {
    type Output = Result<Collection>;

    fn div(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl BitAnd for Collection {
    type Output = Result<Option<bool>>;

    fn bitand(self, rhs: Self) -> Self::Output {
        match (self.to_boolean()?, rhs.to_boolean()?) {
            (Some(false), _) => Ok(Some(false)),
            (_, Some(false)) => Ok(Some(false)),
            (_, None) => Ok(None),
            (None, _) => Ok(None),
            (Some(true), Some(true)) => Ok(Some(true)),
        }
    }
}

impl BitOr for Collection {
    type Output = Result<Option<bool>>;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self.to_boolean()?, rhs.to_boolean()?) {
            (Some(true), _) => Ok(Some(true)),
            (_, Some(true)) => Ok(Some(true)),
            (_, None) => Ok(None),
            (None, _) => Ok(None),
            (Some(false), Some(false)) => Ok(Some(false)),
        }
    }
}

impl BitXor for Collection {
    type Output = Result<Option<bool>>;

    fn bitxor(self, rhs: Self) -> Self::Output {
        match (self.to_boolean()?, rhs.to_boolean()?) {
            (_, None) => Ok(None),
            (None, _) => Ok(None),
            (Some(true), Some(false)) => Ok(Some(true)),
            (Some(false), Some(true)) => Ok(Some(true)),
            (Some(true), Some(true)) => Ok(Some(false)),
            (Some(false), Some(false)) => Ok(Some(false)),
        }
    }
}
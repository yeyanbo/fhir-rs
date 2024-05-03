use crate::prelude::{Result, FhirError};
use super::*;

#[derive(Debug)]
pub struct Collection(pub Vec<Box<dyn Executor>>);

impl Collection {

    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn with(value: Box<dyn Executor>) -> Self {
        Self(vec![value])
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

    pub fn all_true(&self) -> Result<bool> {
        // for part in &self.0 {
        //     if !part.as_bool()? { return Ok(false)}
        // }
        Ok(true)
    }

    pub fn element(self, comp: &PathComponent) -> Result<Collection> {
        let mut vv = Collection::new();
        for part in self.0 {
            match part.exec(comp)? {
                PathResponse::Collection(collection) => vv.combine(collection),
                _ => unreachable!(),
            }
        }

        Ok(vv)
    }

    pub fn filter(self, criteria: &PathExpression) -> Result<Collection> {
        let mut collection = Collection::new();
        self.0.into_iter()
            .filter(|item| item.assert(criteria).unwrap())
            .for_each(|item| {
                collection.push(item);
            });

        Ok(collection)
    }

    pub fn single(self) -> Result<Self> {
        if self.count() > 1 {
            return Err(FhirError::error("执行single函数时集合内超过一个元素"))
        }

        Ok(self)
    }

    pub fn equal(self) -> bool {
        true
    }

    pub fn exec(self, comp: &PathComponent) -> Result<PathResponse> {

        match comp {
            PathComponent::Path(_) => {
                Ok(PathResponse::Collection(self.element(comp)?))
            },
            PathComponent::Function(func) => {
                match &func.symbol {
                    FunctionName::Exist => Ok(PathResponse::Bool(self.exists())),
                    FunctionName::Count => Ok(PathResponse::Integer(self.count() as isize)),
                    FunctionName::Empty => Ok(PathResponse::Bool(self.empty())),
                    FunctionName::Single => Ok(PathResponse::Collection(self.single()?)),
                    FunctionName::Where => Ok(PathResponse::Collection(self.filter(&func.params)?)),
                    FunctionName::AllTrue => Ok(PathResponse::Bool(self.all_true()?)),
                    FunctionName::Eq => Ok(PathResponse::Bool(self.equal())),
                    FunctionName::Other => Err(FhirError::error("无效的函数名")),
                }
            },
            _ => unreachable!()
        }
    }
}
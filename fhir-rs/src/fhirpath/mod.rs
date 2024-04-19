mod tokenizer;
mod parser;
mod function;

use crate::prelude::*;
use std::fmt::Debug;
pub use tokenizer::{Token, TokenType, Tokenizer};
pub use function::*;
pub use parser::Parser;
use tracing::info;

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
        for part in &self.0 {
            if !part.as_bool()? { return Ok(false)}
        }
        Ok(true)
    }

    pub fn element(self, func: &Function, paths: &mut FhirPaths) -> Result<Collection> {
        let mut vv = Collection::new();
        for part in self.0 {
            match part.exec(func, paths)? {
                PathResponse::Collection(collection) => vv.combine(collection),
                _ => unreachable!(),
            }
        }

        Ok(vv)
    }

    pub fn single(self) -> Result<Self> {
        if self.count() > 1 {
            return Err(FhirError::error("执行single函数时集合内超过一个元素"))
        }

        Ok(self)
    }

    pub fn exec(self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
        match func.definition.function_name() {
            FunctionName::Element => Ok(PathResponse::Collection(self.element(func, paths)?)),
            FunctionName::Single => Ok(PathResponse::Collection(self.single()?)),
            FunctionName::Child => {
                info!("{}", paths.current());
                Err(FhirError::error("不应该到这里"))
            },
            FunctionName::Where => todo!(),
            FunctionName::Count => Ok(PathResponse::Integer(self.count() as isize)),
            FunctionName::Empty => Ok(PathResponse::Bool(self.empty())),
            FunctionName::Exist => Ok(PathResponse::Bool(self.exists())),
            FunctionName::AllTrue => Ok(PathResponse::Bool(self.all_true()?)),
            FunctionName::Other => Err(FhirError::error("无效的函数名")),
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

    pub fn exec(self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
        match self {
            PathResponse::Collection(collection) => collection.exec(func, paths),
            PathResponse::Integer(value) => value.exec(func, paths),
            PathResponse::String(value) => value.exec(func, paths),
            PathResponse::Bool(value) =>  value.exec(func, paths),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FhirPaths {
    parts: Vec<Function>,
    current: usize,
    branch: usize,
}

impl FhirPaths {
    pub fn parse(input: String) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.tokenize()?;
        let mut parts = Parser::parse_path(tokens)?;

        match parts.first() {
            Some(func) => {
                if !func.is_resource_type_element() {
                    parts.insert(0, Function::create_self_element())
                }
            },
            None => return Err(FhirError::error("路径表达式不能为空")),
        }

        Ok(Self{parts, current: 0, branch: 0})
    }

    pub fn prev(&mut self) {
        self.current -= 1;
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn peek_index(&mut self) -> Option<&Function> {
        if self.current < self.parts.len() {
            let func = &self.parts[self.current];
            match func.definition.function_name() {
                FunctionName::Child => {
                    self.branch += 1;
                    Some(func)
                },
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn response(&self) -> Option<&FunctionResponse> {
        match self.parts.last() {
            Some(part) => Some(part.definition.response()),
            None => None,
        }
    }

    pub fn remove_last(&mut self) -> Function {
        let count = self.parts.len();
        self.parts.remove(count-1)
    }
}

impl Iterator for FhirPaths {
    type Item = Function;

    fn next(&mut self) -> Option<Self::Item> {
        if self.branch > 0 {
            self.current += self.branch;
            self.branch = 0;
        }

        if self.current < self.parts.len() {
            let value = self.parts[self.current].clone();
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub trait Executor: Debug + Base {
    fn as_collection(&self) -> Collection;
    fn as_collection2(&self) -> PathResponse {
        PathResponse::Collection(self.as_collection())
    }

    fn as_bool(&self) -> Result<bool> {
        Err(FhirError::error("该元素不是布尔类型"))
    }

    fn to_bool(&self) -> Result<bool> {
        Err(FhirError::error("该元素不能转换为布尔类型"))
    }

    fn converts_to_bool(&self) -> Result<bool> {
        Err(FhirError::error("该元素不能转换为布尔类型"))
    }

    fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
        Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", func)))
    }
}

impl Executor for String {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for usize {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for isize {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for f64 {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for i64 {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for bool {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_bool(&self) -> Result<bool> {
        Ok(self.clone())
    }

    fn to_bool(&self) -> Result<bool> {
        Ok(self.clone())
    }

    fn converts_to_bool(&self) -> Result<bool> {
        Ok(self.clone())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for Instant {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for DateTime {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for Time {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for Date {
    fn exec(&self, _func: &Function, _paths: &mut FhirPaths) -> Result<PathResponse> {
        Ok(self.as_collection2())
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl<T: Executor> Executor for Box<T> {
    fn as_collection(&self) -> Collection {
        self.as_ref().as_collection()
    }
}

impl<T: Executor> Executor for Option<T> {

    fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
        info!("enter into option exec");
        match self {
            Some(value) => value.exec(func, paths),
            None => Ok(PathResponse::Collection(Collection::new())),
        }
    }

    fn as_collection(&self) -> Collection {
        match self {
            Some(value) => value.as_collection(),
            None => Collection::new(),
        }
    }
}

impl<T: Executor> Executor for Vec<T> {
    fn exec(&self, _func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
        info!("enter vec exec");

        match paths.peek_index() {
            Some(func) => {
                match func.definition.function_name() {
                    FunctionName::Child => {
                        match func.params {
                            FunctionParam::Integer(index) => {
                                let item : Option<&T> = self.get(index as usize);
                                match item {
                                    Some(val) => Ok(val.as_collection2()),
                                    None => Ok(PathResponse::Collection(Collection::new())),
                                }
                            },
                            _ => unreachable!(),
                        }
                    }
                    _ => Ok(self.as_collection2()),
                }
            },
            None => Ok(self.as_collection2()),
        }
    }

    fn as_collection(&self) -> Collection {
        let mut vv = Collection::new();
        for item in self {
            vv.combine(item.as_collection())
        };
        vv
    }
}


impl Executor for AnyType {
    fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
        match self {
            AnyType::PositiveInt(value) => value.exec(func, paths),
            AnyType::String(value) => value.exec(func, paths),
            AnyType::Coding(value) => value.exec(func, paths),
            AnyType::DateTime(value) => value.exec(func, paths),
            AnyType::Base64Binary(value) => value.exec(func, paths),
            AnyType::Boolean(value) => value.exec(func, paths),
            AnyType::Code(value) => value.exec(func, paths),
            AnyType::Id(value) => value.exec(func, paths),
            AnyType::Markdown(value) => value.exec(func, paths),
            AnyType::Uri(value) => value.exec(func, paths),
            AnyType::Url(value) => value.exec(func, paths),
            AnyType::Uuid(value) => value.exec(func, paths),
            AnyType::Oid(value) => value.exec(func, paths),
            AnyType::Canonical(value) => value.exec(func, paths),
            AnyType::Date(value) => value.exec(func, paths),
            AnyType::Time(value) => value.exec(func, paths),
            AnyType::Instant(value) => value.exec(func, paths),
            AnyType::UnsignedInt(value) => value.exec(func, paths),
            AnyType::Integer(value) => value.exec(func, paths),
            AnyType::Integer64(value) => value.exec(func, paths),
            AnyType::Decimal(value) => value.exec(func, paths),
            AnyType::Address(value) => value.exec(func, paths),
            AnyType::Age(value) => value.exec(func, paths),
            AnyType::Annotation(value) => value.exec(func, paths),
            AnyType::Attachment(value) => value.exec(func, paths),
            AnyType::CodeableConcept(value) => value.exec(func, paths),
            AnyType::CodeableReference(value) => value.exec(func, paths),
            AnyType::ContactPoint(value) => value.exec(func, paths),
            AnyType::Count(value) => value.exec(func, paths),
            AnyType::Distance(value) => value.exec(func, paths),
            AnyType::Duration(value) => value.exec(func, paths),
            AnyType::HumanName(value) => value.exec(func, paths),
            AnyType::Identifier(value) => value.exec(func, paths),
            AnyType::Money(value) => value.exec(func, paths),
            AnyType::Period(value) => value.exec(func, paths),
            AnyType::Quantity(value) => value.exec(func, paths),
            AnyType::Range(value) => value.exec(func, paths),
            AnyType::Ratio(value) => value.exec(func, paths),
            AnyType::RatioRange(value) => value.exec(func, paths),
            AnyType::Reference(value) => value.exec(func, paths),
            AnyType::SampledData(value) => value.exec(func, paths),
            AnyType::Signature(value) => value.exec(func, paths),
            AnyType::Timing(value) => value.exec(func, paths),
            AnyType::ContactDetail(value) => value.exec(func, paths),
            AnyType::DataRequirement(value) => value.exec(func, paths),
            AnyType::Expression(value) => value.exec(func, paths),
            AnyType::ParameterDefinition(value) => value.exec(func, paths),
            AnyType::RelatedArtifact(value) => value.exec(func, paths),
            AnyType::TriggerDefinition(value) => value.exec(func, paths),
            AnyType::UsageContext(value) => value.exec(func, paths),
            AnyType::Availability(value) => value.exec(func, paths),
            AnyType::ExtendedContactDetail(value) => value.exec(func, paths),
            AnyType::Dosage(value) => value.exec(func, paths),
            AnyType::Meta(value) => value.exec(func, paths),
        }
    }

    fn as_collection(&self) -> Collection {
        match self {
            AnyType::PositiveInt(value) => {
                value.as_collection()
            }
            AnyType::String(value) => {
                value.as_collection()
            }
            AnyType::Coding(value) => {
                value.as_collection()
            }
            AnyType::DateTime(value) => {
                value.as_collection()
            }
            AnyType::Base64Binary(value) => {
                value.as_collection()
            }
            AnyType::Boolean(value) => {
                value.as_collection()
            }
            AnyType::Code(value) => {
                value.as_collection()
            }
            AnyType::Id(value) => {
                value.as_collection()
            }
            AnyType::Markdown(value) => {
                value.as_collection()
            }
            AnyType::Uri(value) => {
                value.as_collection()
            }
            AnyType::Url(value) => {
                value.as_collection()
            }
            AnyType::Uuid(value) => {
                value.as_collection()
            }
            AnyType::Oid(value) => {
                value.as_collection()
            }
            AnyType::Canonical(value) => {
                value.as_collection()
            }
            AnyType::Date(value) => {
                value.as_collection()
            }
            AnyType::Time(value) => {
                value.as_collection()
            }
            AnyType::Instant(value) => {
                value.as_collection()
            }
            AnyType::UnsignedInt(value) => {
                value.as_collection()
            }
            AnyType::Integer(value) => {
                value.as_collection()
            }
            AnyType::Integer64(value) => {
                value.as_collection()
            }
            AnyType::Decimal(value) => {
                value.as_collection()
            }
            AnyType::Address(value) => {
                value.as_collection()
            }
            AnyType::Age(value) => {
                value.as_collection()
            }
            AnyType::Annotation(value) => {
                value.as_collection()
            }
            AnyType::Attachment(value) => {
                value.as_collection()
            }
            AnyType::CodeableConcept(value) => {
                value.as_collection()
            }
            AnyType::CodeableReference(value) => {
                value.as_collection()
            }
            AnyType::ContactPoint(value) => {
                value.as_collection()
            }
            AnyType::Count(value) => {
                value.as_collection()
            }
            AnyType::Distance(value) => {
                value.as_collection()
            }
            AnyType::Duration(value) => {
                value.as_collection()
            }
            AnyType::HumanName(value) => {
                value.as_collection()
            }
            AnyType::Identifier(value) => {
                value.as_collection()
            }
            AnyType::Money(value) => {
                value.as_collection()
            }
            AnyType::Period(value) => {
                value.as_collection()
            }
            AnyType::Quantity(value) => {
                value.as_collection()
            }
            AnyType::Range(value) => {
                value.as_collection()
            }
            AnyType::Ratio(value) => {
                value.as_collection()
            }
            AnyType::RatioRange(value) => {
                value.as_collection()
            }
            AnyType::Reference(value) => {
                value.as_collection()
            }
            AnyType::SampledData(value) => {
                value.as_collection()
            }
            AnyType::Signature(value) => {
                value.as_collection()
            }
            AnyType::Timing(value) => {
                value.as_collection()
            }
            AnyType::ContactDetail(value) => {
                value.as_collection()
            }
            AnyType::DataRequirement(value) => {
                value.as_collection()
            }
            AnyType::Expression(value) => {
                value.as_collection()
            }
            AnyType::ParameterDefinition(value) => {
                value.as_collection()
            }
            AnyType::RelatedArtifact(value) => {
                value.as_collection()
            }
            AnyType::TriggerDefinition(value) => {
                value.as_collection()
            }
            AnyType::UsageContext(value) => {
                value.as_collection()
            }
            AnyType::Availability(value) => {
                value.as_collection()
            }
            AnyType::ExtendedContactDetail(value) => {
                value.as_collection()
            }
            AnyType::Dosage(value) => {
                value.as_collection()
            }
            AnyType::Meta(value) => {
                value.as_collection()
            }
        }
    }
}
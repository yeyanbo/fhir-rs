mod tokenizer;
mod parser;
mod function;


use crate::prelude::*;
use std::fmt::Debug;
pub use tokenizer::{Token, TokenType, Tokenizer};
pub use function::*;
pub use parser::Parser;

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
}

#[derive(Debug, Clone)]
pub struct FhirPaths {
    parts: Vec<Function>,
    current: usize,
}

impl FhirPaths {
    pub fn parse(input: String) -> Result<Self> {
        let mut tokenizer = Tokenizer::new(&input);
        let tokens = tokenizer.tokenize()?;
        let parts = Parser::parse_path(tokens)?;

        Ok(Self{parts, current: 0})
    }

    pub fn prev(&mut self) {
        self.current -= 1;
    }

    pub fn response(&self) -> Option<&FunctionResponse> {
        match self.parts.last() {
            Some(part) => Some(part.definition.response()),
            None => None,
        }
    }
}

impl Iterator for FhirPaths {
    type Item = Function;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.parts.len() {
            let value = self.parts[self.current].clone();
            self.current += 1;
            Some(value)
        } else {
            None
        }
    }
}

pub trait Executor: Debug {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection>;
    fn as_collection(&self) -> Collection;
}

impl Executor for String {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for usize {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for isize {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for f64 {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for i64 {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for bool {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for Instant {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for DateTime {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for Time {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl Executor for Date {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match paths.next() {
            Some(func) => Err(FhirError::Message(format!("String: 基础类型不支持的函数:{:?}", &func))),
            None => Ok(self.as_collection()),
        }
    }

    fn as_collection(&self) -> Collection {
        Collection::with(Box::new(self.clone()))
    }
}

impl<T: Executor> Executor for Box<T> {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
            self.as_ref().path(paths)
    }

    fn as_collection(&self) -> Collection {
        self.as_ref().as_collection()
    }
}

impl<T: Executor> Executor for Option<T> {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match self {
            Some(value) => value.path(paths),
            None => Ok(Collection::new()),
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
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {

        match paths.next() {
            Some(func) => {
                match func.definition.function_name() {
                    FunctionName::Child => {
                        match func.params {
                            FunctionParam::Integer(index) => {
                                let item : Option<&T> = self.get(index as usize);
                                match item {
                                    Some(val) => val.path(paths),
                                    None => todo!(),
                                }
                            },
                            _ => unreachable!(),
                        }
                    },
                    FunctionName::Element => {
                        paths.prev();
                        let mut vv = Collection::new();
                        for item in self {
                            let rs = item.path(&mut paths.clone())?;
                            vv.combine(rs);
                        }
                        Ok(vv)
                    },
                    _ => todo!(),
                }
            },
            None => Ok(self.as_collection()),
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
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match self {
            AnyType::PositiveInt(value) => {
                value.path(paths)
            }
            AnyType::String(value) => {
                value.path(paths)
            }
            AnyType::Coding(value) => {
                value.path(paths)
            }
            AnyType::DateTime(value) => {
                value.path(paths)
            }
            AnyType::Base64Binary(value) => {
                value.path(paths)
            }
            AnyType::Boolean(value) => {
                value.path(paths)
            }
            AnyType::Code(value) => {
                value.path(paths)
            }
            AnyType::Id(value) => {
                value.path(paths)
            }
            AnyType::Markdown(value) => {
                value.path(paths)
            }
            AnyType::Uri(value) => {
                value.path(paths)
            }
            AnyType::Url(value) => {
                value.path(paths)
            }
            AnyType::Uuid(value) => {
                value.path(paths)
            }
            AnyType::Oid(value) => {
                value.path(paths)
            }
            AnyType::Canonical(value) => {
                value.path(paths)
            }
            AnyType::Date(value) => {
                value.path(paths)
            }
            AnyType::Time(value) => {
                value.path(paths)
            }
            AnyType::Instant(value) => {
                value.path(paths)
            }
            AnyType::UnsignedInt(value) => {
                value.path(paths)
            }
            AnyType::Integer(value) => {
                value.path(paths)
            }
            AnyType::Integer64(value) => {
                value.path(paths)
            }
            AnyType::Decimal(value) => {
                value.path(paths)
            }
            AnyType::Address(value) => {
                value.path(paths)
            }
            AnyType::Age(value) => {
                value.path(paths)
            }
            AnyType::Annotation(value) => {
                value.path(paths)
            }
            AnyType::Attachment(value) => {
                value.path(paths)
            }
            AnyType::CodeableConcept(value) => {
                value.path(paths)
            }
            AnyType::CodeableReference(value) => {
                value.path(paths)
            }
            AnyType::ContactPoint(value) => {
                value.path(paths)
            }
            AnyType::Count(value) => {
                value.path(paths)
            }
            AnyType::Distance(value) => {
                value.path(paths)
            }
            AnyType::Duration(value) => {
                value.path(paths)
            }
            AnyType::HumanName(value) => {
                value.path(paths)
            }
            AnyType::Identifier(value) => {
                value.path(paths)
            }
            AnyType::Money(value) => {
                value.path(paths)
            }
            AnyType::Period(value) => {
                value.path(paths)
            }
            AnyType::Quantity(value) => {
                value.path(paths)
            }
            AnyType::Range(value) => {
                value.path(paths)
            }
            AnyType::Ratio(value) => {
                value.path(paths)
            }
            AnyType::RatioRange(value) => {
                value.path(paths)
            }
            AnyType::Reference(value) => {
                value.path(paths)
            }
            AnyType::SampledData(value) => {
                value.path(paths)
            }
            AnyType::Signature(value) => {
                value.path(paths)
            }
            AnyType::Timing(value) => {
                value.path(paths)
            }
            AnyType::ContactDetail(value) => {
                value.path(paths)
            }
            AnyType::DataRequirement(value) => {
                value.path(paths)
            }
            AnyType::Expression(value) => {
                value.path(paths)
            }
            AnyType::ParameterDefinition(value) => {
                value.path(paths)
            }
            AnyType::RelatedArtifact(value) => {
                value.path(paths)
            }
            AnyType::TriggerDefinition(value) => {
                value.path(paths)
            }
            AnyType::UsageContext(value) => {
                value.path(paths)
            }
            AnyType::Availability(value) => {
                value.path(paths)
            }
            AnyType::ExtendedContactDetail(value) => {
                value.path(paths)
            }
            AnyType::Dosage(value) => {
                value.path(paths)
            }
            AnyType::Meta(value) => {
                value.path(paths)
            }
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
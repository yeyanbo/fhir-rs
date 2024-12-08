mod primitive;
mod complex;
mod base;
mod macros;
mod extension;
mod anytype;

pub use primitive::*;
pub use complex::*;
pub use base::*;
pub use extension::*;
pub use anytype::*;

use std::fmt::{Debug, Display};
use std::str::FromStr;

/// 所有数据类型的基类
pub trait Base : Debug {
    fn type_name(&self) -> &str;
}

/// 所有数据元素的基类型，继承自Base
pub trait Element: Base {
    fn id(&self) -> Option<&String>;
    fn set_id<T: Into<String>>(self, id: T) -> Self;
    fn extensions(&self) -> Option<&Vec<Extension>>;
    fn set_extensions(self, ext: Vec<Extension>) -> Self;
    fn add_extension(self, ext: Extension) -> Self;
}

/// 用于简单类型和复合类型的基类
pub trait DataType : Element {}


pub trait Backbone {
    fn modifier_extensions(&self) -> Option<&Vec<Extension>>;
    fn set_modifier_extensions(self, ext: Vec<Extension>) -> Self;
    fn add_modifier_extension(self, ext: Extension) -> Self;
    fn modifier_extension<U: Into<Url>>(&self, url: U) -> Option<&Extension>;
}
/// 有一些特殊类需要从Backbone类型继承
/// Timing, Dosage, ElementDefinition
pub trait BackboneType : DataType + Backbone {}

pub trait BackboneElement : Element + Backbone {}

/// FHIR简单类型的特性
/// FHIR简单类型是RUST简单数据类型的包装器
///
pub trait Primitive: DataType + Display + FromStr {
    type T;
    fn new<A: Into<Self::T>>(v: A) -> Self;
    fn has_value(&self) -> bool;
    fn value(&self) -> &Option<Self::T>;
    fn set_value(self, v: Self::T) -> Self;
    fn combine(&mut self, other: Self);
}

pub trait Resource {
    fn id(&self) -> &Option<Id>;
    fn set_id<T: Into<Id>>(self, id: T) -> Self;
    fn meta(&self) -> &Option<Meta>;
    fn set_meta(self, meta: Meta) -> Self;
}

pub trait DomainResource: Resource {
    fn extension(&self) -> &Option<Vec<Extension>>;
    fn set_extension(self, ext: Vec<Extension>) -> Self;
    fn add_extension(self, ext: Extension) -> Self;
    fn modifier_extension(&self) -> &Option<Vec<Extension>>;
    fn set_modifier_extension(self, ext: Vec<Extension>) -> Self;
    fn add_modifier_extension(self, ext: Extension) -> Self;
}


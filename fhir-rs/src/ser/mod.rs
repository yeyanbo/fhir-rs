//! FHIR资源的序列化解析器
//!
//! 序列化解析器的作用是将FHIR资源转换为指定格式的字符串
//!
//! 目前提供了两种格式的序列化解析器
//! * XML格式
//! * JSON格式
//!
//!

mod json_serializer;
mod xml_serializer;

pub use json_serializer::to_string as to_json;
pub use xml_serializer::to_string as to_xml;

use crate::prelude::*;

/// 实现该特性的数据结构体能够被序列化到指定类型的字符串
pub trait Serialize {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()>;
}

/// 能够将资源序列化到指定格式字符串的解析器
pub trait Serializer: Sized {
    type SerializeStruct: SerializeStruct;
    type SerializeVec: SerializeVec;
    type SerializePrimitive: SerializePrimitive;
    type SerializeExtension: SerializeExtension;

    fn serialize_any<T: Serialize>(self, name: &str, value: &T) -> Result<()>;
    fn serialize_str(self, value: &str) -> Result<()>;

    fn serialize_string(self, value: String) -> Result<()>;
    /// 序列化一个布尔值
    ///
    /// Serialize a `bool` value
    fn serialize_bool(self, value: bool) -> Result<()>;

    /// 序列化一个正整数值
    ///
    /// Serialize an `positiveInt` value.
    ///
    /// 对应FHIR规范定义的positiveInt类型(1..2,147,483,647)
    ///
    /// positiveInt: Any positive integer in the range 1..2,147,483,647
    fn serialize_number(self, value: usize) -> Result<()>;
    fn serialize_integer(self, value: isize) -> Result<()>;
    fn serialize_integer64(self, value: i64) -> Result<()>;
    fn serialize_decimal(self, value: f64) -> Result<()>;
    fn serialize_none(self) -> Result<()>;
    fn serialize_primitive(self) -> Result<Self::SerializePrimitive>;
    fn serialize_vec(self, len: Option<usize>) -> Result<Self::SerializeVec>;
    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct>;
    fn serialize_extension(self) -> Result<Self::SerializeExtension>;
}

pub trait SerializeStruct {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()>;
    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()>;
    fn serialize_field<T: Serialize>(&mut self, name: &'static str, value: &T) -> Result<()>;

    fn serialize_end(self) -> Result<()>;
}

pub trait SerializeVec {
    fn serialize_element<T: Serialize>(&mut self, value: &T) -> Result<()>;
    fn serialize_end(self) -> Result<()>;
}

pub trait SerializePrimitive {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()>;
    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()>;
    fn serialize_value<T: Serialize>(&mut self, value: &Option<T>) -> Result<()>;
    fn serialize_end(self) -> Result<()>;
}

pub trait SerializeExtension {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()>;
    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()>;
    fn serialize_url(&mut self, value: &Option<String>) -> Result<()>;
    fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<()>;
    fn serialize_end(self) -> Result<()>;
}

impl Serialize for String {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_str(self)
    }
}

impl Serialize for Boolean {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_bool(*self)
    }
}

impl Serialize for usize {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_number(*self)
    }
}

impl Serialize for isize {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_integer(*self)
    }
}

impl Serialize for f64 {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_decimal(*self)
    }
}

impl Serialize for i64 {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_integer64(*self)
    }
}

impl Serialize for Date {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_string(self.to_string())
    }
}

impl Serialize for Time {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_string(self.to_string())
    }
}

impl Serialize for DateTime {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_string(self.to_string())
    }
}

impl Serialize for Instant {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_string(self.to_string())
    }
}

impl<T: Serialize> Serialize for Vec<T> {
    #[inline]
    fn serialize<Ser: Serializer>(&self, serializer: Ser)-> Result<()> {
        tracing::debug!("开始处理数组");

        let iter = self.into_iter();
        let mut vec = serializer.serialize_vec(Some(self.len()))?;
        for item in iter {
            vec.serialize_element(item)?;
        }
        vec.serialize_end()
    }
}

impl<T: Serialize> Serialize for Option<T> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        match self {
            Some(value) => {
                value.serialize(serializer)
            },
            None => {
                serializer.serialize_none()?;
                Ok(())
            },
        }
    }
}

impl<T: Serialize> Serialize for Box<T> {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        (**self).serialize(serializer)
    }
}

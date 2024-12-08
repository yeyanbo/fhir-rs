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
pub use json_serializer::to_string_pretty as to_json_pretty;
pub use xml_serializer::to_string as to_xml;
pub use xml_serializer::to_string_pretty as to_xml_pretty;

use crate::prelude::*;

/// 实现该特性的数据结构体能够被序列化到指定类型的字符串
pub trait Serialize {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()>;
}

/// 能够将资源序列化到指定格式字符串的解析器
pub trait Serializer: Sized {
    type SerializeResource: SerializeResource;
    type SerializePrimitive: SerializePrimitive;
    type SerializeStruct: SerializeStruct;
    type SerializeExtension: SerializeExtension;
    type SerializeNarrative: SerializeNarrative;
    type SerializeVec: SerializeVec;

    fn serialize_any<T: Serialize>(self, type_name: &str, value: &T) -> Result<()>;
    fn serialize_str(self, value: &str) -> Result<()>;
    fn serialize_string(self, value: String) -> Result<()>;

    /// 序列化一个复合XHTML规则的字符串
    fn serialize_xhtml(self, value: &Xhtml) -> Result<()>;
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
    fn serialize_resource(self, name: &'static str) -> Result<Self::SerializeResource>;
    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct>;
    fn serialize_extension(self) -> Result<Self::SerializeExtension>;
    fn serialize_narrative(self) -> Result<Self::SerializeNarrative>;
}

pub trait SerializeResource {
    fn serialize_id(&mut self, value: &Option<Id>) -> Result<()>;
    fn serialize_field<T: Serialize>(&mut self, name: &'static str, value: &T) -> Result<()>;
    fn serialize_end(self) -> Result<()>;
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

pub trait SerializeNarrative {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()>;
    fn serialize_xhtml<T: Serialize>(&mut self, value: &Option<T>) -> Result<()>;
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

impl Serialize for Xhtml {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        serializer.serialize_xhtml(&self)
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


macro_rules! impl_serialize_for_primitive {
    (
        $($ty: ident, )+
    ) => {
        $(
            impl Serialize for $ty {
                fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
                    let mut primitive  = serializer.serialize_primitive()?;
                    primitive.serialize_id(&self.id)?;
                    primitive.serialize_value(&self.value)?;
                    primitive.serialize_extension(&self.extension)?;
                    primitive.serialize_end()
                }
            }
        )+
    };
}

impl_serialize_for_primitive!{
    StringDt,
    IdDt,
    Base64BinaryDt,
    MarkdownDt,
    UriDt,
    UrlDt,
    OidDt,
    UuidDt,
    CanonicalDt,
    CodeDt,
    BooleanDt,
    DateTimeDt,
    DateDt,
    TimeDt,
    InstantDt,
    UnsignedIntDt,
    PositiveIntDt,
    IntegerDt,
    Integer64Dt,
    DecimalDt,
}

macro_rules! impl_serialize_for_anytype {
    (
        $($id: ident,)+
    ) => {
        impl Serialize for AnyType {
            fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
                match self {
                    $(AnyType::$id(value) => serializer.serialize_any(stringify!($id), value),)+
                }
            }
        }
    };
}

impl_serialize_for_anytype!{
    String,
    Id,
    Base64Binary,
    Markdown,
    Uri,
    Url,
    Oid,
    Uuid,
    Canonical,
    Code,
    Boolean,
    DateTime,
    Date,
    Time,
    Instant,
    UnsignedInt,
    PositiveInt,
    Integer,
    Integer64,
    Decimal,
    Address,
    Age,
    Annotation,
    Attachment,
    CodeableConcept,
    CodeableReference,
    Coding,
    ContactPoint,
    Count,
    Distance,
    Duration,
    HumanName,
    Identifier,
    Money,
    Period,
    Quantity,
    Range,
    Ratio,
    RatioRange,
    Reference,
    SampledData,
    Signature,
    Timing,
    ContactDetail,
    DataRequirement,
    Expression,
    ParameterDefinition,
    RelatedArtifact,
    TriggerDefinition,
    UsageContext,
    Availability,
    ExtendedContactDetail,
    Dosage,
    Meta,
}

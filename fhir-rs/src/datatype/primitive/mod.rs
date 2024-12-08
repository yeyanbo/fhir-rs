mod datetime;
mod xhtml;

pub use datetime::*;
pub use xhtml::*;

use crate::prelude::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use fhir_derive::Element;

#[derive(Element, Primitive, Debug, Clone)]
pub struct StringDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for string
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<String>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct IdDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for id
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Id>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct Base64BinaryDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for base64Binary
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Base64Binary>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct MarkdownDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for markdown
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Markdown>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct UriDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for uri
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Uri>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct OidDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for oid
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Oid>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct CanonicalDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for canonical
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Canonical>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct CodeDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for code
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Code>,
}

/// 布尔类型
///
/// true | false
#[derive(Element, Primitive, Clone, Debug)]
pub struct BooleanDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for boolean
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Boolean>,
}

/// 正整数类型
///
/// 1..2,147,483,647
#[derive(Element, Primitive, Debug, Clone)]
pub struct PositiveIntDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for positiveInt
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<PositiveInt>,
}


#[derive(Element, Primitive, Clone, Debug)]
pub struct IntegerDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for integer
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Integer>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct Integer64Dt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for integer64
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Integer64>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct DecimalDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for decimal
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Decimal>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct UnsignedIntDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for unsignedInt
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<UnsignedInt>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct UrlDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for url
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Url>,
}

#[derive(Element, Primitive, Debug, Clone)]
pub struct UuidDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for uuid
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Uuid>,
}

macro_rules! impl_from_str {
    ($primitive:ident, $from:ident $($cast:tt)*) => {
        impl FromStr for $primitive {
            type Err = FhirError;

            fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
                let val: $from = FromStr::from_str(s)?;
                Ok(Self {
                    id: None,
                    extension: None,
                    value: Some(val $($cast)*),
                })
            }
        }
    }
}

impl_from_str!(StringDt, String);
impl_from_str!(UriDt, Uri);
impl_from_str!(UrlDt, Url);
impl_from_str!(CanonicalDt, Canonical);
impl_from_str!(UuidDt, Uuid);
impl_from_str!(OidDt, Oid);
impl_from_str!(IdDt, Id);
impl_from_str!(CodeDt, Code);
impl_from_str!(MarkdownDt, Markdown);
impl_from_str!(Base64BinaryDt, Base64Binary);
impl_from_str!(XhtmlDt, Xhtml);

impl_from_str!(PositiveIntDt, PositiveInt);
impl_from_str!(IntegerDt, Integer);
impl_from_str!(Integer64Dt, Integer64);
impl_from_str!(UnsignedIntDt, UnsignedInt);
impl_from_str!(DecimalDt, Decimal);

impl_from_str!(BooleanDt, Boolean);

impl_from_str!(InstantDt, Instant);
impl_from_str!(DateTimeDt, DateTime);
impl_from_str!(DateDt, Date);
impl_from_str!(TimeDt, Time);

macro_rules! primitive_from_impl {
    ($primitive:ident) => {
        impl From<&str> for $primitive {
            fn from(value: &str) -> Self {
                let value = FromStr::from_str(value).unwrap();
                $primitive {
                    id: None,
                    extension: None,
                    value: Some(value),
                }
            }
        }
    };

    ($primitive:ident, $from:ident $($cast:tt)*) => {
        impl From<$from> for $primitive {
            fn from(value: $from) -> Self {
                $primitive {
                    id: None,
                    extension: None,
                    value: Some(value $($cast)*),
                }
            }
        }
    }
}

primitive_from_impl!(StringDt);
primitive_from_impl!(UriDt);
primitive_from_impl!(UrlDt);
primitive_from_impl!(CanonicalDt);
primitive_from_impl!(UuidDt);
primitive_from_impl!(OidDt);
primitive_from_impl!(IdDt);
primitive_from_impl!(CodeDt);
primitive_from_impl!(MarkdownDt);
primitive_from_impl!(Base64BinaryDt);
primitive_from_impl!(XhtmlDt);
primitive_from_impl!(BooleanDt, bool);
primitive_from_impl!(PositiveIntDt, u8 as usize);
primitive_from_impl!(PositiveIntDt, u16 as usize);
primitive_from_impl!(PositiveIntDt, u32 as usize);
primitive_from_impl!(PositiveIntDt, i8 as usize);
primitive_from_impl!(PositiveIntDt, i16 as usize);
primitive_from_impl!(PositiveIntDt, i32 as usize);
primitive_from_impl!(IntegerDt, u8 as isize);
primitive_from_impl!(IntegerDt, u16 as isize);
primitive_from_impl!(IntegerDt, u32 as isize);
primitive_from_impl!(IntegerDt, i8 as isize);
primitive_from_impl!(IntegerDt, i16 as isize);
primitive_from_impl!(IntegerDt, i32 as isize);
primitive_from_impl!(Integer64Dt, i8 as i64);
primitive_from_impl!(Integer64Dt, i16 as i64);
primitive_from_impl!(Integer64Dt, i32 as i64);
primitive_from_impl!(Integer64Dt, i64);

primitive_from_impl!(InstantDt, Instant);
primitive_from_impl!(DateTimeDt, DateTime);
primitive_from_impl!(DateDt, Date);
primitive_from_impl!(TimeDt, Time);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::prelude::*;

    #[test]
    fn test_string() {
        let name = StringDt::new("zhangsan");
        assert_eq!(name.value.unwrap(), "zhangsan".to_string())
    }

    #[test]
    fn test_int() {
        let size = PositiveIntDt::new(356usize);
        assert_eq!(size.value.unwrap(), 356)
    }

    #[test]
    fn test_bool() {
        let gender = BooleanDt::new(false);
        assert_eq!(gender.value.unwrap(), false)
    }
    
    #[test]
    fn test_date() -> Result<()> {
        let d1 = DateDt::from_str("2009")?;
        tracing::debug!("Date: {}", d1.to_string());
        let d1 = DateDt::from_str("2009-12")?;
        tracing::debug!("Date: {}", d1);
        let d1 = DateDt::from_str("2009-12-23")?;
        tracing::debug!("Date: {}", d1);

        let t1 = TimeDt::from_str("23:12:45")?;
        tracing::debug!("Time: {:?}", t1);
        let t1 = TimeDt::from_str("23:12:45.234")?;
        tracing::debug!("Time: {}", t1);
        let t1 = TimeDt::from_str("23:12:60.040")?;
        tracing::debug!("Time: {}", t1);

        tracing::debug!("=======================");

        let dt1 = DateTimeDt::from_str("2009")?;
        tracing::debug!("DateTime: {}", dt1.to_string());
        let dt1 = DateTimeDt::from_str("2009-12")?;
        tracing::debug!("DateTime: {:?}", dt1);
        let dt1 = DateTimeDt::from_str("2009-12-23")?;
        tracing::debug!("DateTime: {}", dt1);
        let dt1 = DateTimeDt::from_str("2009-12-23T23:12:45Z")?;
        tracing::debug!("DateTime: {:?}", dt1);
        let dt1 = DateTimeDt::from_str("2009-12-23T23:12:45.234Z")?;
        tracing::debug!("DateTime: {}", dt1);
        let dt1 = DateTimeDt::from_str("2009-12-23T23:12:45-06:00")?;
        tracing::debug!("DateTime: {}", dt1);

        tracing::debug!("=======================");

        let instant1 = InstantDt::from_str("2009-12-23T23:12:45Z")?;
        tracing::debug!("Instant: {}", instant1);
        let instant1 = InstantDt::from_str("2009-12-23T23:12:45.234Z")?;
        tracing::debug!("Instant: {}", instant1);
        let instant1 = InstantDt::from_str("2009-12-23T23:12:45-06:00")?;
        tracing::debug!("Instant: {}", instant1);
        let instant1 = InstantDt::from_str("2009-12-23T23:12:45.456-06:00")?;
        tracing::debug!("Instant: {}", instant1);
        Ok(())
    }

}

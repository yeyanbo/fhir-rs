use crate::prelude::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Primitive, Debug, Clone)]
pub struct StringDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for string
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<String>,
}

#[derive(Primitive, Debug, Clone)]
pub struct IdDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for id
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Id>,
}

#[derive(Primitive, Debug, Clone)]
pub struct Base64BinaryDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for base64Binary
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Base64Binary>,
}

#[derive(Primitive, Debug, Clone)]
pub struct MarkdownDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for markdown
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Markdown>,
}

#[derive(Primitive, Debug, Clone)]
pub struct UriDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for uri
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Uri>,
}

#[derive(Primitive, Debug, Clone)]
pub struct OidDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for oid
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Oid>,
}

#[derive(Primitive, Debug, Clone)]
pub struct CanonicalDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for canonical
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Canonical>,
}

#[derive(Primitive, Debug, Clone)]
pub struct CodeDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for code
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Code>,
}

/// 布尔类型
///
/// true | false
#[derive(Primitive, Clone, Debug)]
pub struct BooleanDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for boolean
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Boolean>,
}

/// 正整数类型
///
/// 1..2,147,483,647
#[derive(Primitive, Debug, Clone)]
pub struct PositiveIntDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for positiveInt
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<PositiveInt>,
}


#[derive(Primitive, Clone, Debug)]
pub struct IntegerDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for integer
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Integer>,
}



#[derive(Primitive, Debug, Clone)]
pub struct Integer64Dt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for integer64
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Integer64>,
}

#[derive(Primitive, Debug, Clone)]
pub struct DecimalDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for decimal
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Decimal>,
}

/// 日期类型
///
/// 表示日期，包括年月日。格式为YYYY, YYYY-MM, YYYY-MM-DD。不要指定时区。
#[derive(Primitive, Debug, Clone)]
pub struct DateDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for date
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Date>,
}

/// 时间类型
///
/// 表示一天内的时间。24小时制，格式为HH::MM:SS。不要指定时区。
#[derive(Primitive, Clone, Debug)]
pub struct TimeDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for time
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Time>,
}

/// 日期与时间
///
/// 时间部分如果存在，必须到秒，如果精度不足，可以使用00填充。
/// 如果存在时间，则必须提供时区。如果不确定时区，可以使用Z表示当地时区。
///
/// 精度不限，展示形式灵活。可以表示如下几种格式：
/// * 年 - 2023
/// * 年月 - 2023-08
/// * 年月日 - 2023-08-17
/// * 年月日时分秒 - 2023-08-17T08:21:45Z
/// * 年月日时分秒(毫秒) - 2023-08-17T08:21:45.234Z
/// * 年月日时分秒(毫秒)时区 - 2023-08-17T08:21:45.234+08:00
#[derive(Primitive, Debug, Clone)]
pub struct DateTimeDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for dateTime
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<DateTime>,
}

/// 日期与时间
///
/// 精度更高，最小精度要到秒，必须有时区。一般这种时间都是由机器生成的。
///
/// 只接受FHIR规范约定的四种格式：
/// * 2009-12-24T07:12:45Z
/// * 2009-12-24T07:12:45.234Z
/// * 2009-12-24T07:12:45+08:00
/// * 2009-12-24T07:12:45.234+08:00
///
/// 输出时，只有两种：
/// * 2009-12-24T07:12:45+08:00
/// * 2009-12-24T07:12:45.234+08:00
///
/// 导入之后，会根据当地的时区将Z转换为对应的时区
#[derive(Primitive, Debug, Clone)]
pub struct InstantDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for instant
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Instant>,
}

#[derive(Primitive, Debug, Clone)]
pub struct UnsignedIntDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for unsignedInt
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<UnsignedInt>,
}

#[derive(Primitive, Debug, Clone)]
pub struct UrlDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for url
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Url>,
}

#[derive(Primitive, Debug, Clone)]
pub struct UuidDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for uuid
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<Uuid>,
}

// TODO 这里的extension的基数为0..0,需要特殊处理一下
#[derive(Primitive, Debug, Clone)]
pub struct XhtmlDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="0", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Actual xhtml
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false)]
    pub value: Option<Xhtml>,
}

// macro_rules! primitive_fromstr_impl {
//     ($primitive:ident, $inner:ident) => {
//         impl FromStr for $primitive {
//             type Err = FhirError;
        
//             fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
//                 let val = $inner::from_str(s)?;
//                 Ok($primitive {
//                     id: None,
//                     extension: None,
//                     value: Some(val),
//                 })
//             }
//         }
//     }
// }

// primitive_fromstr_impl!(StringDt, String);
// primitive_fromstr_impl!(IdDt, Id);
// primitive_fromstr_impl!(Base64BinaryDt, Base64Binary);
// primitive_fromstr_impl!(MarkdownDt, Markdown);
// primitive_fromstr_impl!(UriDt, Uri);
// primitive_fromstr_impl!(UrlDt, Url);
// primitive_fromstr_impl!(OidDt, Oid);
// primitive_fromstr_impl!(CanonicalDt, Canonical);
// primitive_fromstr_impl!(CodeDt, Code);
// primitive_fromstr_impl!(UuidDt, Uuid);
// primitive_fromstr_impl!(XhtmlDt, Xhtml);

// primitive_fromstr_impl!(BooleanDt, Boolean);

// primitive_fromstr_impl!(UnsignedIntDt, UnsignedInt);
// primitive_fromstr_impl!(DecimalDt, Decimal);
// primitive_fromstr_impl!(IntegerDt, Integer);
// primitive_fromstr_impl!(Integer64Dt, Integer64);
// primitive_fromstr_impl!(PositiveIntDt, PositiveInt);

// primitive_fromstr_impl!(InstantDt, Instant);
// primitive_fromstr_impl!(DateTimeDt, DateTime);
// primitive_fromstr_impl!(DateDt, Date);
// primitive_fromstr_impl!(TimeDt, Time);

macro_rules! primitive_from_impl {
    ($primitive:ident) => {
        impl From<&str> for $primitive {
            fn from(value: &str) -> Self {
                $primitive::from_str(value).unwrap()
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

primitive_from_impl!(InstantDt);
primitive_from_impl!(DateTimeDt);
primitive_from_impl!(DateDt);
primitive_from_impl!(TimeDt);

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

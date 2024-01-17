pub mod primitive;
pub mod complex;

pub use primitive::*;
pub use complex::*;
use crate::prelude::*;

use std::fmt::{Display, Formatter};
use std::str::FromStr;
pub type Id = String;
pub type Code = String;
pub type Markdown = String;

pub type Base64Binary = String;
pub type Xhtml = String;
pub type Boolean = bool;
pub type Uri = String;
pub type Url = String;
pub type Oid = String;
pub type Uuid = String;
pub type Canonical = String;
pub type PositiveInt = usize;
pub type UnsignedInt = usize;
pub type Decimal = f64;
pub type Integer = isize;
pub type Integer64 = i64;

pub trait Element {
    fn has_id(&self) -> bool;
    fn id(&self) -> &Option<String>;
    fn set_id(self, id: String) -> Self;
    fn has_extension(&self) -> bool;
    fn extension(&self) -> &Option<Vec<Extension>>;
    fn set_extension(self, ext: Vec<Extension>) -> Self;
    fn add_extension(self, ext: Extension) -> Self;
}

/// FHIR简单类型的特性
/// FHIR简单类型是RUST简单数据类型的包装器
///
pub trait Primitive {
    type T;
    fn new<A: Into<Self::T>>(v: A) -> Self;
    fn value(&self) -> &Option<Self::T>;
    fn set_value(self, v: Self::T) -> Self;
}

#[derive(Clone, Debug)]
pub struct Date(chrono::NaiveDate, usize);

impl FromStr for Date {
    type Err = FhirError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let precision = s.len();
        let val = match precision {
            4 => {format!("{}-01-01", s)},
            7 => {format!("{}-01", s)},
            10 => {format!("{}", s)},
            _ => {return Err(FhirError::error("错误的日期格式，只接受YYYY,YYYY-MM,YYYY-MM-DD三种格式"));}
        };
        let val = chrono::NaiveDate::from_str(val.as_str())?;
        Ok(Date(val, precision))
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            4 => {write!(f, "{}", self.0.format("%Y"))},
            7 => {write!(f, "{}", self.0.format("%Y-%m"))},
            10 => {write!(f, "{}", self.0.format("%Y-%m-%d"))},
            _ => {write!(f, "Error")}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Time(chrono::NaiveTime, usize);

impl Display for Time {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            8 => {write!(f, "{}", self.0.format("%H:%M:%S"))},
            12 => {write!(f, "{}", self.0.format("%H:%M:%S.%3f"))},
            _ => {write!(f, "Error")}
        }
    }
}

impl FromStr for Time {
    type Err = FhirError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let precision = s.len();
        let val = match precision {
            8 => {chrono::NaiveTime::parse_from_str(s, "%H:%M:%S")?},
            12 => {chrono::NaiveTime::parse_from_str(s, "%H:%M:%S.%3f")?},
            _ => {return Err(FhirError::error("错误的时间格式，只接受HH::MM::SS,HH:MM:SS.sss两种格式"));}
        };
        Ok(Time(val, precision))
    }
}

/// 日期与时间
///
/// 精度不限，展示形式灵活.
///
/// 可以表示：
/// * 年 - 2023
/// * 年月 - 2023-08
/// * 年月日 - 2023-08-17
/// * 年月日时分秒 - 2023-08-17T08:21:45
/// * 年月日时分秒(毫秒) - 2023-08-17T08:21:45.234
/// * 年月日时分秒(毫秒)时区 - 2023-08-17T08:21:45.234+08:00
#[derive(Clone, Debug)]
pub struct DateTime(chrono::DateTime<chrono::FixedOffset>, usize);

impl Display for DateTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            4 => {write!(f, "{}", self.0.format("%Y"))},
            7 => {write!(f, "{}", self.0.format("%Y-%m"))},
            10 => {write!(f, "{}", self.0.format("%Y-%m-%d"))},
            20|25 => {write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S%:z"))},
            24|29 => {write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S.%3f%:z"))},
            _ => {write!(f, "Error")}
        }
    }
}

impl FromStr for DateTime {
    type Err = FhirError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let pattern = "0000-01-01T00:00:00.000Z";
        let precision = s.len();
        let val = match precision {
            4|7|10|24 => {
                let dt = format!("{}{}", s, &pattern[precision..]);
                chrono::DateTime::<chrono::Local>::from_str(dt.as_str())?.into()
            },
            20 => {
                let dt = format!("{}{}", &s[0..precision-1], &pattern[precision-1..]);
                chrono::DateTime::<chrono::Local>::from_str(dt.as_str())?.into()
            },
            25|29 => {
                chrono::DateTime::from_str(s)?
            },
            _ => {return Err(FhirError::error("错误的时间格式，只接受FHIR规范约定的日期时间格式"));}
        };
        Ok(DateTime(val, precision))
    }
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
/// 导入之后，会根据当地的时区将Z转换为对应的时区
#[derive(Clone, Debug)]
pub struct Instant(chrono::DateTime<chrono::FixedOffset>, usize);

impl Display for Instant {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            20|25 => {write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S%:z"))},
            24|29 => {write!(f, "{}", self.0.format("%Y-%m-%dT%H:%M:%S.%3f%:z"))},
            _ => {write!(f, "Error")}
        }
    }
}

impl FromStr for Instant {
    type Err = FhirError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let precision = s.len();
        let val = match precision {
            20|24 => {
                chrono::DateTime::<chrono::Local>::from_str(s)?.into()
            },
            25|29 => {
                chrono::DateTime::from_str(s)?
            },
            _ => {return Err(FhirError::error("错误的时间格式，只接受FHIR规范约定的日期时间格式"));}
        };
        Ok(Instant(val, precision))
    }
}

#[derive(Extension, Debug, Clone)]
pub struct Extension {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// identifies the meaning of the extension
    #[fhir(name="url", min="1", max="1", summary="false", modifier="false")]
    pub url: Option<Uri>,
    /// Value of extension
    #[fhir(name="value", min="0", max="1", summary="false", modifier="false")]
    pub value: Option<Any>,
}

impl Serialize for Extension {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        let mut extension  = serializer.serialize_extension()?;
        extension.serialize_id(&self.id)?;
        extension.serialize_url(&self.url)?;
        extension.serialize_extension(&self.extension)?;
        extension.serialize_value(&self.value)?;
        extension.serialize_end()
    }
}

impl<'de> Deserialize<'de> for Extension {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {

        pub struct ExtensionVisitor;
        impl<'de> Visitor<'de> for ExtensionVisitor {
            type Value = Extension;

            fn visit_map<V>(self, mut map: V) -> Result<Extension>
                where
                    V: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut url : Option<String> = None;
                let mut extension: Option<Vec<Extension>> = None;
                let mut value: Option<Any> = None;

                while let Some(key) = map.next_key()? {
                    match key.as_str() {
                        "id" => {
                            id = Some(map.next_value()?);
                            tracing::debug!("读取到值: {:?}", &id);
                        },
                        "url" => {
                            url = Some(map.next_value()?);
                            tracing::debug!("读取到值: {:?}", &url);
                        }
                        "extension" => {
                            extension = Some(map.next_value()?);
                            tracing::debug!("读取到值: {:?}", &url);
                        }
                        "valueString" => {
                            let temp: StringDt = map.next_value()?;
                            value = Some(Any::String(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        "valuePositiveInt" => {
                            let temp: PositiveIntDt = map.next_value()?;
                            value = Some(Any::PositiveInt(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        _ => {return Err(FhirError::error("Extension读到不存在的key了"));},
                    }
                }

                Ok(Extension {
                    id,
                    url,
                    extension,
                    value,
                })
            }
        }

        deserializer.deserialize_struct("Extension", ExtensionVisitor)
    }
}

#[derive(Clone, Debug)]
pub enum Any {
    PositiveInt(PositiveIntDt),
    String(StringDt),
    Coding(Coding),
}

impl Serialize for Any {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        match self {
            Any::PositiveInt(value) => {
                serializer.serialize_any("valuePositiveInt", value)
            }
            Any::String(value) => {
                serializer.serialize_any("valueString", value)
            }
            Any::Coding(value) => {
                serializer.serialize_any("valueCoding", value)
            }
        }
    }
}

impl<'de> Deserialize<'de> for Any {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
        
        pub struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = Any;
        }
        
        deserializer.deserialize_enum(AnyVisitor)
    }
}
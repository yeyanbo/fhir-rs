use std::str::FromStr;
use crate::datatype::*;

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

impl From<&str> for StringDt {
    fn from(value: &str) -> Self {
        StringDt::from_str(value).unwrap()
    }
}

impl FromStr for StringDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(StringDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl From<&str> for IdDt {
    fn from(value: &str) -> Self {
        IdDt::from_str(value).unwrap()
    }
}

impl FromStr for IdDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(IdDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for Base64BinaryDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(Base64BinaryDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for MarkdownDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(MarkdownDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for UriDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(UriDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for OidDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(OidDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for CanonicalDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(CanonicalDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl From<&str> for CodeDt {
    fn from(value: &str) -> Self {
        CodeDt::from_str(value).unwrap()
    }
}

impl FromStr for CodeDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = String::from_str(s)?;
        Ok(CodeDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for BooleanDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = bool::from_str(s)?;
        Ok(BooleanDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for PositiveIntDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = usize::from_str(s)?;
        Ok(PositiveIntDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for IntegerDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = isize::from_str(s)?;
        Ok(IntegerDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for Integer64Dt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Integer64::from_str(s)?;
        Ok(Integer64Dt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for DecimalDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = f64::from_str(s)?;
        Ok(DecimalDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl Display for DateDt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            None => {write!(f, "")}
            Some(val) => {write!(f, "{}", val)}
        }
    }
}

impl FromStr for DateDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Date::from_str(s)?;
        Ok(DateDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl Display for TimeDt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            None => {write!(f, "")}
            Some(val) => {write!(f, "{}", val)}
        }
    }
}

impl FromStr for TimeDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Time::from_str(s)?;
        Ok(TimeDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
}

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

impl Display for DateTimeDt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            None => {write!(f, "")}
            Some(val) => {write!(f, "{}", val)}
        }
    }
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
impl FromStr for DateTimeDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = DateTime::from_str(s)?;
        Ok(DateTimeDt {
            id: None,
            extension: None,
            value: Some(val),
        })
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

impl Display for InstantDt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            None => {write!(f, "")}
            Some(val) => {write!(f, "{}", val)}
        }
    }
}

impl FromStr for InstantDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Instant::from_str(s)?;
        Ok(InstantDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for UnsignedIntDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = UnsignedInt::from_str(s)?;
        Ok(UnsignedIntDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for UrlDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Url::from_str(s)?;
        Ok(UrlDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for UuidDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Uuid::from_str(s)?;
        Ok(UuidDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
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

impl FromStr for XhtmlDt {
    type Err = FhirError;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let val = Xhtml::from_str(s)?;
        Ok(XhtmlDt {
            id: None,
            extension: None,
            value: Some(val),
        })
    }
}

#[cfg(test)]
mod tests {
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

}

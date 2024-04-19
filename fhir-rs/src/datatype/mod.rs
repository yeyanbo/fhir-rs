pub mod primitive;
pub mod complex;

pub use primitive::*;
pub use complex::*;
use crate::prelude::*;

use std::fmt::{Display, Debug, Formatter};
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

pub trait Base {
    fn is_empty(&self) -> bool {
        false
    }
}

impl Base for String {}
impl Base for Boolean {}
impl Base for PositiveInt {}
impl Base for Decimal {}
impl Base for Integer {}
impl Base for Integer64 {}
impl Base for DateTime {}
impl Base for Date {}
impl Base for Time {}
impl Base for Instant {}
impl<T> Base for Option<T> {}
impl<T> Base for Vec<T> {}
impl<T> Base for Box<T> {}
impl Base for AnyType {}

pub trait Element {
    fn has_id(&self) -> bool;
    fn id(&self) -> &Option<String>;
    fn set_id<T: Into<String>>(self, id: T) -> Self;
    fn has_extension(&self) -> bool;
    fn extension(&self) -> &Option<Vec<Extension>>;
    fn set_extension(self, ext: Vec<Extension>) -> Self;
    fn add_extension(self, ext: Extension) -> Self;
}

/// FHIR简单类型的特性
/// FHIR简单类型是RUST简单数据类型的包装器
///
pub trait Primitive: Display + FromStr {
    type T;
    fn new<A: Into<Self::T>>(v: A) -> Self;
    fn value(&self) -> &Option<Self::T>;
    fn set_value(self, v: Self::T) -> Self;
}

pub trait Resource {
    fn resource_name(&self) -> String;
    fn id(&self) -> &Option<Id>;
    fn set_id<T: Into<Id>>(self, id: T) -> Self;
    fn meta(&self) -> &Option<Meta>;
    fn set_meta(self, meta: Meta) -> Self;
    fn assert(&self, path: String) -> Result<bool>;
    fn path(&self, path: String) -> Result<Collection>;
}

pub trait DomainResource: Resource {
    fn extension(&self) -> &Option<Vec<Extension>>;
    fn set_extension(self, ext: Vec<Extension>) -> Self;
    fn add_extension(self, ext: Extension) -> Self;
    fn modifier_extension(&self) -> &Option<Vec<Extension>>;
    fn set_modifier_extension(self, ext: Vec<Extension>) -> Self;
    fn add_modifier_extension(self, ext: Extension) -> Self;
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

#[derive(Extension, Debug, Clone, Default)]
pub struct Extension {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// identifies the meaning of the extension
    #[fhir(name="url", min="1", max="1", summary=false, modifier=false)]
    pub url: Option<Uri>,
    /// Value of extension
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<AnyType>,
}

impl Extension {
    pub fn new<U: Into<Url>>(url: U, value: AnyType) -> Extension {
        Extension {
            id: None,
            extension: None,
            url: Some(url.into()),
            value: Some(value),
        }
    }

    pub fn with_url<U: Into<Url>>(url: U) -> Extension {
        Extension {
            id: None,
            extension: None,
            url: Some(url.into()),
            value: None,
        }
    }
}

impl Executor for Extension {
    fn as_collection(&self) -> Collection {
        Collection(vec![Box::new(self.clone())])
    }
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
                let mut value: Option<AnyType> = None;

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
                            value = Some(AnyType::String(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        // b 
                        // if b.starts_with("value") => {
                        //     tracing::warn!("hahahahahaha");
                        // }
                        // valueBase64Binary
                        "valueBoolean" => {
                            let temp: BooleanDt = map.next_value()?;
                            value = Some(AnyType::Boolean(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        // valueCanonical
                        "valueCode" => {
                            let temp: CodeDt = map.next_value()?;
                            value = Some(AnyType::Code(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        // valueDate
                        "valueDateTime" => {
                            let temp: DateTimeDt = map.next_value()?;
                            value = Some(AnyType::DateTime(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        // valueDecimal
                        // valueId
                        // valueInstant: instant
                        // valueInteger: integer
                        // valueInteger64: integer64
                        // valueMarkdown: markdown
                        // valueOid: oid
                        // valuePositiveInt: positiveInt
                        // valueString
                        // valueTime
                        // valueUnsignedInt
                        // valueUri
                        // valueUrl
                        // valueUuid: uuid
                        // valueAddress: Address
                        // valueAge: Age
                        // valueAnnotation: Annotation
                        // valueAttachment: Attachment
                        // valueCodeableConcept: CodeableConcept
                        // valueCodeableReference: CodeableReference
                        "valueCoding" => {
                            let temp: Coding = map.next_value()?;
                            value = Some(AnyType::Coding(temp));
                            tracing::debug!("读取到值: {:?}", &value);
                        }
                        // valueContactPoint: ContactPoint
                        // valueCount: Count
                        // valueDistance: Distance
                        // valueDuration: Duration
                        // valueHumanName: HumanName
                        // valueIdentifier: Identifier
                        // valueMoney: Money
                        // valuePeriod: Period
                        // valueQuantity: Quantity
                        // valueRange: Range
                        // valueRatio: Ratio
                        // valueRatioRange: RatioRange
                        // valueReference: Reference - a reference to another resource
                        // valueSampledData: SampledData
                        // valueSignature: Signature
                        // valueTiming: Timing
                        // valueContactDetail: ContactDetail
                        // valueDataRequirement: DataRequirement
                        // valueExpression: Expression
                        // valueParameterDefinition: ParameterDefinition
                        // valueRelatedArtifact: RelatedArtifact
                        // valueTriggerDefinition: TriggerDefinition
                        // valueUsageContext: UsageContext
                        // valueAvailability: Availability
                        // valueExtendedContactDetail: ExtendedContactDetail
                        // valueDosage: Dosage
                        // valueMeta: Meta
                        "valuePositiveInt" => {
                            let temp: PositiveIntDt = map.next_value()?;
                            value = Some(AnyType::PositiveInt(temp));
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
pub enum AnyType {
    // 原始类型
    String(StringDt),
    Id(IdDt),
    Base64Binary(Base64BinaryDt),
    Markdown(MarkdownDt),
    Uri(UriDt),
    Url(UrlDt),
    Oid(OidDt),
    Uuid(UuidDt),
    Canonical(CanonicalDt),
    Code(CodeDt),
    Boolean(BooleanDt),
    DateTime(DateTimeDt),
    Date(DateDt),
    Time(TimeDt),
    Instant(InstantDt),
    UnsignedInt(UnsignedIntDt),
    PositiveInt(PositiveIntDt),
    Integer(IntegerDt),
    Integer64(Integer64Dt),
    Decimal(DecimalDt),
    // 复合类型
    Address(Address),
    Age(Age),
    Annotation(Annotation),
    Attachment(Attachment),
    CodeableConcept(CodeableConcept),
    CodeableReference(CodeableReference),
    Coding(Coding),
    ContactPoint(ContactPoint),
    Count(Count),
    Distance(Distance),
    Duration(Duration),
    HumanName(HumanName),
    Identifier(Identifier),
    Money(Money),
    Period(Period),
    Quantity(Quantity),
    Range(Range),
    Ratio(Ratio),
    RatioRange(RatioRange),
    Reference(Reference),
    SampledData(SampledData),
    Signature(Signature),
    Timing(Timing),
    ContactDetail(ContactDetail),
    DataRequirement(DataRequirement),
    Expression(Expression),
    ParameterDefinition(ParameterDefinition),
    RelatedArtifact(RelatedArtifact),
    TriggerDefinition(TriggerDefinition),
    UsageContext(UsageContext),
    Availability(Availability),
    ExtendedContactDetail(ExtendedContactDetail),
    Dosage(Dosage),
    Meta(Meta),
}

impl Serialize for AnyType {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        match self {
            AnyType::PositiveInt(value) => {
                serializer.serialize_any("valuePositiveInt", value)
            }
            AnyType::String(value) => {
                serializer.serialize_any("valueString", value)
            }
            AnyType::Coding(value) => {
                serializer.serialize_any("valueCoding", value)
            }
            AnyType::DateTime(value) => {
                serializer.serialize_any("valueDateTime", value)
            }
            AnyType::Base64Binary(value) => {
                serializer.serialize_any("valueBase64Binary", value)
            }
            AnyType::Boolean(value) => {
                serializer.serialize_any("valueBool", value)
            }
            AnyType::Code(value) => {
                serializer.serialize_any("valueCode", value)
            }
            AnyType::Id(value) => {
                serializer.serialize_any("valueId", value)
            }
            AnyType::Markdown(value) => {
                serializer.serialize_any("valueMarkdown", value)
            }
            AnyType::Uri(value) => {
                serializer.serialize_any("valueUri", value)
            }
            AnyType::Url(value) => {
                serializer.serialize_any("valueUrl", value)
            }
            AnyType::Uuid(value) => {
                serializer.serialize_any("valueUuid", value)
            }
            AnyType::Oid(value) => {
                serializer.serialize_any("valueOid", value)
            }
            AnyType::Canonical(value) => {
                serializer.serialize_any("valueCanonical", value)
            }
            AnyType::Date(value) => {
                serializer.serialize_any("valueDate", value)
            }
            AnyType::Time(value) => {
                serializer.serialize_any("valueTime", value)
            }
            AnyType::Instant(value) => {
                serializer.serialize_any("valueInstant", value)
            }
            AnyType::UnsignedInt(value) => {
                serializer.serialize_any("valueUnsignedInt", value)
            }
            AnyType::Integer(value) => {
                serializer.serialize_any("valueInteger", value)
            }
            AnyType::Integer64(value) => {
                serializer.serialize_any("valueInteger64", value)
            }
            AnyType::Decimal(value) => {
                serializer.serialize_any("valueDecimal", value)
            }
            AnyType::Address(value) => {
                serializer.serialize_any("valueAddress", value)
            }
            AnyType::Age(value) => {
                serializer.serialize_any("valueAge", value)
            }
            AnyType::Annotation(value) => {
                serializer.serialize_any("valueAnnotation", value)
            }
            AnyType::Attachment(value) => {
                serializer.serialize_any("valueAttachment", value)
            }
            AnyType::CodeableConcept(value) => {
                serializer.serialize_any("valueCodeableConcept", value)
            }
            AnyType::CodeableReference(value) => {
                serializer.serialize_any("valueCodeableReference", value)
            }
            AnyType::ContactPoint(value) => {
                serializer.serialize_any("valueContactPoint", value)
            }
            AnyType::Count(value) => {
                serializer.serialize_any("valueCount", value)
            }
            AnyType::Distance(value) => {
                serializer.serialize_any("valueDistance", value)
            }
            AnyType::Duration(value) => {
                serializer.serialize_any("valueDuration", value)
            }
            AnyType::HumanName(value) => {
                serializer.serialize_any("valueHumanName", value)
            }
            AnyType::Identifier(value) => {
                serializer.serialize_any("valueIdentifier", value)
            }
            AnyType::Money(value) => {
                serializer.serialize_any("valueMoney", value)
            }
            AnyType::Period(value) => {
                serializer.serialize_any("valuePeriod", value)
            }
            AnyType::Quantity(value) => {
                serializer.serialize_any("valueQuantity", value)
            }
            AnyType::Range(value) => {
                serializer.serialize_any("valueRange", value)
            }
            AnyType::Ratio(value) => {
                serializer.serialize_any("valueRatio", value)
            }
            AnyType::RatioRange(value) => {
                serializer.serialize_any("valueRatioRange", value)
            }
            AnyType::Reference(value) => {
                serializer.serialize_any("valueReference", value)
            }
            AnyType::SampledData(value) => {
                serializer.serialize_any("valueSampledData", value)
            }
            AnyType::Signature(value) => {
                serializer.serialize_any("valueSignature", value)
            }
            AnyType::Timing(value) => {
                serializer.serialize_any("valueTiming", value)
            }
            AnyType::ContactDetail(value) => {
                serializer.serialize_any("valueContactDetail", value)
            }
            AnyType::DataRequirement(value) => {
                serializer.serialize_any("valueDataRequirement", value)
            }
            AnyType::Expression(value) => {
                serializer.serialize_any("valueExpression", value)
            }
            AnyType::ParameterDefinition(value) => {
                serializer.serialize_any("valueParameterDefinition", value)
            }
            AnyType::RelatedArtifact(value) => {
                serializer.serialize_any("valueRelatedArtifact", value)
            }
            AnyType::TriggerDefinition(value) => {
                serializer.serialize_any("valueTriggerDefinition", value)
            }
            AnyType::UsageContext(value) => {
                serializer.serialize_any("valueUsageContext", value)
            }
            AnyType::Availability(value) => {
                serializer.serialize_any("valueAvailability", value)
            }
            AnyType::ExtendedContactDetail(value) => {
                serializer.serialize_any("valueExtendedContactDetail", value)
            }
            AnyType::Dosage(value) => {
                serializer.serialize_any("valueDosage", value)
            }
            AnyType::Meta(value) => {
                serializer.serialize_any("valueMeta", value)
            }
        }
    }
}

impl<'de> Deserialize<'de> for AnyType {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
        
        pub struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = AnyType;
        }
        
        deserializer.deserialize_enum(AnyVisitor)
    }
}
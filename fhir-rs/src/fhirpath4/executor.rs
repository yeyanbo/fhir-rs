use crate::{impl_executor_for_anytype, impl_executor_for_primitive};
use super::*;
use crate::prelude::*;

pub trait Executor: Base + Convert + Compare {

    fn child(&self, _index: usize) -> Result<Collection> {
        Err(FhirError::error("该数据类型不支持获取子元素"))
    }

    fn element(&self, _symbol: &String, _index: &Option<usize>) -> crate::prelude::Result<Collection> {
        Err(FhirError::error("该数据类型不支持获取子路径"))
    }

    fn to_collection(&self, index: &Option<usize>) -> Collection;
}

pub trait Convert {
    fn to_integer(&self) -> Result<Integer> {
        Err(FhirError::error("该数据类型不能转换为整数"))
    }

    fn to_decimal(&self) -> Result<Decimal> {
        Err(FhirError::error("该数据类型不能转换为数值"))
    }

    fn to_strings(&self) -> Result<String> {
        Err(FhirError::error("该数据类型不能转换为字符串"))
    }

    fn to_datetime(&self) -> Result<DateTime> {
        Err(FhirError::error("该数据类型不能转换为日期和时间"))
    }

    fn to_boolean(&self) -> Result<Boolean> {
        Err(FhirError::error("该数据类型不能转换为布尔"))
    }
}

pub trait Compare {

    fn eq(&self, _right: &dyn Executor) -> Result<bool> {
        Err(FhirError::error("该数据类型不支持比较运算符[==]"))
    }

    fn gt(&self, _right: &dyn Executor) -> Result<bool> {
        Err(FhirError::error("该数据类型不支持比较运算符[>]"))
    }

    fn ge(&self, _right: &dyn Executor) -> Result<bool> {
        Err(FhirError::error("该数据类型不支持比较运算符[>=]"))
    }

    fn lt(&self, _right: &dyn Executor) -> Result<bool> {
        Err(FhirError::error("该数据类型不支持比较运算符[<]"))
    }

    fn le(&self, _right: &dyn Executor) -> Result<bool> {
        Err(FhirError::error("该数据类型不支持比较运算符[<=]"))
    }
}

macro_rules! impl_executor {
    (
        $($ty:ident,)+
    ) => {
        $(
            impl Executor for $ty {
                fn to_collection(&self, _index: &Option<usize>) -> Collection {
                    Collection::new_any(Box::new(self.clone()))
                }
            }
        )+
    }
}

macro_rules! impl_convert {
    (
        $($ty:ident,)+
    ) => {
        $(
            impl Convert for $ty {}
        )+
    }
}

macro_rules! impl_compare {
    (
        $($ty:ident,)+
    ) => {
        $(
            impl Compare for $ty {}
        )+
    }
}

impl_executor!(usize, isize, i64, f64, bool, Instant, DateTime, Time, Date, String, Xhtml,);
impl_convert!(usize, isize, i64, f64, Instant, DateTime, Time, Date,);
impl_compare!(usize, isize, i64, f64, bool, Instant, DateTime, Time, Date,);

impl Compare for String {
    fn eq(&self, right: &dyn Executor) -> Result<bool> {
        let rhs = right.to_strings()?;
        Ok(*self == rhs)
    }
}
impl Convert for String {

    fn to_strings(&self) -> Result<String> {
        Ok(self.clone())
    }
}

impl Compare for Xhtml {
    fn eq(&self, right: &dyn Executor) -> Result<bool> {
        let rhs = right.to_strings()?;
        Ok(*self.0 == rhs)
    }
}
impl Convert for Xhtml {

    fn to_strings(&self) -> Result<String> {
        Ok(self.0.clone())
    }
}

impl Convert for Boolean {
    fn to_boolean(&self) -> Result<Boolean> {
        Ok(*self)
    }
}


impl<T: Executor> Executor for Box<T> {
    fn to_collection(&self, index: &Option<usize>) -> Collection {
        self.as_ref().to_collection(index)
    }
}

impl<T: Executor> Convert for Box<T> {}
impl<T: Executor> Compare for Box<T> {}

impl<T: Executor> Executor for Option<T> {

    fn to_collection(&self, index: &Option<usize>) -> Collection {
        match self {
            None => Collection::new(),
            Some(value) => value.to_collection(index),
        }
    }
}

impl<T: Executor> Convert for Option<T> {}
impl<T: Executor> Compare for Option<T> {}

impl<T: Executor> Executor for Vec<T> {
    fn to_collection(&self, index: &Option<usize>) -> Collection {
        match index {
            Some(idx) => {
                match self.get(*idx) {
                    Some(val) => val.to_collection(index),
                    None => Collection::new(),
                }
            },
            None => {
                let mut collection = Collection::new();
                for part in self{
                    collection.combine(part.to_collection(index));
                }
                collection
            }
        }
    }
}

impl<T: Executor> Convert for Vec<T> {}
impl<T: Executor> Compare for Vec<T> {}

impl_executor_for_primitive!{
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
    XhtmlDt,
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

impl_executor_for_anytype!{
    Base64Binary,
    Boolean,
    Canonical,
    Code,
    DateTime,
    Date,
    Decimal,
    Id,
    Instant,
    Integer,
    Integer64,
    Markdown,
    Oid,
    PositiveInt,
    String,
    Time,
    UnsignedInt,
    Uri,
    Url,
    Uuid,
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
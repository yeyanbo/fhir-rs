use fhir_derive::{Element, Primitive};
use crate::prelude::*;
use std::fmt::{Display, Formatter};
use chrono::Local;

/// 日期类型
///
/// 表示日期，包括年月日。格式为YYYY, YYYY-MM, YYYY-MM-DD。不要指定时区。
#[derive(Element, Primitive, Debug, Clone)]
pub struct DateDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for date
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Date>,
}

impl From<chrono::DateTime<Local>> for DateDt {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self::from(Date::from(value))
    }
}

/// 时间类型
///
/// 表示一天内的时间。24小时制，格式为HH::MM:SS。不要指定时区。
#[derive(Element, Primitive, Clone, Debug)]
pub struct TimeDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for time
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Time>,
}

impl From<chrono::DateTime<Local>> for TimeDt {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self::from(Time::from(value))
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
#[derive(Element, Primitive, Debug, Clone)]
pub struct DateTimeDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for dateTime
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<DateTime>,
}

impl From<chrono::DateTime<Local>> for DateTimeDt {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self::from(DateTime::from(value))
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
#[derive(Element, Primitive, Debug, Clone)]
pub struct InstantDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Primitive value for instant
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Instant>,
}

impl From<chrono::DateTime<Local>> for InstantDt {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self::from(Instant::from(value))
    }
}

#[test]
fn test_datetime() -> Result<()> {
    let d1 = DateDt::from(Local::now());
    println!("Date: {}", d1.to_string());

    let t1 = TimeDt::from(Local::now());
    println!("Time: {}", t1.to_string());

    let dt1 = DateTimeDt::from(Local::now());
    println!("DateTime: {}", dt1.to_string());

    let instance = InstantDt::from(Local::now());
    println!("Instant: {}", instance.to_string());

    Ok(())
}

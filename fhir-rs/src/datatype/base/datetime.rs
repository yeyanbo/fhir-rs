use std::fmt::{Display, Formatter};
use std::str::FromStr;
use chrono::{FixedOffset, Local};
use crate::error::FhirError;

#[derive(Clone, Debug)]
pub struct Date(chrono::NaiveDate, usize);

impl From<chrono::DateTime<Local>> for Date {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self(value.date_naive(), 10)
    }
}

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

impl From<chrono::DateTime<Local>> for Time {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self(value.time(), 12)
    }
}

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
pub struct DateTime(chrono::DateTime<FixedOffset>, usize);

impl From<chrono::DateTime<Local>> for DateTime {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self(value.into(), 29)
    }
}

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
pub struct Instant(chrono::DateTime<FixedOffset>, usize);

impl From<chrono::DateTime<Local>> for Instant {
    fn from(value: chrono::DateTime<Local>) -> Self {
        Self(value.into(), 29)
    }
}

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

#[test]
fn test_datetime() -> crate::prelude::Result<()> {
    let d1 = Date::from(Local::now());
    println!("Date: {}", d1.to_string());

    let t1 = Time::from(Local::now());
    println!("Time: {}", t1.to_string());

    let local = Local::now();

    let dt1 = DateTime::from(local);
    println!("DateTime: {}", dt1.to_string());

    let instance = Instant::from(local);
    println!("Instant: {}", instance.to_string());

    Ok(())
}

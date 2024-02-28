use std::fmt;
use std::fmt::{Debug, Display};
use std::string::FromUtf8Error;

pub enum FhirError {
    Message(String),
    UnImplementation(String),
    EndArrayWhileParsingList,
    FromStrError(String),
}

impl FhirError {
    pub fn error(msg: &str) -> Self {
        FhirError::Message(String::from(msg))
    }

    pub fn error_string(msg: String) -> Self {
        FhirError::Message(msg)
    }

    pub fn un_implementation(msg: &str) -> Self {
        FhirError::UnImplementation(String::from(msg))
    }
}

impl From<std::io::Error> for FhirError {
    fn from(value: std::io::Error) -> Self {
        FhirError::Message(value.to_string())
    }
}

impl From<xml::reader::Error> for FhirError {
    fn from(value: xml::reader::Error) -> Self {
        FhirError::Message(value.to_string())
    }
}

impl From<xml::writer::Error> for FhirError {
    fn from(value: xml::writer::Error) -> Self {
        FhirError::Message(value.to_string())
    }
}

impl From<core::convert::Infallible> for FhirError {
    fn from(value: core::convert::Infallible) -> Self {
        FhirError::FromStrError(value.to_string())
    }
}

impl From<std::num::ParseIntError> for FhirError {
    fn from(value: std::num::ParseIntError) -> Self {
        FhirError::FromStrError(value.to_string())
    }
}

impl From<std::num::ParseFloatError> for FhirError {
    fn from(value: std::num::ParseFloatError) -> Self {
        FhirError::FromStrError(value.to_string())
    }
}

impl From<std::str::ParseBoolError> for FhirError {
    fn from(value: std::str::ParseBoolError) -> Self {
        FhirError::FromStrError(value.to_string())
    }
}

impl From<chrono::format::ParseError> for FhirError {
    fn from(value: chrono::format::ParseError) -> Self {
        FhirError::FromStrError(value.to_string())
    }
}

impl From<FromUtf8Error> for FhirError {
    fn from(value: FromUtf8Error) -> Self {
        FhirError::FromStrError(value.to_string())
    }
}

impl Debug for FhirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FhirError::Message(msg) => write!(f, "错误消息: {:?}", msg),
            FhirError::UnImplementation(func) => write!(f, "函数未实现: {:?}", func),
            FhirError::EndArrayWhileParsingList => write!(f, "解析数组时遇到数组结束符号"),
            FhirError::FromStrError(msg) => write!(f, "字符串转换错误: {:?}", msg),
        }
    }
}

impl Display for FhirError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}


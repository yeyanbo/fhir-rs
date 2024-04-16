//! # 概述
//!
//! FHIR资源的序列化和反序列化解析器
//!
//! # 设计
//!
//! 设计思路参考了Serde的理念。
//!
pub mod error;
pub mod datatype;
pub mod de;
pub mod ser;

#[cfg(features = "server")]
mod server;
#[cfg(features = "client")]
mod client;
#[cfg(features = "validate")]
mod validate;
// #[cfg(features = "fhirpath")]
pub mod fhirpath;

pub mod prelude {
    pub use super::de::*;
    pub use super::ser::*;
    pub use super::error::*;
    pub use super::datatype::*;
    pub use super::fhirpath::*;
    pub use fhir_derive::{Extension, BackboneElement, Complex, Primitive, Resource};
    pub type Result<T> = std::result::Result<T, FhirError>;
}
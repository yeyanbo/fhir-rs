//! # 概述
//!
//! FHIR资源的序列化和反序列化解析器
//!
//! # 设计
//!
//! 设计思路参考了Serde的理念。
//!

mod error;
mod datatype;
mod de;
mod ser;
mod resource;
// pub mod expand;
// #[cfg(features = "server")]
mod server;
// #[cfg(features = "client")]
mod client;
// #[cfg(features = "validate")]
mod validate;
// #[cfg(features = "fhirpath")]
mod fhirpath4;

pub type Result<T> = std::result::Result<T, error::FhirError>;

pub mod prelude {
    pub use super::de::*;
    pub use super::ser::*;
    pub use super::error::*;
    pub use super::datatype::*;
    pub use super::fhirpath4::*;
    pub use super::resource::*;
    pub use super::validate::*;
    pub use super::Result;

    pub use fhir_derive::{Element, BackboneElement, Complex, Primitive, Resource};

    pub use tracing::{debug, info, error};
}
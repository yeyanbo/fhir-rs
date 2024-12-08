mod datetime;
mod xhtml;

use std::fmt::Debug;
pub use datetime::*;
pub use xhtml::Xhtml;

use crate::prelude::Base;

pub type Id = String;
pub type Code = String;
pub type Markdown = String;

pub type Base64Binary = String;

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

macro_rules! base_impl {
    (
        $($ty: ident,)+
    ) => {
        $(
            impl Base for $ty {
                fn type_name(&self) -> &str { stringify!($ty) }
            }
        )+
    };
}

base_impl!{String, Boolean, PositiveInt, Decimal, Integer, Integer64, DateTime, Date, Time, Instant, Xhtml,}

impl<T: Debug> Base for Option<T> {
    // fn is_empty(&self) -> bool {
    //     self.is_none()
    // }
    fn type_name(&self) -> &str {
        "Option"
    }
}
impl<T: Debug> Base for Vec<T> {
    // fn is_empty(&self) -> bool {
    //     self.is_empty()
    // }
    fn type_name(&self) -> &str {
        "Vec"
    }
}
impl<T: Debug> Base for Box<T> {
    fn type_name(&self) -> &str {
        "Box"
    }
}

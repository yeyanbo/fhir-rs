use std::fmt::Display;
use std::str::FromStr;
use crate::prelude::{Result, FhirError};

#[derive(Debug, Clone)]
pub struct Xhtml(pub(crate) String);


impl Display for Xhtml {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Xhtml {
    type Err = FhirError;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Self(String::from(s)))
    }
}
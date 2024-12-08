use fhir_derive::{Element, Primitive};
use crate::prelude::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

// TODO 这里的extension的基数为0..0,需要特殊处理一下
#[derive(Element, Primitive, Debug, Clone)]
pub struct XhtmlDt {
    /// xml:id (or equivalent in JSON)
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Actual xhtml
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Xhtml>,
}

impl Serialize for XhtmlDt {
    fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
        let mut primitive  = serializer.serialize_narrative()?;
        primitive.serialize_id(&self.id)?;
        primitive.serialize_xhtml(&self.value)?;
        primitive.serialize_end()
    }
}

impl<'de> Deserialize<'de> for XhtmlDt {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
        pub struct XhtmlDtVisitor;
        impl<'de> Visitor<'de> for XhtmlDtVisitor {
            type Value = XhtmlDt;

            fn visit_str(self, v: &str) -> Result<Self::Value> {
                XhtmlDt::from_str(v)
            }
        }

        deserializer.deserialize_narrative(XhtmlDtVisitor)
    }
}


use fhir_derive::Complex;
use crate::prelude::*;

#[derive(Element, Complex, Debug, Clone, Default)]
pub struct Narrative {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// generated | extensions | additional | empty
    #[fhir(name="status", min="1", max="1", summary=false, modifier=false, choice="")]
    pub status: Option<CodeDt>,
    /// Limited xhtml content
    #[fhir(name="div", min="1", max="1", summary=false, modifier=false, choice="")]
    pub div: Option<XhtmlDt>,
}
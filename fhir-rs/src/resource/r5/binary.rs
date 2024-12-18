use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="Resource")]
pub struct Binary {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice="")]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice="")]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice="")]
    pub language: Option<CodeDt>,
    /// MimeType of the binary content
    #[fhir(name="contentType", min="1", max="1", summary=true, modifier=false, choice="")]
    pub content_type: Option<CodeDt>,
    /// Identifies another resource to use as proxy when enforcing access control
    #[fhir(name="securityContext", min="0", max="1", summary=true, modifier=false, choice="")]
    pub security_context: Option<Reference>,
    /// The actual content
    #[fhir(name="data", min="0", max="1", summary=false, modifier=false, choice="")]
    pub data: Option<Base64BinaryDt>,
}


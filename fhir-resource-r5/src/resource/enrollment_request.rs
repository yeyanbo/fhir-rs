use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct EnrollmentRequest {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business Identifier
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Creation date
    #[fhir(name="created", min="0", max="1", summary=false, modifier=false)]
    pub created: Option<DateTimeDt>,
    /// Target
    #[fhir(name="insurer", min="0", max="1", summary=false, modifier=false)]
    pub insurer: Option<Reference>,
    /// Responsible practitioner
    #[fhir(name="provider", min="0", max="1", summary=false, modifier=false)]
    pub provider: Option<Reference>,
    /// The subject to be enrolled
    #[fhir(name="candidate", min="0", max="1", summary=false, modifier=false)]
    pub candidate: Option<Reference>,
    /// Insurance information
    #[fhir(name="coverage", min="0", max="1", summary=false, modifier=false)]
    pub coverage: Option<Reference>,
}


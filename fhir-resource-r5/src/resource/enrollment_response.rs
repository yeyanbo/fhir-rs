use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct EnrollmentResponse {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary="true", modifier="false")]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary="true", modifier="false")]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary="true", modifier="true")]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary="false", modifier="false")]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary="false", modifier="false")]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary="false", modifier="false")]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business Identifier
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="0", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Claim reference
    #[fhir(name="request", min="0", max="1", summary="false", modifier="false")]
    pub request: Option<Reference>,
    /// queued | complete | error | partial
    #[fhir(name="outcome", min="0", max="1", summary="false", modifier="false")]
    pub outcome: Option<CodeDt>,
    /// Disposition Message
    #[fhir(name="disposition", min="0", max="1", summary="false", modifier="false")]
    pub disposition: Option<StringDt>,
    /// Creation date
    #[fhir(name="created", min="0", max="1", summary="false", modifier="false")]
    pub created: Option<DateTimeDt>,
    /// Insurer
    #[fhir(name="organization", min="0", max="1", summary="false", modifier="false")]
    pub organization: Option<Reference>,
    /// Responsible practitioner
    #[fhir(name="requestProvider", min="0", max="1", summary="false", modifier="false")]
    pub request_provider: Option<Reference>,
}


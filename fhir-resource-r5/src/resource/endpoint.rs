use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Endpoint {
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
    /// Identifies this endpoint across multiple systems
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// active | suspended | error | off | entered-in-error | test
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Protocol/Profile/Standard to be used with this endpoint connection
    #[fhir(name="connectionType", min="1", max="*", summary=true, modifier=false)]
    pub connection_type: Option<Vec<CodeableConcept>>,
    /// A name that this endpoint can be identified by
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Additional details about the endpoint that could be displayed as further information to identify the description beyond its name
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<StringDt>,
    /// The type of environment(s) exposed at this endpoint
    #[fhir(name="environmentType", min="0", max="*", summary=true, modifier=false)]
    pub environment_type: Option<Vec<CodeableConcept>>,
    /// Organization that manages this endpoint (might not be the organization that exposes the endpoint)
    #[fhir(name="managingOrganization", min="0", max="1", summary=true, modifier=false)]
    pub managing_organization: Option<Reference>,
    /// Contact details for source (e.g. troubleshooting)
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false)]
    pub contact: Option<Vec<ContactPoint>>,
    /// Interval the endpoint is expected to be operational
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
    /// Set of payloads that are provided by this endpoint
    #[fhir(name="payload", min="0", max="*", summary=false, modifier=false)]
    pub payload: Option<Vec<EndpointPayloadBackboneElement>>,
    /// The technical base address for connecting to this endpoint
    #[fhir(name="address", min="1", max="1", summary=true, modifier=false)]
    pub address: Option<UrlDt>,
    /// Usage depends on the channel type
    #[fhir(name="header", min="0", max="*", summary=false, modifier=false)]
    pub header: Option<Vec<StringDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EndpointPayloadBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of content that may be used at this endpoint (e.g. XDS Discharge summaries)
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Mimetype to send. If not specified, the content could be anything (including no payload, if the connectionType defined this)
    #[fhir(name="mimeType", min="0", max="*", summary=true, modifier=false)]
    pub mime_type: Option<Vec<CodeDt>>,
}


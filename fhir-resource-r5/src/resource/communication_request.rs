use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CommunicationRequest {
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
    /// Unique identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan or proposal
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false)]
    pub based_on: Option<Vec<Reference>>,
    /// Request(s) replaced by this request
    #[fhir(name="replaces", min="0", max="*", summary=true, modifier=false)]
    pub replaces: Option<Vec<Reference>>,
    /// Composite request this is part of
    #[fhir(name="groupIdentifier", min="0", max="1", summary=true, modifier=false)]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary=false, modifier=false)]
    pub status_reason: Option<CodeableConcept>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary=true, modifier=true)]
    pub intent: Option<CodeDt>,
    /// Message category
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary=true, modifier=false)]
    pub priority: Option<CodeDt>,
    /// True if request is prohibiting action
    #[fhir(name="doNotPerform", min="0", max="1", summary=true, modifier=true)]
    pub do_not_perform: Option<BooleanDt>,
    /// A channel of communication
    #[fhir(name="medium", min="0", max="*", summary=false, modifier=false)]
    pub medium: Option<Vec<CodeableConcept>>,
    /// Focus of message
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false)]
    pub subject: Option<Reference>,
    /// Resources that pertain to this communication request
    #[fhir(name="about", min="0", max="*", summary=false, modifier=false)]
    pub about: Option<Vec<Reference>>,
    /// The Encounter during which this CommunicationRequest was created
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false)]
    pub encounter: Option<Reference>,
    /// Message payload
    #[fhir(name="payload", min="0", max="*", summary=false, modifier=false)]
    pub payload: Option<Vec<CommunicationRequestPayloadBackboneElement>>,
    /// When scheduled
    #[fhir(name="occurrence", min="0", max="1", summary=true, modifier=false)]
    pub occurrence: Option<Period>,
    /// When request transitioned to being actionable
    #[fhir(name="authoredOn", min="0", max="1", summary=true, modifier=false)]
    pub authored_on: Option<DateTimeDt>,
    /// Who asks for the information to be shared
    #[fhir(name="requester", min="0", max="1", summary=true, modifier=false)]
    pub requester: Option<Reference>,
    /// Who to share the information with
    #[fhir(name="recipient", min="0", max="*", summary=false, modifier=false)]
    pub recipient: Option<Vec<Reference>>,
    /// Who should share the information
    #[fhir(name="informationProvider", min="0", max="*", summary=true, modifier=false)]
    pub information_provider: Option<Vec<Reference>>,
    /// Why is communication needed?
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// Comments made about communication request
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CommunicationRequestPayloadBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Message part content
    #[fhir(name="content", min="1", max="1", summary=false, modifier=false)]
    pub content: Option<CodeableConcept>,
}


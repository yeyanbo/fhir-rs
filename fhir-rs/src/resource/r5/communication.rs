use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Communication {
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
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// Request fulfilled by this communication
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event (e.g. Communication, Procedure)
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// Reply to
    #[fhir(name="inResponseTo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub in_response_to: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary=true, modifier=false, choice="")]
    pub status_reason: Option<CodeableConcept>,
    /// Message category
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary=true, modifier=false, choice="")]
    pub priority: Option<CodeDt>,
    /// A channel of communication
    #[fhir(name="medium", min="0", max="*", summary=false, modifier=false, choice="")]
    pub medium: Option<Vec<CodeableConcept>>,
    /// Focus of message
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Description of the purpose/content
    #[fhir(name="topic", min="0", max="1", summary=false, modifier=false, choice="")]
    pub topic: Option<CodeableConcept>,
    /// Resources that pertain to this communication
    #[fhir(name="about", min="0", max="*", summary=false, modifier=false, choice="")]
    pub about: Option<Vec<Reference>>,
    /// The Encounter during which this Communication was created
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// When sent
    #[fhir(name="sent", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sent: Option<DateTimeDt>,
    /// When received
    #[fhir(name="received", min="0", max="1", summary=false, modifier=false, choice="")]
    pub received: Option<DateTimeDt>,
    /// Who the information is shared with
    #[fhir(name="recipient", min="0", max="*", summary=false, modifier=false, choice="")]
    pub recipient: Option<Vec<Reference>>,
    /// Who shares the information
    #[fhir(name="sender", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sender: Option<Reference>,
    /// Indication for message
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Message payload
    #[fhir(name="payload", min="0", max="*", summary=false, modifier=false, choice="")]
    pub payload: Option<Vec<CommunicationPayloadBackboneElement>>,
    /// Comments made about the communication
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CommunicationPayloadBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Message part content
    #[fhir(name="content", min="1", max="1", summary=false, modifier=false, choice="")]
    pub content: Option<CodeableConcept>,
}


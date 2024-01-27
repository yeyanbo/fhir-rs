use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MessageHeader {
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
    /// Event code or link to EventDefinition
    #[fhir(name="event", min="1", max="1", summary="true", modifier="false")]
    pub event: Option<CanonicalDt>,
    /// Message destination application(s)
    #[fhir(name="destination", min="0", max="*", summary="true", modifier="false")]
    pub destination: Option<Vec<MessageHeaderDestinationBackboneElement>>,
    /// Real world sender of the message
    #[fhir(name="sender", min="0", max="1", summary="true", modifier="false")]
    pub sender: Option<Reference>,
    /// The source of the decision
    #[fhir(name="author", min="0", max="1", summary="true", modifier="false")]
    pub author: Option<Reference>,
    /// Message source application
    #[fhir(name="source", min="1", max="1", summary="true", modifier="false")]
    pub source: Option<MessageHeaderSourceBackboneElement>,
    /// Final responsibility for event
    #[fhir(name="responsible", min="0", max="1", summary="true", modifier="false")]
    pub responsible: Option<Reference>,
    /// Cause of event
    #[fhir(name="reason", min="0", max="1", summary="true", modifier="false")]
    pub reason: Option<CodeableConcept>,
    /// If this is a reply to prior message
    #[fhir(name="response", min="0", max="1", summary="true", modifier="false")]
    pub response: Option<MessageHeaderResponseBackboneElement>,
    /// The actual content of the message
    #[fhir(name="focus", min="0", max="*", summary="true", modifier="false")]
    pub focus: Option<Vec<Reference>>,
    /// Link to the definition for this message
    #[fhir(name="definition", min="0", max="1", summary="true", modifier="false")]
    pub definition: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MessageHeaderResponseBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Bundle.identifier of original message
    #[fhir(name="identifier", min="1", max="1", summary="true", modifier="false")]
    pub identifier: Option<Identifier>,
    /// ok | transient-error | fatal-error
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeDt>,
    /// Specific list of hints/warnings/errors
    #[fhir(name="details", min="0", max="1", summary="true", modifier="false")]
    pub details: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MessageHeaderSourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Actual source address or Endpoint resource
    #[fhir(name="endpoint", min="0", max="1", summary="true", modifier="false")]
    pub endpoint: Option<Reference>,
    /// Name of system
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name of software running the system
    #[fhir(name="software", min="0", max="1", summary="true", modifier="false")]
    pub software: Option<StringDt>,
    /// Version of software running
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// Human contact for problems
    #[fhir(name="contact", min="0", max="1", summary="true", modifier="false")]
    pub contact: Option<ContactPoint>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MessageHeaderDestinationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Actual destination address or Endpoint resource
    #[fhir(name="endpoint", min="0", max="1", summary="true", modifier="false")]
    pub endpoint: Option<Reference>,
    /// Name of system
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Particular delivery destination within the destination
    #[fhir(name="target", min="0", max="1", summary="true", modifier="false")]
    pub target: Option<Reference>,
    /// Intended "real-world" recipient for the data
    #[fhir(name="receiver", min="0", max="1", summary="true", modifier="false")]
    pub receiver: Option<Reference>,
}


use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct MessageDefinition {
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
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The cannonical URL for a given MessageDefinition
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Business Identifier for a given MessageDefinition
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the message definition
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this message definition (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this message definition (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// Takes the place of
    #[fhir(name="replaces", min="0", max="*", summary="true", modifier="false")]
    pub replaces: Option<Vec<CanonicalDt>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="true", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="1", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the message definition
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for message definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this message definition is defined
    #[fhir(name="purpose", min="0", max="1", summary="true", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// Definition this one is based on
    #[fhir(name="base", min="0", max="1", summary="true", modifier="false")]
    pub base: Option<CanonicalDt>,
    /// Protocol/workflow this is part of
    #[fhir(name="parent", min="0", max="*", summary="true", modifier="false")]
    pub parent: Option<Vec<CanonicalDt>>,
    /// Event code  or link to the EventDefinition
    #[fhir(name="event", min="1", max="1", summary="true", modifier="false")]
    pub event: Option<UriDt>,
    /// consequence | currency | notification
    #[fhir(name="category", min="0", max="1", summary="true", modifier="false")]
    pub category: Option<CodeDt>,
    /// Resource(s) that are the subject of the event
    #[fhir(name="focus", min="0", max="*", summary="true", modifier="false")]
    pub focus: Option<Vec<MessageDefinitionFocusBackboneElement>>,
    /// always | on-error | never | on-success
    #[fhir(name="responseRequired", min="0", max="1", summary="false", modifier="false")]
    pub response_required: Option<CodeDt>,
    /// Responses to this message
    #[fhir(name="allowedResponse", min="0", max="*", summary="false", modifier="false")]
    pub allowed_response: Option<Vec<MessageDefinitionAllowedResponseBackboneElement>>,
    /// Canonical reference to a GraphDefinition
    #[fhir(name="graph", min="0", max="1", summary="false", modifier="false")]
    pub graph: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MessageDefinitionFocusBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of resource
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeDt>,
    /// Profile that must be adhered to by focus
    #[fhir(name="profile", min="0", max="1", summary="false", modifier="false")]
    pub profile: Option<CanonicalDt>,
    /// Minimum number of focuses of this type
    #[fhir(name="min", min="1", max="1", summary="true", modifier="false")]
    pub min: Option<UnsignedIntDt>,
    /// Maximum number of focuses of this type
    #[fhir(name="max", min="0", max="1", summary="false", modifier="false")]
    pub max: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MessageDefinitionAllowedResponseBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to allowed message definition response
    #[fhir(name="message", min="1", max="1", summary="false", modifier="false")]
    pub message: Option<CanonicalDt>,
    /// When should this response be used
    #[fhir(name="situation", min="0", max="1", summary="false", modifier="false")]
    pub situation: Option<MarkdownDt>,
}


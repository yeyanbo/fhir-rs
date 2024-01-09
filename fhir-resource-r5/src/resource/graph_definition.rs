use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct GraphDefinition {
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
    /// Canonical identifier for this graph definition, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the GraphDefinition (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the graph definition
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this graph definition (computer friendly)
    #[fhir(name="name", min="1", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this graph definition (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="true", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the graph definition
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for graph definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this graph definition is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// Starting Node
    #[fhir(name="start", min="0", max="1", summary="false", modifier="false")]
    pub start: Option<IdDt>,
    /// Potential target for the link
    #[fhir(name="node", min="0", max="*", summary="false", modifier="false")]
    pub node: Option<Vec<GraphDefinitionNodeBackboneElement>>,
    /// Links this graph makes rules about
    #[fhir(name="link", min="0", max="*", summary="false", modifier="false")]
    pub link: Option<Vec<GraphDefinitionLinkBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GraphDefinitionNodeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Internal ID - target for link references
    #[fhir(name="nodeId", min="1", max="1", summary="false", modifier="false")]
    pub node_id: Option<IdDt>,
    /// Why this node is specified
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// Type of resource this link refers to
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeDt>,
    /// Profile for the target resource
    #[fhir(name="profile", min="0", max="1", summary="false", modifier="false")]
    pub profile: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GraphDefinitionLinkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Why this link is specified
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// Minimum occurrences for this link
    #[fhir(name="min", min="0", max="1", summary="false", modifier="false")]
    pub min: Option<IntegerDt>,
    /// Maximum occurrences for this link
    #[fhir(name="max", min="0", max="1", summary="false", modifier="false")]
    pub max: Option<StringDt>,
    /// Source Node for this link
    #[fhir(name="sourceId", min="1", max="1", summary="false", modifier="false")]
    pub source_id: Option<IdDt>,
    /// Path in the resource that contains the link
    #[fhir(name="path", min="0", max="1", summary="false", modifier="false")]
    pub path: Option<StringDt>,
    /// Which slice (if profiled)
    #[fhir(name="sliceName", min="0", max="1", summary="false", modifier="false")]
    pub slice_name: Option<StringDt>,
    /// Target Node for this link
    #[fhir(name="targetId", min="1", max="1", summary="false", modifier="false")]
    pub target_id: Option<IdDt>,
    /// Criteria for reverse lookup
    #[fhir(name="params", min="0", max="1", summary="false", modifier="false")]
    pub params: Option<StringDt>,
    /// Compartment Consistency Rules
    #[fhir(name="compartment", min="0", max="*", summary="false", modifier="false")]
    pub compartment: Option<Vec<GraphDefinitionLinkCompartmentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GraphDefinitionLinkCompartmentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// where | requires
    #[fhir(name="use", min="1", max="1", summary="false", modifier="false")]
    pub use_: Option<CodeDt>,
    /// identical | matching | different | custom
    #[fhir(name="rule", min="1", max="1", summary="false", modifier="false")]
    pub rule: Option<CodeDt>,
    /// Patient | Encounter | RelatedPerson | Practitioner | Device | EpisodeOfCare
    #[fhir(name="code", min="1", max="1", summary="false", modifier="false")]
    pub code: Option<CodeDt>,
    /// Custom rule, as a FHIRPath expression
    #[fhir(name="expression", min="0", max="1", summary="false", modifier="false")]
    pub expression: Option<StringDt>,
    /// Documentation for FHIRPath expression
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
}


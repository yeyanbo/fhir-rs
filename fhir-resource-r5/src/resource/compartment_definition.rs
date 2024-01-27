use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct CompartmentDefinition {
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
    /// Canonical identifier for this compartment definition, represented as a URI (globally unique)
    #[fhir(name="url", min="1", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Business version of the compartment definition
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this compartment definition (computer friendly)
    #[fhir(name="name", min="1", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this compartment definition (human friendly)
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
    /// Natural language description of the compartment definition
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Why this compartment definition is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Patient | Encounter | RelatedPerson | Practitioner | Device | EpisodeOfCare
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeDt>,
    /// Whether the search syntax is supported
    #[fhir(name="search", min="1", max="1", summary="true", modifier="false")]
    pub search: Option<BooleanDt>,
    /// How a resource is related to the compartment
    #[fhir(name="resource", min="0", max="*", summary="true", modifier="false")]
    pub resource: Option<Vec<CompartmentDefinitionResourceBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CompartmentDefinitionResourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of resource type
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeDt>,
    /// Search Parameter Name, or chained parameters
    #[fhir(name="param", min="0", max="*", summary="true", modifier="false")]
    pub param: Option<Vec<StringDt>>,
    /// Additional documentation about the resource and compartment
    #[fhir(name="documentation", min="0", max="1", summary="false", modifier="false")]
    pub documentation: Option<StringDt>,
    /// Search Param for interpreting $everything.start
    #[fhir(name="startParam", min="0", max="1", summary="false", modifier="false")]
    pub start_param: Option<UriDt>,
    /// Search Param for interpreting $everything.end
    #[fhir(name="endParam", min="0", max="1", summary="false", modifier="false")]
    pub end_param: Option<UriDt>,
}


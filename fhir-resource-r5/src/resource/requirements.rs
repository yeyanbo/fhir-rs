use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Requirements {
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
    /// Canonical identifier for this Requirements, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the Requirements (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the Requirements
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this Requirements (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this Requirements (human friendly)
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
    /// Natural language description of the requirements
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for Requirements (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this Requirements is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// Other set of Requirements this builds on
    #[fhir(name="derivedFrom", min="0", max="*", summary="true", modifier="false")]
    pub derived_from: Option<Vec<CanonicalDt>>,
    /// External artifact (rule/document etc. that) created this set of requirements
    #[fhir(name="reference", min="0", max="*", summary="false", modifier="false")]
    pub reference: Option<Vec<UrlDt>>,
    /// Actor for these requirements
    #[fhir(name="actor", min="0", max="*", summary="false", modifier="false")]
    pub actor: Option<Vec<CanonicalDt>>,
    /// Actual statement as markdown
    #[fhir(name="statement", min="0", max="*", summary="false", modifier="false")]
    pub statement: Option<Vec<RequirementsStatementBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequirementsStatementBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Key that identifies this statement
    #[fhir(name="key", min="1", max="1", summary="false", modifier="false")]
    pub key: Option<IdDt>,
    /// Short Human label for this statement
    #[fhir(name="label", min="0", max="1", summary="false", modifier="false")]
    pub label: Option<StringDt>,
    /// SHALL | SHOULD | MAY | SHOULD-NOT
    #[fhir(name="conformance", min="0", max="*", summary="false", modifier="false")]
    pub conformance: Option<Vec<CodeDt>>,
    /// Set to true if requirements statement is conditional
    #[fhir(name="conditionality", min="0", max="1", summary="false", modifier="false")]
    pub conditionality: Option<BooleanDt>,
    /// The actual requirement
    #[fhir(name="requirement", min="1", max="1", summary="false", modifier="false")]
    pub requirement: Option<MarkdownDt>,
    /// Another statement this clarifies/restricts ([url#]key)
    #[fhir(name="derivedFrom", min="0", max="1", summary="false", modifier="false")]
    pub derived_from: Option<StringDt>,
    /// A larger requirement that this requirement helps to refine and enable
    #[fhir(name="parent", min="0", max="1", summary="false", modifier="false")]
    pub parent: Option<StringDt>,
    /// Design artifact that satisfies this requirement
    #[fhir(name="satisfiedBy", min="0", max="*", summary="false", modifier="false")]
    pub satisfied_by: Option<Vec<UrlDt>>,
    /// External artifact (rule/document etc. that) created this requirement
    #[fhir(name="reference", min="0", max="*", summary="false", modifier="false")]
    pub reference: Option<Vec<UrlDt>>,
    /// Who asked for this statement
    #[fhir(name="source", min="0", max="*", summary="false", modifier="false")]
    pub source: Option<Vec<Reference>>,
}


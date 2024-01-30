use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SearchParameter {
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
    /// Canonical identifier for this search parameter, represented as a URI (globally unique)
    #[fhir(name="url", min="1", max="1", summary=true, modifier=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the search parameter (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the search parameter
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this search parameter (computer friendly)
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name for this search parameter (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
    /// Original definition for the search parameter
    #[fhir(name="derivedFrom", min="0", max="1", summary=false, modifier=false)]
    pub derived_from: Option<CanonicalDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false)]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false)]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the search parameter
    #[fhir(name="description", min="1", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for search parameter (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this search parameter is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false)]
    pub copyright_label: Option<StringDt>,
    /// Recommended name for parameter in search url
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
    /// The resource type(s) this search parameter applies to
    #[fhir(name="base", min="1", max="*", summary=true, modifier=false)]
    pub base: Option<Vec<CodeDt>>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// FHIRPath expression that extracts the values
    #[fhir(name="expression", min="0", max="1", summary=false, modifier=false)]
    pub expression: Option<StringDt>,
    /// normal | phonetic | other
    #[fhir(name="processingMode", min="0", max="1", summary=false, modifier=false)]
    pub processing_mode: Option<CodeDt>,
    /// FHIRPath expression that constraints the usage of this SearchParamete
    #[fhir(name="constraint", min="0", max="1", summary=false, modifier=false)]
    pub constraint: Option<StringDt>,
    /// Types of resource (if a resource reference)
    #[fhir(name="target", min="0", max="*", summary=false, modifier=false)]
    pub target: Option<Vec<CodeDt>>,
    /// Allow multiple values per parameter (or)
    #[fhir(name="multipleOr", min="0", max="1", summary=false, modifier=false)]
    pub multiple_or: Option<BooleanDt>,
    /// Allow multiple parameters (and)
    #[fhir(name="multipleAnd", min="0", max="1", summary=false, modifier=false)]
    pub multiple_and: Option<BooleanDt>,
    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[fhir(name="comparator", min="0", max="*", summary=false, modifier=false)]
    pub comparator: Option<Vec<CodeDt>>,
    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false)]
    pub modifier: Option<Vec<CodeDt>>,
    /// Chained names supported
    #[fhir(name="chain", min="0", max="*", summary=false, modifier=false)]
    pub chain: Option<Vec<StringDt>>,
    /// For Composite resources to define the parts
    #[fhir(name="component", min="0", max="*", summary=false, modifier=false)]
    pub component: Option<Vec<SearchParameterComponentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SearchParameterComponentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Defines how the part works
    #[fhir(name="definition", min="1", max="1", summary=false, modifier=false)]
    pub definition: Option<CanonicalDt>,
    /// Subexpression relative to main expression
    #[fhir(name="expression", min="1", max="1", summary=false, modifier=false)]
    pub expression: Option<StringDt>,
}


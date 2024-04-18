use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Library {
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
    /// Canonical identifier for this library, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the library
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the library
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this library (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name for this library (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
    /// Subordinate title of the library
    #[fhir(name="subtitle", min="0", max="1", summary=false, modifier=false)]
    pub subtitle: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false)]
    pub experimental: Option<BooleanDt>,
    /// logic-library | model-definition | asset-collection | module-definition
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Type of individual the library content is focused on
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false)]
    pub subject: Option<Reference>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false)]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the library
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for library (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this library is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false)]
    pub purpose: Option<MarkdownDt>,
    /// Describes the clinical usage of the library
    #[fhir(name="usage", min="0", max="1", summary=false, modifier=false)]
    pub usage: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false)]
    pub copyright_label: Option<StringDt>,
    /// When the library was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false)]
    pub approval_date: Option<DateDt>,
    /// When the library was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false)]
    pub last_review_date: Option<DateDt>,
    /// When the library is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false)]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary=false, modifier=false)]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false)]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false)]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false)]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false)]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false)]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Parameters defined by the library
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false)]
    pub parameter: Option<Vec<ParameterDefinition>>,
    /// What data is referenced by this library
    #[fhir(name="dataRequirement", min="0", max="*", summary=false, modifier=false)]
    pub data_requirement: Option<Vec<DataRequirement>>,
    /// Contents of the library, either embedded or referenced
    #[fhir(name="content", min="0", max="*", summary=true, modifier=false)]
    pub content: Option<Vec<Attachment>>,
}


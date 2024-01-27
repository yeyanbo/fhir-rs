use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct NamingSystem {
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
    /// Canonical identifier for this naming system, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the naming system (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the naming system
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this naming system (computer friendly)
    #[fhir(name="name", min="1", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Title for this naming system (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// codesystem | identifier | root
    #[fhir(name="kind", min="1", max="1", summary="true", modifier="false")]
    pub kind: Option<CodeDt>,
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
    /// Who maintains system namespace?
    #[fhir(name="responsible", min="0", max="1", summary="false", modifier="false")]
    pub responsible: Option<StringDt>,
    /// e.g. driver,  provider,  patient, bank etc
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Natural language description of the naming system
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for naming system (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this naming system is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// When the NamingSystem was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary="false", modifier="false")]
    pub approval_date: Option<DateDt>,
    /// When the NamingSystem was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary="false", modifier="false")]
    pub last_review_date: Option<DateDt>,
    /// When the NamingSystem is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary="true", modifier="false")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary="false", modifier="false")]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the CodeSystem
    #[fhir(name="author", min="0", max="*", summary="false", modifier="false")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the NamingSystem
    #[fhir(name="editor", min="0", max="*", summary="false", modifier="false")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the NamingSystem
    #[fhir(name="reviewer", min="0", max="*", summary="false", modifier="false")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the NamingSystem
    #[fhir(name="endorser", min="0", max="*", summary="false", modifier="false")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary="false", modifier="false")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// How/where is it used
    #[fhir(name="usage", min="0", max="1", summary="false", modifier="false")]
    pub usage: Option<StringDt>,
    /// Unique identifiers used for system
    #[fhir(name="uniqueId", min="1", max="*", summary="true", modifier="false")]
    pub unique_id: Option<Vec<NamingSystemUniqueIdBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NamingSystemUniqueIdBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// oid | uuid | uri | iri-stem | v2csmnemonic | other
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeDt>,
    /// The unique identifier
    #[fhir(name="value", min="1", max="1", summary="true", modifier="false")]
    pub value: Option<StringDt>,
    /// Is this the id that should be used for this type
    #[fhir(name="preferred", min="0", max="1", summary="false", modifier="false")]
    pub preferred: Option<BooleanDt>,
    /// Notes about identifier usage
    #[fhir(name="comment", min="0", max="1", summary="false", modifier="false")]
    pub comment: Option<StringDt>,
    /// When is identifier valid?
    #[fhir(name="period", min="0", max="1", summary="false", modifier="false")]
    pub period: Option<Period>,
    /// Whether the identifier is authoritative
    #[fhir(name="authoritative", min="0", max="1", summary="false", modifier="false")]
    pub authoritative: Option<BooleanDt>,
}


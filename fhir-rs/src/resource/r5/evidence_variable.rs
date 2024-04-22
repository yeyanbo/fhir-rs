use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct EvidenceVariable {
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
    /// Canonical identifier for this evidence variable, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the evidence variable
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the evidence variable
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this evidence variable (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this evidence variable (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// Title for use in informal contexts
    #[fhir(name="shortTitle", min="0", max="1", summary=true, modifier=false, choice="")]
    pub short_title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=false, modifier=false, choice="")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice="")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice="")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the evidence variable
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Used for footnotes or explanatory notes
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Why this EvidenceVariable is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// When the resource was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub approval_date: Option<DateDt>,
    /// When the resource was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub last_review_date: Option<DateDt>,
    /// When the resource is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=false, modifier=false, choice="")]
    pub effective_period: Option<Period>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false, choice="")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false, choice="")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false, choice="")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Actual or conceptual
    #[fhir(name="actual", min="0", max="1", summary=false, modifier=false, choice="")]
    pub actual: Option<BooleanDt>,
    /// A defining factor of the EvidenceVariable
    #[fhir(name="characteristic", min="0", max="*", summary=true, modifier=false, choice="")]
    pub characteristic: Option<Vec<EvidenceVariableCharacteristicBackboneElement>>,
    /// continuous | dichotomous | ordinal | polychotomous
    #[fhir(name="handling", min="0", max="1", summary=false, modifier=false, choice="")]
    pub handling: Option<CodeDt>,
    /// A grouping for ordinal or polychotomous variables
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<EvidenceVariableCategoryBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceVariableCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for internal linking
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link_id: Option<IdDt>,
    /// Natural language description of the characteristic
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Used for footnotes or explanatory notes
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Whether the characteristic is an inclusion criterion or exclusion criterion
    #[fhir(name="exclude", min="0", max="1", summary=false, modifier=false, choice="")]
    pub exclude: Option<BooleanDt>,
    /// Defines the characteristic (without using type and value) by a Reference
    #[fhir(name="definitionReference", min="0", max="1", summary=true, modifier=false, choice="")]
    pub definition_reference: Option<Reference>,
    /// Defines the characteristic (without using type and value) by a Canonical
    #[fhir(name="definitionCanonical", min="0", max="1", summary=true, modifier=false, choice="")]
    pub definition_canonical: Option<CanonicalDt>,
    /// Defines the characteristic (without using type and value) by a CodeableConcept
    #[fhir(name="definitionCodeableConcept", min="0", max="1", summary=true, modifier=false, choice="")]
    pub definition_codeable_concept: Option<CodeableConcept>,
    /// Defines the characteristic (without using type and value) by an expression
    #[fhir(name="definitionExpression", min="0", max="1", summary=true, modifier=false, choice="")]
    pub definition_expression: Option<Expression>,
    /// Defines the characteristic (without using type and value) by an id
    #[fhir(name="definitionId", min="0", max="1", summary=true, modifier=false, choice="")]
    pub definition_id: Option<IdDt>,
    /// Defines the characteristic using type and value
    #[fhir(name="definitionByTypeAndValue", min="0", max="1", summary=true, modifier=false, choice="")]
    pub definition_by_type_and_value: Option<EvidenceVariableCharacteristicDefinitionByTypeAndValueBackboneElement>,
    /// Used to specify how two or more characteristics are combined
    #[fhir(name="definitionByCombination", min="0", max="1", summary=false, modifier=false, choice="")]
    pub definition_by_combination: Option<EvidenceVariableCharacteristicDefinitionByCombinationBackboneElement>,
    /// Number of occurrences meeting the characteristic
    #[fhir(name="instances", min="0", max="1", summary=false, modifier=false, choice="")]
    pub instances: Option<Range>,
    /// Length of time in which the characteristic is met
    #[fhir(name="duration", min="0", max="1", summary=false, modifier=false, choice="")]
    pub duration: Option<Range>,
    /// Timing in which the characteristic is determined
    #[fhir(name="timeFromEvent", min="0", max="*", summary=false, modifier=false, choice="")]
    pub time_from_event: Option<Vec<EvidenceVariableCharacteristicTimeFromEventBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceVariableCharacteristicDefinitionByCombinationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// all-of | any-of | at-least | at-most | statistical | net-effect | dataset
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Provides the value of "n" when "at-least" or "at-most" codes are used
    #[fhir(name="threshold", min="0", max="1", summary=false, modifier=false, choice="")]
    pub threshold: Option<PositiveIntDt>,
    /// A defining factor of the characteristic
    #[fhir(name="characteristic", min="1", max="*", summary=false, modifier=false, choice="")]
    pub characteristic: Option<Vec<EvidenceVariableCharacteristicBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceVariableCharacteristicDefinitionByTypeAndValueBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Expresses the type of characteristic
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Method for how the characteristic value was determined
    #[fhir(name="method", min="0", max="*", summary=false, modifier=false, choice="")]
    pub method: Option<Vec<CodeableConcept>>,
    /// Device used for determining characteristic
    #[fhir(name="device", min="0", max="1", summary=false, modifier=false, choice="")]
    pub device: Option<Reference>,
    /// Defines the characteristic when coupled with characteristic.type
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false, choice="")]
    pub value: Option<IdDt>,
    /// Reference point for valueQuantity or valueRange
    #[fhir(name="offset", min="0", max="1", summary=false, modifier=false, choice="")]
    pub offset: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceVariableCharacteristicTimeFromEventBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Human readable description
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Used for footnotes or explanatory notes
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// The event used as a base point (reference point) in time
    #[fhir(name="event", min="0", max="1", summary=false, modifier=false, choice="")]
    pub event: Option<IdDt>,
    /// Used to express the observation at a defined amount of time before or after the event
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Used to express the observation within a period before and/or after the event
    #[fhir(name="range", min="0", max="1", summary=false, modifier=false, choice="")]
    pub range: Option<Range>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceVariableCategoryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Description of the grouping
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Definition of the grouping
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Range>,
}


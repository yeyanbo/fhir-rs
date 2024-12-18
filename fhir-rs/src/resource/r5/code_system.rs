use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CodeSystem {
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
    /// Canonical identifier for this code system, represented as a URI (globally unique) (Coding.system)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the code system (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the code system (Coding.version)
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this code system (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this code system (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice="")]
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
    /// Natural language description of the code system
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for code system (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this code system is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// When the CodeSystem was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub approval_date: Option<DateDt>,
    /// When the CodeSystem was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub last_review_date: Option<DateDt>,
    /// When the CodeSystem is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false, choice="")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary=false, modifier=false, choice="")]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the CodeSystem
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false, choice="")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the CodeSystem
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false, choice="")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the CodeSystem
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the CodeSystem
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false, choice="")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// If code comparison is case sensitive
    #[fhir(name="caseSensitive", min="0", max="1", summary=true, modifier=false, choice="")]
    pub case_sensitive: Option<BooleanDt>,
    /// Canonical reference to the value set with entire code system
    #[fhir(name="valueSet", min="0", max="1", summary=true, modifier=false, choice="")]
    pub value_set: Option<CanonicalDt>,
    /// grouped-by | is-a | part-of | classified-with
    #[fhir(name="hierarchyMeaning", min="0", max="1", summary=true, modifier=false, choice="")]
    pub hierarchy_meaning: Option<CodeDt>,
    /// If code system defines a compositional grammar
    #[fhir(name="compositional", min="0", max="1", summary=true, modifier=false, choice="")]
    pub compositional: Option<BooleanDt>,
    /// If definitions are not stable
    #[fhir(name="versionNeeded", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_needed: Option<BooleanDt>,
    /// not-present | example | fragment | complete | supplement
    #[fhir(name="content", min="1", max="1", summary=true, modifier=false, choice="")]
    pub content: Option<CodeDt>,
    /// Canonical URL of Code System this adds designations and properties to
    #[fhir(name="supplements", min="0", max="1", summary=true, modifier=false, choice="")]
    pub supplements: Option<CanonicalDt>,
    /// Total concepts in the code system
    #[fhir(name="count", min="0", max="1", summary=true, modifier=false, choice="")]
    pub count: Option<UnsignedIntDt>,
    /// Filter that can be used in a value set
    #[fhir(name="filter", min="0", max="*", summary=true, modifier=false, choice="")]
    pub filter: Option<Vec<CodeSystemFilterBackboneElement>>,
    /// Additional information supplied about each concept
    #[fhir(name="property", min="0", max="*", summary=true, modifier=false, choice="")]
    pub property: Option<Vec<CodeSystemPropertyBackboneElement>>,
    /// Concepts in the code system
    #[fhir(name="concept", min="0", max="*", summary=false, modifier=false, choice="")]
    pub concept: Option<Vec<CodeSystemConceptBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CodeSystemFilterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that identifies the filter
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// How or why the filter is used
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
    #[fhir(name="operator", min="1", max="*", summary=true, modifier=false, choice="")]
    pub operator: Option<Vec<CodeDt>>,
    /// What to use for the value
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false, choice="")]
    pub value: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CodeSystemPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifies the property on the concepts, and when referred to in operations
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Formal identifier for the property
    #[fhir(name="uri", min="0", max="1", summary=true, modifier=false, choice="")]
    pub uri: Option<UriDt>,
    /// Why the property is defined, and/or what it conveys
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// code | Coding | string | integer | boolean | dateTime | decimal
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CodeSystemConceptBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that identifies concept
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Text to display to the user
    #[fhir(name="display", min="0", max="1", summary=false, modifier=false, choice="")]
    pub display: Option<StringDt>,
    /// Formal definition
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub definition: Option<StringDt>,
    /// Additional representations for the concept
    #[fhir(name="designation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub designation: Option<Vec<CodeSystemConceptDesignationBackboneElement>>,
    /// Property value for the concept
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false, choice="")]
    pub property: Option<Vec<CodeSystemConceptPropertyBackboneElement>>,
    /// Child Concepts (is-a/contains/categorizes)
    #[fhir(name="concept", min="0", max="*", summary=false, modifier=false, choice="")]
    pub concept: Option<Vec<CodeSystemConceptBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CodeSystemConceptDesignationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Human language of the designation
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice="")]
    pub language: Option<CodeDt>,
    /// Details how this designation would be used
    #[fhir(name="use", min="0", max="1", summary=false, modifier=false, choice="")]
    pub use_: Option<Coding>,
    /// Additional ways how this designation would be used
    #[fhir(name="additionalUse", min="0", max="*", summary=false, modifier=false, choice="")]
    pub additional_use: Option<Vec<Coding>>,
    /// The text value for this designation
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CodeSystemConceptPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to CodeSystem.property.code
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Value of the property for this concept
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<DecimalDt>,
}


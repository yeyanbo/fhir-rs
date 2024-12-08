use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ValueSet {
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
    /// Canonical identifier for this value set, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the value set (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the value set
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this value set (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this value set (human friendly)
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
    /// Natural language description of the value set
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for value set (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Indicates whether or not any change to the content logical definition may occur
    #[fhir(name="immutable", min="0", max="1", summary=true, modifier=false, choice="")]
    pub immutable: Option<BooleanDt>,
    /// Why this value set is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// When the ValueSet was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub approval_date: Option<DateDt>,
    /// When the ValueSet was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub last_review_date: Option<DateDt>,
    /// When the ValueSet is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false, choice="")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary=false, modifier=false, choice="")]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the ValueSet
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false, choice="")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the ValueSet
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false, choice="")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the ValueSet
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the ValueSet
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false, choice="")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Content logical definition of the value set (CLD)
    #[fhir(name="compose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub compose: Option<ValueSetComposeBackboneElement>,
    /// Used when the value set is "expanded"
    #[fhir(name="expansion", min="0", max="1", summary=false, modifier=false, choice="")]
    pub expansion: Option<ValueSetExpansionBackboneElement>,
    /// Description of the semantic space the Value Set Expansion is intended to cover and should further clarify the text in ValueSet.description
    #[fhir(name="scope", min="0", max="1", summary=false, modifier=false, choice="")]
    pub scope: Option<ValueSetScopeBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetScopeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Criteria describing which concepts or codes should be included and why
    #[fhir(name="inclusionCriteria", min="0", max="1", summary=false, modifier=false, choice="")]
    pub inclusion_criteria: Option<StringDt>,
    /// Criteria describing which concepts or codes should be excluded and why
    #[fhir(name="exclusionCriteria", min="0", max="1", summary=false, modifier=false, choice="")]
    pub exclusion_criteria: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetExpansionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifies the value set expansion (business identifier)
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identifier: Option<UriDt>,
    /// Opaque urls for paging through expansion results
    #[fhir(name="next", min="0", max="1", summary=false, modifier=false, choice="")]
    pub next: Option<UriDt>,
    /// Time ValueSet expansion happened
    #[fhir(name="timestamp", min="1", max="1", summary=false, modifier=false, choice="")]
    pub timestamp: Option<DateTimeDt>,
    /// Total number of codes in the expansion
    #[fhir(name="total", min="0", max="1", summary=false, modifier=false, choice="")]
    pub total: Option<IntegerDt>,
    /// Offset at which this resource starts
    #[fhir(name="offset", min="0", max="1", summary=false, modifier=false, choice="")]
    pub offset: Option<IntegerDt>,
    /// Parameter that controlled the expansion process
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub parameter: Option<Vec<ValueSetExpansionParameterBackboneElement>>,
    /// Additional information supplied about each concept
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false, choice="")]
    pub property: Option<Vec<ValueSetExpansionPropertyBackboneElement>>,
    /// Codes in the value set
    #[fhir(name="contains", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contains: Option<Vec<ValueSetExpansionContainsBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetExpansionParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name as assigned by the client or server
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Value of the named parameter
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<DateTimeDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetExpansionContainsBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// System value for the code
    #[fhir(name="system", min="0", max="1", summary=false, modifier=false, choice="")]
    pub system: Option<UriDt>,
    /// If user cannot select this entry
    #[fhir(name="abstract", min="0", max="1", summary=false, modifier=false, choice="")]
    pub abstract_: Option<BooleanDt>,
    /// If concept is inactive in the code system
    #[fhir(name="inactive", min="0", max="1", summary=false, modifier=false, choice="")]
    pub inactive: Option<BooleanDt>,
    /// Version in which this code/display is defined
    #[fhir(name="version", min="0", max="1", summary=false, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// Code - if blank, this is not a selectable code
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// User display for the concept
    #[fhir(name="display", min="0", max="1", summary=false, modifier=false, choice="")]
    pub display: Option<StringDt>,
    /// Additional representations for this item
    #[fhir(name="designation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub designation: Option<Vec<ValueSetComposeIncludeConceptDesignationBackboneElement>>,
    /// Property value for the concept
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false, choice="")]
    pub property: Option<Vec<ValueSetExpansionContainsPropertyBackboneElement>>,
    /// Codes contained under this entry
    #[fhir(name="contains", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contains: Option<Vec<ValueSetExpansionContainsBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetExpansionContainsPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to ValueSet.expansion.property.code
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Value of the property for this concept
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<DecimalDt>,
    /// SubProperty value for the concept
    #[fhir(name="subProperty", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_property: Option<Vec<ValueSetExpansionContainsPropertySubPropertyBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetExpansionContainsPropertySubPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to ValueSet.expansion.property.code
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Value of the subproperty for this concept
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<DecimalDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetExpansionPropertyBackboneElement {
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
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Formal identifier for the property
    #[fhir(name="uri", min="0", max="1", summary=false, modifier=false, choice="")]
    pub uri: Option<UriDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetComposeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Fixed date for references with no specified version (transitive)
    #[fhir(name="lockedDate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub locked_date: Option<DateDt>,
    /// Whether inactive codes are in the value set
    #[fhir(name="inactive", min="0", max="1", summary=true, modifier=false, choice="")]
    pub inactive: Option<BooleanDt>,
    /// Include one or more codes from a code system or other value set(s)
    #[fhir(name="include", min="1", max="*", summary=true, modifier=false, choice="")]
    pub include: Option<Vec<ValueSetComposeIncludeBackboneElement>>,
    /// Explicitly exclude codes from a code system or other value sets
    #[fhir(name="exclude", min="0", max="*", summary=false, modifier=false, choice="")]
    pub exclude: Option<Vec<ValueSetComposeIncludeBackboneElement>>,
    /// Property to return if client doesn't override
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false, choice="")]
    pub property: Option<Vec<StringDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetComposeIncludeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The system the codes come from
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false, choice="")]
    pub system: Option<UriDt>,
    /// Specific version of the code system referred to
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// A concept defined in the system
    #[fhir(name="concept", min="0", max="*", summary=false, modifier=false, choice="")]
    pub concept: Option<Vec<ValueSetComposeIncludeConceptBackboneElement>>,
    /// Select codes/concepts by their properties (including relationships)
    #[fhir(name="filter", min="0", max="*", summary=true, modifier=false, choice="")]
    pub filter: Option<Vec<ValueSetComposeIncludeFilterBackboneElement>>,
    /// Select the contents included in this value set
    #[fhir(name="valueSet", min="0", max="*", summary=true, modifier=false, choice="")]
    pub value_set: Option<Vec<CanonicalDt>>,
    /// A copyright statement for the specific code system included in the value set
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetComposeIncludeConceptBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code or expression from system
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Text to display for this code for this value set in this valueset
    #[fhir(name="display", min="0", max="1", summary=false, modifier=false, choice="")]
    pub display: Option<StringDt>,
    /// Additional representations for this concept
    #[fhir(name="designation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub designation: Option<Vec<ValueSetComposeIncludeConceptDesignationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ValueSetComposeIncludeConceptDesignationBackboneElement {
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
    /// Types of uses of designations
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
pub struct ValueSetComposeIncludeFilterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A property/filter defined by the code system
    #[fhir(name="property", min="1", max="1", summary=true, modifier=false, choice="")]
    pub property: Option<CodeDt>,
    /// = | is-a | descendent-of | is-not-a | regex | in | not-in | generalizes | child-of | descendent-leaf | exists
    #[fhir(name="op", min="1", max="1", summary=true, modifier=false, choice="")]
    pub op: Option<CodeDt>,
    /// Code from the system, or regex criteria, or boolean value for exists
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false, choice="")]
    pub value: Option<StringDt>,
}


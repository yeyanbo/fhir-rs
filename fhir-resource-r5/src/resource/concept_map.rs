use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct ConceptMap {
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
    /// Canonical identifier for this concept map, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the concept map
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the concept map
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this concept map (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this concept map (human friendly)
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
    /// Natural language description of the concept map
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for concept map (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this concept map is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// When the ConceptMap was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary="false", modifier="false")]
    pub approval_date: Option<DateDt>,
    /// When the ConceptMap was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary="false", modifier="false")]
    pub last_review_date: Option<DateDt>,
    /// When the ConceptMap is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary="true", modifier="false")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary="false", modifier="false")]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the ConceptMap
    #[fhir(name="author", min="0", max="*", summary="false", modifier="false")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the ConceptMap
    #[fhir(name="editor", min="0", max="*", summary="false", modifier="false")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the ConceptMap
    #[fhir(name="reviewer", min="0", max="*", summary="false", modifier="false")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the ConceptMap
    #[fhir(name="endorser", min="0", max="*", summary="false", modifier="false")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary="false", modifier="false")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Additional properties of the mapping
    #[fhir(name="property", min="0", max="*", summary="true", modifier="false")]
    pub property: Option<Vec<ConceptMapPropertyBackboneElement>>,
    /// Definition of an additional attribute to act as a data source or target
    #[fhir(name="additionalAttribute", min="0", max="*", summary="true", modifier="false")]
    pub additional_attribute: Option<Vec<ConceptMapAdditionalAttributeBackboneElement>>,
    /// The source value set that contains the concepts that are being mapped
    #[fhir(name="sourceScope", min="0", max="1", summary="true", modifier="false")]
    pub source_scope: Option<CanonicalDt>,
    /// The target value set which provides context for the mappings
    #[fhir(name="targetScope", min="0", max="1", summary="true", modifier="false")]
    pub target_scope: Option<CanonicalDt>,
    /// Same source and target systems
    #[fhir(name="group", min="0", max="*", summary="false", modifier="false")]
    pub group: Option<Vec<ConceptMapGroupBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifies the property on the mappings, and when referred to in the $translate operation
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeDt>,
    /// Formal identifier for the property
    #[fhir(name="uri", min="0", max="1", summary="true", modifier="false")]
    pub uri: Option<UriDt>,
    /// Why the property is defined, and/or what it conveys
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<StringDt>,
    /// Coding | string | integer | boolean | dateTime | decimal | code
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeDt>,
    /// The CodeSystem from which code values come
    #[fhir(name="system", min="0", max="1", summary="true", modifier="false")]
    pub system: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapAdditionalAttributeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifies this additional attribute through this resource
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeDt>,
    /// Formal identifier for the data element referred to in this attribte
    #[fhir(name="uri", min="0", max="1", summary="true", modifier="false")]
    pub uri: Option<UriDt>,
    /// Why the additional attribute is defined, and/or what the data element it refers to is
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<StringDt>,
    /// code | Coding | string | boolean | Quantity
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapGroupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Source system where concepts to be mapped are defined
    #[fhir(name="source", min="0", max="1", summary="false", modifier="false")]
    pub source: Option<CanonicalDt>,
    /// Target system that the concepts are to be mapped to
    #[fhir(name="target", min="0", max="1", summary="false", modifier="false")]
    pub target: Option<CanonicalDt>,
    /// Mappings for a concept from the source set
    #[fhir(name="element", min="1", max="*", summary="false", modifier="false")]
    pub element: Option<Vec<ConceptMapGroupElementBackboneElement>>,
    /// What to do when there is no mapping target for the source concept and ConceptMap.group.element.noMap is not true
    #[fhir(name="unmapped", min="0", max="1", summary="false", modifier="false")]
    pub unmapped: Option<ConceptMapGroupUnmappedBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapGroupElementBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifies element being mapped
    #[fhir(name="code", min="0", max="1", summary="false", modifier="false")]
    pub code: Option<CodeDt>,
    /// Display for the code
    #[fhir(name="display", min="0", max="1", summary="false", modifier="false")]
    pub display: Option<StringDt>,
    /// Identifies the set of concepts being mapped
    #[fhir(name="valueSet", min="0", max="1", summary="false", modifier="false")]
    pub value_set: Option<CanonicalDt>,
    /// No mapping to a target concept for this source concept
    #[fhir(name="noMap", min="0", max="1", summary="false", modifier="false")]
    pub no_map: Option<BooleanDt>,
    /// Concept in target system for element
    #[fhir(name="target", min="0", max="*", summary="false", modifier="false")]
    pub target: Option<Vec<ConceptMapGroupElementTargetBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapGroupElementTargetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that identifies the target element
    #[fhir(name="code", min="0", max="1", summary="false", modifier="false")]
    pub code: Option<CodeDt>,
    /// Display for the code
    #[fhir(name="display", min="0", max="1", summary="false", modifier="false")]
    pub display: Option<StringDt>,
    /// Identifies the set of target concepts
    #[fhir(name="valueSet", min="0", max="1", summary="false", modifier="false")]
    pub value_set: Option<CanonicalDt>,
    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    #[fhir(name="relationship", min="1", max="1", summary="false", modifier="true")]
    pub relationship: Option<CodeDt>,
    /// Description of status/issues in mapping
    #[fhir(name="comment", min="0", max="1", summary="false", modifier="false")]
    pub comment: Option<StringDt>,
    /// Property value for the source -> target mapping
    #[fhir(name="property", min="0", max="*", summary="false", modifier="false")]
    pub property: Option<Vec<ConceptMapGroupElementTargetPropertyBackboneElement>>,
    /// Other properties required for this mapping
    #[fhir(name="dependsOn", min="0", max="*", summary="false", modifier="false")]
    pub depends_on: Option<Vec<ConceptMapGroupElementTargetDependsOnBackboneElement>>,
    /// Other data elements that this mapping also produces
    #[fhir(name="product", min="0", max="*", summary="false", modifier="false")]
    pub product: Option<Vec<ConceptMapGroupElementTargetDependsOnBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapGroupElementTargetPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to ConceptMap.property.code
    #[fhir(name="code", min="1", max="1", summary="false", modifier="false")]
    pub code: Option<CodeDt>,
    /// Value of the property for this concept
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapGroupElementTargetDependsOnBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A reference to a mapping attribute defined in ConceptMap.additionalAttribute
    #[fhir(name="attribute", min="1", max="1", summary="false", modifier="false")]
    pub attribute: Option<CodeDt>,
    /// Value of the referenced data element
    #[fhir(name="value", min="0", max="1", summary="false", modifier="false")]
    pub value: Option<Quantity>,
    /// The mapping depends on a data element with a value from this value set
    #[fhir(name="valueSet", min="0", max="1", summary="false", modifier="false")]
    pub value_set: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConceptMapGroupUnmappedBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// use-source-code | fixed | other-map
    #[fhir(name="mode", min="1", max="1", summary="false", modifier="false")]
    pub mode: Option<CodeDt>,
    /// Fixed code when mode = fixed
    #[fhir(name="code", min="0", max="1", summary="false", modifier="false")]
    pub code: Option<CodeDt>,
    /// Display for the code
    #[fhir(name="display", min="0", max="1", summary="false", modifier="false")]
    pub display: Option<StringDt>,
    /// Fixed code set when mode = fixed
    #[fhir(name="valueSet", min="0", max="1", summary="false", modifier="false")]
    pub value_set: Option<CanonicalDt>,
    /// related-to | equivalent | source-is-narrower-than-target | source-is-broader-than-target | not-related-to
    #[fhir(name="relationship", min="0", max="1", summary="false", modifier="true")]
    pub relationship: Option<CodeDt>,
    /// canonical reference to an additional ConceptMap to use for mapping if the source concept is unmapped
    #[fhir(name="otherMap", min="0", max="1", summary="false", modifier="false")]
    pub other_map: Option<CanonicalDt>,
}


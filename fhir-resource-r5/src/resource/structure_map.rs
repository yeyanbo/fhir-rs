use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct StructureMap {
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
    /// Canonical identifier for this structure map, represented as a URI (globally unique)
    #[fhir(name="url", min="1", max="1", summary=true, modifier=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the structure map
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the structure map
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this structure map (computer friendly)
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name for this structure map (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
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
    /// Natural language description of the structure map
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for structure map (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this structure map is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false)]
    pub copyright_label: Option<StringDt>,
    /// Structure Definition used by this map
    #[fhir(name="structure", min="0", max="*", summary=true, modifier=false)]
    pub structure: Option<Vec<StructureMapStructureBackboneElement>>,
    /// Other maps used by this map (canonical URLs)
    #[fhir(name="import", min="0", max="*", summary=true, modifier=false)]
    pub import: Option<Vec<CanonicalDt>>,
    /// Definition of the constant value used in the map rules
    #[fhir(name="const", min="0", max="*", summary=true, modifier=false)]
    pub const_: Option<Vec<StructureMapConstBackboneElement>>,
    /// Named sections for reader convenience
    #[fhir(name="group", min="1", max="*", summary=true, modifier=false)]
    pub group: Option<Vec<StructureMapGroupBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapConstBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Constant name
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<IdDt>,
    /// FHIRPath exression - value of the constant
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Human-readable label
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<IdDt>,
    /// Another group that this group adds rules to
    #[fhir(name="extends", min="0", max="1", summary=true, modifier=false)]
    pub extends: Option<IdDt>,
    /// types | type-and-types
    #[fhir(name="typeMode", min="0", max="1", summary=true, modifier=false)]
    pub type_mode: Option<CodeDt>,
    /// Additional description/explanation for group
    #[fhir(name="documentation", min="0", max="1", summary=true, modifier=false)]
    pub documentation: Option<StringDt>,
    /// Named instance provided when invoking the map
    #[fhir(name="input", min="1", max="*", summary=true, modifier=false)]
    pub input: Option<Vec<StructureMapGroupInputBackboneElement>>,
    /// Transform Rule from source to target
    #[fhir(name="rule", min="0", max="*", summary=true, modifier=false)]
    pub rule: Option<Vec<StructureMapGroupRuleBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupInputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name for this instance of data
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<IdDt>,
    /// Type for this instance of data
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<StringDt>,
    /// source | target
    #[fhir(name="mode", min="1", max="1", summary=true, modifier=false)]
    pub mode: Option<CodeDt>,
    /// Documentation for this instance of data
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false)]
    pub documentation: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupRuleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of the rule for internal references
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<IdDt>,
    /// Source inputs to the mapping
    #[fhir(name="source", min="1", max="*", summary=true, modifier=false)]
    pub source: Option<Vec<StructureMapGroupRuleSourceBackboneElement>>,
    /// Content to create because of this mapping rule
    #[fhir(name="target", min="0", max="*", summary=true, modifier=false)]
    pub target: Option<Vec<StructureMapGroupRuleTargetBackboneElement>>,
    /// Rules contained in this rule
    #[fhir(name="rule", min="0", max="*", summary=true, modifier=false)]
    pub rule: Option<Vec<StructureMapGroupRuleBackboneElement>>,
    /// Which other rules to apply in the context of this rule
    #[fhir(name="dependent", min="0", max="*", summary=true, modifier=false)]
    pub dependent: Option<Vec<StructureMapGroupRuleDependentBackboneElement>>,
    /// Documentation for this instance of data
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false)]
    pub documentation: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupRuleTargetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Variable this rule applies to
    #[fhir(name="context", min="0", max="1", summary=true, modifier=false)]
    pub context: Option<StringDt>,
    /// Field to create in the context
    #[fhir(name="element", min="0", max="1", summary=true, modifier=false)]
    pub element: Option<StringDt>,
    /// Named context for field, if desired, and a field is specified
    #[fhir(name="variable", min="0", max="1", summary=true, modifier=false)]
    pub variable: Option<IdDt>,
    /// first | share | last | single
    #[fhir(name="listMode", min="0", max="*", summary=true, modifier=false)]
    pub list_mode: Option<Vec<CodeDt>>,
    /// Internal rule reference for shared list items
    #[fhir(name="listRuleId", min="0", max="1", summary=true, modifier=false)]
    pub list_rule_id: Option<IdDt>,
    /// create | copy +
    #[fhir(name="transform", min="0", max="1", summary=true, modifier=false)]
    pub transform: Option<CodeDt>,
    /// Parameters to the transform
    #[fhir(name="parameter", min="0", max="*", summary=true, modifier=false)]
    pub parameter: Option<Vec<StructureMapGroupRuleTargetParameterBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupRuleTargetParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Parameter value - variable or literal
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false)]
    pub value: Option<DateTimeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupRuleDependentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of a rule or group to apply
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<IdDt>,
    /// Parameter to pass to the rule or group
    #[fhir(name="parameter", min="1", max="*", summary=true, modifier=false)]
    pub parameter: Option<Vec<StructureMapGroupRuleTargetParameterBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapGroupRuleSourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type or variable this rule applies to
    #[fhir(name="context", min="1", max="1", summary=true, modifier=false)]
    pub context: Option<IdDt>,
    /// Specified minimum cardinality
    #[fhir(name="min", min="0", max="1", summary=true, modifier=false)]
    pub min: Option<IntegerDt>,
    /// Specified maximum cardinality (number or *)
    #[fhir(name="max", min="0", max="1", summary=true, modifier=false)]
    pub max: Option<StringDt>,
    /// Rule only applies if source has this type
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<StringDt>,
    /// Default value if no value exists
    #[fhir(name="defaultValue", min="0", max="1", summary=true, modifier=false)]
    pub default_value: Option<StringDt>,
    /// Optional field for this source
    #[fhir(name="element", min="0", max="1", summary=true, modifier=false)]
    pub element: Option<StringDt>,
    /// first | not_first | last | not_last | only_one
    #[fhir(name="listMode", min="0", max="1", summary=true, modifier=false)]
    pub list_mode: Option<CodeDt>,
    /// Named context for field, if a field is specified
    #[fhir(name="variable", min="0", max="1", summary=true, modifier=false)]
    pub variable: Option<IdDt>,
    /// FHIRPath expression  - must be true or the rule does not apply
    #[fhir(name="condition", min="0", max="1", summary=true, modifier=false)]
    pub condition: Option<StringDt>,
    /// FHIRPath expression  - must be true or the mapping engine throws an error instead of completing
    #[fhir(name="check", min="0", max="1", summary=true, modifier=false)]
    pub check: Option<StringDt>,
    /// Message to put in log if source exists (FHIRPath)
    #[fhir(name="logMessage", min="0", max="1", summary=true, modifier=false)]
    pub log_message: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct StructureMapStructureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Canonical reference to structure definition
    #[fhir(name="url", min="1", max="1", summary=true, modifier=false)]
    pub url: Option<CanonicalDt>,
    /// source | queried | target | produced
    #[fhir(name="mode", min="1", max="1", summary=true, modifier=false)]
    pub mode: Option<CodeDt>,
    /// Name for type in this map
    #[fhir(name="alias", min="0", max="1", summary=true, modifier=false)]
    pub alias: Option<StringDt>,
    /// Documentation on use of structure
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false)]
    pub documentation: Option<StringDt>,
}


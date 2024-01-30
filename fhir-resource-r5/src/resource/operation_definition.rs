use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct OperationDefinition {
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
    /// Canonical identifier for this operation definition, represented as an absolute URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the implementation guide (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the operation definition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this operation definition (computer friendly)
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name for this operation definition (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// operation | query
    #[fhir(name="kind", min="1", max="1", summary=true, modifier=false)]
    pub kind: Option<CodeDt>,
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
    /// Natural language description of the operation definition
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for operation definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this operation definition is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false)]
    pub copyright_label: Option<StringDt>,
    /// Whether content is changed by the operation
    #[fhir(name="affectsState", min="0", max="1", summary=true, modifier=false)]
    pub affects_state: Option<BooleanDt>,
    /// Recommended name for operation in search url
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
    /// Additional information about use
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false)]
    pub comment: Option<MarkdownDt>,
    /// Marks this as a profile of the base
    #[fhir(name="base", min="0", max="1", summary=true, modifier=false)]
    pub base: Option<CanonicalDt>,
    /// Types this operation applies to
    #[fhir(name="resource", min="0", max="*", summary=true, modifier=false)]
    pub resource: Option<Vec<CodeDt>>,
    /// Invoke at the system level?
    #[fhir(name="system", min="1", max="1", summary=true, modifier=false)]
    pub system: Option<BooleanDt>,
    /// Invoke at the type level?
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<BooleanDt>,
    /// Invoke on an instance?
    #[fhir(name="instance", min="1", max="1", summary=true, modifier=false)]
    pub instance: Option<BooleanDt>,
    /// Validation information for in parameters
    #[fhir(name="inputProfile", min="0", max="1", summary=false, modifier=false)]
    pub input_profile: Option<CanonicalDt>,
    /// Validation information for out parameters
    #[fhir(name="outputProfile", min="0", max="1", summary=false, modifier=false)]
    pub output_profile: Option<CanonicalDt>,
    /// Parameters for the operation/query
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false)]
    pub parameter: Option<Vec<OperationDefinitionParameterBackboneElement>>,
    /// Define overloaded variants for when  generating code
    #[fhir(name="overload", min="0", max="*", summary=false, modifier=false)]
    pub overload: Option<Vec<OperationDefinitionOverloadBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct OperationDefinitionParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name in Parameters.parameter.name or in URL
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false)]
    pub name: Option<CodeDt>,
    /// in | out
    #[fhir(name="use", min="1", max="1", summary=false, modifier=false)]
    pub use_: Option<CodeDt>,
    /// instance | type | system
    #[fhir(name="scope", min="0", max="*", summary=false, modifier=false)]
    pub scope: Option<Vec<CodeDt>>,
    /// Minimum Cardinality
    #[fhir(name="min", min="1", max="1", summary=false, modifier=false)]
    pub min: Option<IntegerDt>,
    /// Maximum Cardinality (a number or *)
    #[fhir(name="max", min="1", max="1", summary=false, modifier=false)]
    pub max: Option<StringDt>,
    /// Description of meaning/use
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false)]
    pub documentation: Option<MarkdownDt>,
    /// What type this parameter has
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Allowed sub-type this parameter can have (if type is abstract)
    #[fhir(name="allowedType", min="0", max="*", summary=false, modifier=false)]
    pub allowed_type: Option<Vec<CodeDt>>,
    /// If type is Reference | canonical, allowed targets. If type is 'Resource', then this constrains the allowed resource types
    #[fhir(name="targetProfile", min="0", max="*", summary=false, modifier=false)]
    pub target_profile: Option<Vec<CanonicalDt>>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[fhir(name="searchType", min="0", max="1", summary=false, modifier=false)]
    pub search_type: Option<CodeDt>,
    /// ValueSet details if this is coded
    #[fhir(name="binding", min="0", max="1", summary=false, modifier=false)]
    pub binding: Option<OperationDefinitionParameterBindingBackboneElement>,
    /// References to this parameter
    #[fhir(name="referencedFrom", min="0", max="*", summary=false, modifier=false)]
    pub referenced_from: Option<Vec<OperationDefinitionParameterReferencedFromBackboneElement>>,
    /// Parts of a nested Parameter
    #[fhir(name="part", min="0", max="*", summary=false, modifier=false)]
    pub part: Option<Vec<OperationDefinitionParameterBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct OperationDefinitionParameterBindingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// required | extensible | preferred | example
    #[fhir(name="strength", min="1", max="1", summary=false, modifier=false)]
    pub strength: Option<CodeDt>,
    /// Source of value set
    #[fhir(name="valueSet", min="1", max="1", summary=false, modifier=false)]
    pub value_set: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct OperationDefinitionParameterReferencedFromBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Referencing parameter
    #[fhir(name="source", min="1", max="1", summary=false, modifier=false)]
    pub source: Option<StringDt>,
    /// Element id of reference
    #[fhir(name="sourceId", min="0", max="1", summary=false, modifier=false)]
    pub source_id: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct OperationDefinitionOverloadBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of parameter to include in overload
    #[fhir(name="parameterName", min="0", max="*", summary=false, modifier=false)]
    pub parameter_name: Option<Vec<StringDt>>,
    /// Comments to go on overload
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false)]
    pub comment: Option<StringDt>,
}


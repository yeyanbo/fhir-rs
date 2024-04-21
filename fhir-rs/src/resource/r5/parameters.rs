use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="Resource")]
pub struct Parameters {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub language: Option<CodeDt>,
    /// Operation Parameter
    #[fhir(name="parameter", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub parameter: Option<Vec<ParametersParameterBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ParametersParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name from the definition
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// If parameter is a data type
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub value: Option<Meta>,
    /// If parameter is a whole resource
    #[fhir(name="resource", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub resource: Option<AnyResource>,
    /// Named part of a multi-part parameter
    #[fhir(name="part", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub part: Option<Vec<ParametersParameterBackboneElement>>,
}


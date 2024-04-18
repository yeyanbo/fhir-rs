use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Substance {
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
    /// Unique identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Is this an instance of a substance or a kind of one
    #[fhir(name="instance", min="1", max="1", summary=true, modifier=true)]
    pub instance: Option<BooleanDt>,
    /// active | inactive | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// What class/type of substance this is
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// What substance this is
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableReference>,
    /// Textual description of the substance, comments
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// When no longer valid to use
    #[fhir(name="expiry", min="0", max="1", summary=true, modifier=false)]
    pub expiry: Option<DateTimeDt>,
    /// Amount of substance in the package
    #[fhir(name="quantity", min="0", max="1", summary=true, modifier=false)]
    pub quantity: Option<Quantity>,
    /// Composition information about the substance
    #[fhir(name="ingredient", min="0", max="*", summary=true, modifier=false)]
    pub ingredient: Option<Vec<SubstanceIngredientBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceIngredientBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Optional amount (concentration)
    #[fhir(name="quantity", min="0", max="1", summary=true, modifier=false)]
    pub quantity: Option<Ratio>,
    /// A component of the substance
    #[fhir(name="substance", min="1", max="1", summary=true, modifier=false)]
    pub substance: Option<Reference>,
}


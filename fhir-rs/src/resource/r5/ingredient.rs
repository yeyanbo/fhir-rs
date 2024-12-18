use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Ingredient {
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
    /// An identifier or code by which the ingredient can be referenced
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// The product which this ingredient is a constituent part of
    #[fhir(name="for", min="0", max="*", summary=true, modifier=false, choice="")]
    pub for_: Option<Vec<Reference>>,
    /// Purpose of the ingredient within the product, e.g. active, inactive
    #[fhir(name="role", min="1", max="1", summary=true, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
    /// Precise action within the drug product, e.g. antioxidant, alkalizing agent
    #[fhir(name="function", min="0", max="*", summary=true, modifier=false, choice="")]
    pub function: Option<Vec<CodeableConcept>>,
    /// A classification of the ingredient according to where in the physical item it tends to be used, such the outer shell of a tablet, inner body or ink
    #[fhir(name="group", min="0", max="1", summary=true, modifier=false, choice="")]
    pub group: Option<CodeableConcept>,
    /// If the ingredient is a known or suspected allergen
    #[fhir(name="allergenicIndicator", min="0", max="1", summary=true, modifier=false, choice="")]
    pub allergenic_indicator: Option<BooleanDt>,
    /// A place for providing any notes that are relevant to the component, e.g. removed during process, adjusted for loss on drying
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub comment: Option<MarkdownDt>,
    /// An organization that manufactures this ingredient
    #[fhir(name="manufacturer", min="0", max="*", summary=true, modifier=false, choice="")]
    pub manufacturer: Option<Vec<IngredientManufacturerBackboneElement>>,
    /// The substance that comprises this ingredient
    #[fhir(name="substance", min="1", max="1", summary=true, modifier=false, choice="")]
    pub substance: Option<IngredientSubstanceBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct IngredientManufacturerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// allowed | possible | actual
    #[fhir(name="role", min="0", max="1", summary=true, modifier=false, choice="")]
    pub role: Option<CodeDt>,
    /// An organization that manufactures this ingredient
    #[fhir(name="manufacturer", min="1", max="1", summary=true, modifier=false, choice="")]
    pub manufacturer: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct IngredientSubstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A code or full resource that represents the ingredient substance
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableReference>,
    /// The quantity of substance, per presentation, or per volume or mass, and type of quantity
    #[fhir(name="strength", min="0", max="*", summary=true, modifier=false, choice="")]
    pub strength: Option<Vec<IngredientSubstanceStrengthBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct IngredientSubstanceStrengthBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The quantity of substance in the unit of presentation
    #[fhir(name="presentation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub presentation: Option<Quantity>,
    /// Text of either the whole presentation strength or a part of it (rest being in Strength.presentation as a ratio)
    #[fhir(name="textPresentation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub text_presentation: Option<StringDt>,
    /// The strength per unitary volume (or mass)
    #[fhir(name="concentration", min="0", max="1", summary=true, modifier=false, choice="")]
    pub concentration: Option<Quantity>,
    /// Text of either the whole concentration strength or a part of it (rest being in Strength.concentration as a ratio)
    #[fhir(name="textConcentration", min="0", max="1", summary=true, modifier=false, choice="")]
    pub text_concentration: Option<StringDt>,
    /// A code that indicates if the strength is, for example, based on the ingredient substance as stated or on the substance base (when the ingredient is a salt)
    #[fhir(name="basis", min="0", max="1", summary=true, modifier=false, choice="")]
    pub basis: Option<CodeableConcept>,
    /// When strength is measured at a particular point or distance
    #[fhir(name="measurementPoint", min="0", max="1", summary=true, modifier=false, choice="")]
    pub measurement_point: Option<StringDt>,
    /// Where the strength range applies
    #[fhir(name="country", min="0", max="*", summary=true, modifier=false, choice="")]
    pub country: Option<Vec<CodeableConcept>>,
    /// Strength expressed in terms of a reference substance
    #[fhir(name="referenceStrength", min="0", max="*", summary=true, modifier=false, choice="")]
    pub reference_strength: Option<Vec<IngredientSubstanceStrengthReferenceStrengthBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct IngredientSubstanceStrengthReferenceStrengthBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Relevant reference substance
    #[fhir(name="substance", min="1", max="1", summary=true, modifier=false, choice="")]
    pub substance: Option<CodeableReference>,
    /// Strength expressed in terms of a reference substance
    #[fhir(name="strength", min="1", max="1", summary=true, modifier=false, choice="")]
    pub strength: Option<Quantity>,
    /// When strength is measured at a particular point or distance
    #[fhir(name="measurementPoint", min="0", max="1", summary=true, modifier=false, choice="")]
    pub measurement_point: Option<StringDt>,
    /// Where the strength range applies
    #[fhir(name="country", min="0", max="*", summary=true, modifier=false, choice="")]
    pub country: Option<Vec<CodeableConcept>>,
}


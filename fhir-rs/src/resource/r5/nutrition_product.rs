use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct NutritionProduct {
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
    /// A code that can identify the detailed nutrients and ingredients in a specific food product
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// active | inactive | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Broad product groups or categories used to classify the product, such as Legume and Legume Products, Beverages, or Beef Products
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// Manufacturer, representative or officially responsible for the product
    #[fhir(name="manufacturer", min="0", max="*", summary=true, modifier=false)]
    pub manufacturer: Option<Vec<Reference>>,
    /// The product's nutritional information expressed by the nutrients
    #[fhir(name="nutrient", min="0", max="*", summary=true, modifier=false)]
    pub nutrient: Option<Vec<NutritionProductNutrientBackboneElement>>,
    /// Ingredients contained in this product
    #[fhir(name="ingredient", min="0", max="*", summary=false, modifier=false)]
    pub ingredient: Option<Vec<NutritionProductIngredientBackboneElement>>,
    /// Known or suspected allergens that are a part of this product
    #[fhir(name="knownAllergen", min="0", max="*", summary=false, modifier=false)]
    pub known_allergen: Option<Vec<CodeableReference>>,
    /// Specifies descriptive properties of the nutrition product
    #[fhir(name="characteristic", min="0", max="*", summary=false, modifier=false)]
    pub characteristic: Option<Vec<NutritionProductCharacteristicBackboneElement>>,
    /// One or several physical instances or occurrences of the nutrition product
    #[fhir(name="instance", min="0", max="*", summary=false, modifier=false)]
    pub instance: Option<Vec<NutritionProductInstanceBackboneElement>>,
    /// Comments made about the product
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionProductIngredientBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The ingredient contained in the product
    #[fhir(name="item", min="1", max="1", summary=true, modifier=false)]
    pub item: Option<CodeableReference>,
    /// The amount of ingredient that is in the product
    #[fhir(name="amount", min="0", max="*", summary=true, modifier=false)]
    pub amount: Option<Vec<Ratio>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionProductCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code specifying the type of characteristic
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The value of the characteristic
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false)]
    pub value: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionProductInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The amount of items or instances
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false)]
    pub quantity: Option<Quantity>,
    /// The identifier for the physical instance, typically a serial number or manufacturer number
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// The name for the specific product
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false)]
    pub name: Option<StringDt>,
    /// The identification of the batch or lot of the product
    #[fhir(name="lotNumber", min="0", max="1", summary=false, modifier=false)]
    pub lot_number: Option<StringDt>,
    /// The expiry date or date and time for the product
    #[fhir(name="expiry", min="0", max="1", summary=false, modifier=false)]
    pub expiry: Option<DateTimeDt>,
    /// The date until which the product is expected to be good for consumption
    #[fhir(name="useBy", min="0", max="1", summary=false, modifier=false)]
    pub use_by: Option<DateTimeDt>,
    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[fhir(name="biologicalSourceEvent", min="0", max="1", summary=false, modifier=false)]
    pub biological_source_event: Option<Identifier>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionProductNutrientBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The (relevant) nutrients in the product
    #[fhir(name="item", min="0", max="1", summary=false, modifier=false)]
    pub item: Option<CodeableReference>,
    /// The amount of nutrient expressed in one or more units: X per pack / per serving / per dose
    #[fhir(name="amount", min="0", max="*", summary=false, modifier=false)]
    pub amount: Option<Vec<Ratio>>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct NutritionIntake {
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
    /// External identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=false, modifier=false, choice="")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=false, modifier=false, choice="")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// Fulfils plan, proposal or order
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// Code representing an overall type of nutrition intake
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Who is/was consuming the food or fluid
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Encounter associated with NutritionIntake
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// The date/time or interval when the food or fluid is/was consumed
    #[fhir(name="occurrence", min="0", max="1", summary=true, modifier=false, choice="")]
    pub occurrence: Option<Period>,
    /// When the intake was recorded
    #[fhir(name="recorded", min="0", max="1", summary=true, modifier=false, choice="")]
    pub recorded: Option<DateTimeDt>,
    /// Person or organization that provided the information about the consumption of this food or fluid
    #[fhir(name="reported", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reported: Option<Reference>,
    /// What food or fluid product or item was consumed
    #[fhir(name="consumedItem", min="1", max="*", summary=false, modifier=false, choice="")]
    pub consumed_item: Option<Vec<NutritionIntakeConsumedItemBackboneElement>>,
    /// Total nutrient for the whole meal, product, serving
    #[fhir(name="ingredientLabel", min="0", max="*", summary=false, modifier=false, choice="")]
    pub ingredient_label: Option<Vec<NutritionIntakeIngredientLabelBackboneElement>>,
    /// Who was performed in the intake
    #[fhir(name="performer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub performer: Option<Vec<NutritionIntakePerformerBackboneElement>>,
    /// Where the intake occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// Additional supporting information
    #[fhir(name="derivedFrom", min="0", max="*", summary=false, modifier=false, choice="")]
    pub derived_from: Option<Vec<Reference>>,
    /// Reason for why the food or fluid is /was consumed
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Further information about the consumption
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionIntakePerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of performer
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false, choice="")]
    pub function: Option<CodeableConcept>,
    /// Who performed the intake
    #[fhir(name="actor", min="1", max="1", summary=false, modifier=false, choice="")]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionIntakeIngredientLabelBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Total nutrient consumed
    #[fhir(name="nutrient", min="1", max="1", summary=false, modifier=false, choice="")]
    pub nutrient: Option<CodeableReference>,
    /// Total amount of nutrient consumed
    #[fhir(name="amount", min="1", max="1", summary=false, modifier=false, choice="")]
    pub amount: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionIntakeConsumedItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of food or fluid product
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Code that identifies the food or fluid product that was consumed
    #[fhir(name="nutritionProduct", min="1", max="1", summary=true, modifier=false, choice="")]
    pub nutrition_product: Option<CodeableReference>,
    /// Scheduled frequency of consumption
    #[fhir(name="schedule", min="0", max="1", summary=false, modifier=false, choice="")]
    pub schedule: Option<Timing>,
    /// Quantity of the specified food
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false, choice="")]
    pub amount: Option<Quantity>,
    /// Rate at which enteral feeding was administered
    #[fhir(name="rate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub rate: Option<Quantity>,
    /// Flag to indicate if the food or fluid item was refused or otherwise not consumed
    #[fhir(name="notConsumed", min="0", max="1", summary=false, modifier=false, choice="")]
    pub not_consumed: Option<BooleanDt>,
    /// Reason food or fluid was not consumed
    #[fhir(name="notConsumedReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub not_consumed_reason: Option<CodeableConcept>,
}


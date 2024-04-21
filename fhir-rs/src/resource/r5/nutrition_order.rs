use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct NutritionOrder {
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
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifiers assigned to this order
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// Instantiates protocol or definition
    #[fhir(name="instantiates", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub instantiates: Option<Vec<UriDt>>,
    /// What this order fulfills
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub based_on: Option<Vec<Reference>>,
    /// Composite Request ID
    #[fhir(name="groupIdentifier", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary=true, modifier=true)]
    pub intent: Option<CodeDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub priority: Option<CodeDt>,
    /// Who requires the diet, formula or nutritional supplement
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// The encounter associated with this nutrition order
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub encounter: Option<Reference>,
    /// Information to support fulfilling of the nutrition order
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supporting_information: Option<Vec<Reference>>,
    /// Date and time the nutrition order was requested
    #[fhir(name="dateTime", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub date_time: Option<DateTimeDt>,
    /// Who ordered the diet, formula or nutritional supplement
    #[fhir(name="orderer", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub orderer: Option<Reference>,
    /// Who is desired to perform the administration of what is being ordered
    #[fhir(name="performer", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub performer: Option<Vec<CodeableReference>>,
    /// List of the patient's food and nutrition-related allergies and intolerances
    #[fhir(name="allergyIntolerance", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub allergy_intolerance: Option<Vec<Reference>>,
    /// Order-specific modifier about the type of food that should be given
    #[fhir(name="foodPreferenceModifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub food_preference_modifier: Option<Vec<CodeableConcept>>,
    /// Order-specific modifier about the type of food that should not be given
    #[fhir(name="excludeFoodModifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub exclude_food_modifier: Option<Vec<CodeableConcept>>,
    /// Capture when a food item is brought in by the patient and/or family
    #[fhir(name="outsideFoodAllowed", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub outside_food_allowed: Option<BooleanDt>,
    /// Oral diet components
    #[fhir(name="oralDiet", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub oral_diet: Option<NutritionOrderOralDietBackboneElement>,
    /// Supplement components
    #[fhir(name="supplement", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supplement: Option<Vec<NutritionOrderSupplementBackboneElement>>,
    /// Enteral formula components
    #[fhir(name="enteralFormula", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub enteral_formula: Option<NutritionOrderEnteralFormulaBackboneElement>,
    /// Comments
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderSupplementBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of supplement product requested
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub type_: Option<CodeableReference>,
    /// Product or brand name of the nutritional supplement
    #[fhir(name="productName", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_name: Option<StringDt>,
    /// Scheduling information for supplements
    #[fhir(name="schedule", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub schedule: Option<NutritionOrderSupplementScheduleBackboneElement>,
    /// Amount of the nutritional supplement
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
    /// Instructions or additional information about the oral supplement
    #[fhir(name="instruction", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub instruction: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderSupplementScheduleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Scheduled frequency of diet
    #[fhir(name="timing", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub timing: Option<Vec<Timing>>,
    /// Take 'as needed'
    #[fhir(name="asNeeded", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub as_needed: Option<BooleanDt>,
    /// Take 'as needed' for x
    #[fhir(name="asNeededFor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub as_needed_for: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderEnteralFormulaBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of enteral or infant formula
    #[fhir(name="baseFormulaType", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub base_formula_type: Option<CodeableReference>,
    /// Product or brand name of the enteral or infant formula
    #[fhir(name="baseFormulaProductName", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub base_formula_product_name: Option<StringDt>,
    /// Intended type of device for the administration
    #[fhir(name="deliveryDevice", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub delivery_device: Option<Vec<CodeableReference>>,
    /// Components to add to the feeding
    #[fhir(name="additive", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub additive: Option<Vec<NutritionOrderEnteralFormulaAdditiveBackboneElement>>,
    /// Amount of energy per specified volume that is required
    #[fhir(name="caloricDensity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub caloric_density: Option<Quantity>,
    /// How the formula should enter the patient's gastrointestinal tract
    #[fhir(name="routeOfAdministration", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub route_of_administration: Option<CodeableConcept>,
    /// Formula feeding instruction as structured data
    #[fhir(name="administration", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub administration: Option<Vec<NutritionOrderEnteralFormulaAdministrationBackboneElement>>,
    /// Upper limit on formula volume per unit of time
    #[fhir(name="maxVolumeToDeliver", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub max_volume_to_deliver: Option<Quantity>,
    /// Formula feeding instructions expressed as text
    #[fhir(name="administrationInstruction", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub administration_instruction: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderEnteralFormulaAdministrationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Scheduling information for enteral formula products
    #[fhir(name="schedule", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub schedule: Option<NutritionOrderEnteralFormulaAdministrationScheduleBackboneElement>,
    /// The volume of formula to provide
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
    /// Speed with which the formula is provided per period of time
    #[fhir(name="rate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub rate: Option<Ratio>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderEnteralFormulaAdministrationScheduleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Scheduled frequency of enteral formula
    #[fhir(name="timing", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub timing: Option<Vec<Timing>>,
    /// Take 'as needed'
    #[fhir(name="asNeeded", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub as_needed: Option<BooleanDt>,
    /// Take 'as needed' for x
    #[fhir(name="asNeededFor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub as_needed_for: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderEnteralFormulaAdditiveBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of modular component to add to the feeding
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableReference>,
    /// Product or brand name of the modular additive
    #[fhir(name="productName", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_name: Option<StringDt>,
    /// Amount of additive to be given or mixed in
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderOralDietBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of oral diet or diet restrictions that describe what can be consumed orally
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Scheduling information for oral diets
    #[fhir(name="schedule", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub schedule: Option<NutritionOrderOralDietScheduleBackboneElement>,
    /// Required  nutrient modifications
    #[fhir(name="nutrient", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub nutrient: Option<Vec<NutritionOrderOralDietNutrientBackboneElement>>,
    /// Required  texture modifications
    #[fhir(name="texture", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub texture: Option<Vec<NutritionOrderOralDietTextureBackboneElement>>,
    /// The required consistency of fluids and liquids provided to the patient
    #[fhir(name="fluidConsistencyType", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub fluid_consistency_type: Option<Vec<CodeableConcept>>,
    /// Instructions or additional information about the oral diet
    #[fhir(name="instruction", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub instruction: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderOralDietNutrientBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of nutrient that is being modified
    #[fhir(name="modifier", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub modifier: Option<CodeableConcept>,
    /// Quantity of the specified nutrient
    #[fhir(name="amount", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub amount: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderOralDietScheduleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Scheduled frequency of diet
    #[fhir(name="timing", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub timing: Option<Vec<Timing>>,
    /// Take 'as needed'
    #[fhir(name="asNeeded", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub as_needed: Option<BooleanDt>,
    /// Take 'as needed' for x
    #[fhir(name="asNeededFor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub as_needed_for: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct NutritionOrderOralDietTextureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code to indicate how to alter the texture of the foods, e.g. pureed
    #[fhir(name="modifier", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub modifier: Option<CodeableConcept>,
    /// Concepts that are used to identify an entity that is ingested for nutritional purposes
    #[fhir(name="foodType", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub food_type: Option<CodeableConcept>,
}


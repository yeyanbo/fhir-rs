use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Medication {
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
    /// Business identifier for this medication
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Codes that identify this medication
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// active | inactive | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Organization that has authorization to market medication
    #[fhir(name="marketingAuthorizationHolder", min="0", max="1", summary=true, modifier=false, choice="")]
    pub marketing_authorization_holder: Option<Reference>,
    /// powder | tablets | capsule +
    #[fhir(name="doseForm", min="0", max="1", summary=false, modifier=false, choice="")]
    pub dose_form: Option<CodeableConcept>,
    /// When the specified product code does not infer a package size, this is the specific amount of drug in the product
    #[fhir(name="totalVolume", min="0", max="1", summary=true, modifier=false, choice="")]
    pub total_volume: Option<Quantity>,
    /// Active or inactive ingredient
    #[fhir(name="ingredient", min="0", max="*", summary=false, modifier=false, choice="")]
    pub ingredient: Option<Vec<MedicationIngredientBackboneElement>>,
    /// Details about packaged medications
    #[fhir(name="batch", min="0", max="1", summary=false, modifier=false, choice="")]
    pub batch: Option<MedicationBatchBackboneElement>,
    /// Knowledge about this medication
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub definition: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationBatchBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifier assigned to batch
    #[fhir(name="lotNumber", min="0", max="1", summary=false, modifier=false, choice="")]
    pub lot_number: Option<StringDt>,
    /// When batch will expire
    #[fhir(name="expirationDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub expiration_date: Option<DateTimeDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationIngredientBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The ingredient (substance or medication) that the ingredient.strength relates to
    #[fhir(name="item", min="1", max="1", summary=false, modifier=false, choice="")]
    pub item: Option<CodeableReference>,
    /// Active ingredient indicator
    #[fhir(name="isActive", min="0", max="1", summary=false, modifier=false, choice="")]
    pub is_active: Option<BooleanDt>,
    /// Quantity of ingredient present
    #[fhir(name="strength", min="0", max="1", summary=false, modifier=false, choice="")]
    pub strength: Option<Quantity>,
}


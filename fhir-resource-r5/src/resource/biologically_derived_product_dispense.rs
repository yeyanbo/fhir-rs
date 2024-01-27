use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct BiologicallyDerivedProductDispense {
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
    /// Business identifier for this dispense
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// The order or request that this dispense is fulfilling
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// Short description
    #[fhir(name="partOf", min="0", max="*", summary="true", modifier="false")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | allocated | issued | unfulfilled | returned | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="false")]
    pub status: Option<CodeDt>,
    /// Relationship between the donor and intended recipient
    #[fhir(name="originRelationshipType", min="0", max="1", summary="true", modifier="false")]
    pub origin_relationship_type: Option<CodeableConcept>,
    /// The BiologicallyDerivedProduct that is dispensed
    #[fhir(name="product", min="1", max="1", summary="true", modifier="false")]
    pub product: Option<Reference>,
    /// The intended recipient of the dispensed product
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
    /// Indicates the type of matching associated with the dispense
    #[fhir(name="matchStatus", min="0", max="1", summary="true", modifier="false")]
    pub match_status: Option<CodeableConcept>,
    /// Indicates who or what performed an action
    #[fhir(name="performer", min="0", max="*", summary="true", modifier="false")]
    pub performer: Option<Vec<BiologicallyDerivedProductDispensePerformerBackboneElement>>,
    /// Where the dispense occurred
    #[fhir(name="location", min="0", max="1", summary="true", modifier="false")]
    pub location: Option<Reference>,
    /// Amount dispensed
    #[fhir(name="quantity", min="0", max="1", summary="true", modifier="false")]
    pub quantity: Option<Quantity>,
    /// When product was selected/matched
    #[fhir(name="preparedDate", min="0", max="1", summary="true", modifier="false")]
    pub prepared_date: Option<DateTimeDt>,
    /// When the product was dispatched
    #[fhir(name="whenHandedOver", min="0", max="1", summary="true", modifier="false")]
    pub when_handed_over: Option<DateTimeDt>,
    /// Where the product was dispatched to
    #[fhir(name="destination", min="0", max="1", summary="true", modifier="false")]
    pub destination: Option<Reference>,
    /// Additional notes
    #[fhir(name="note", min="0", max="*", summary="true", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Specific instructions for use
    #[fhir(name="usageInstruction", min="0", max="1", summary="true", modifier="false")]
    pub usage_instruction: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BiologicallyDerivedProductDispensePerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifies the function of the performer during the dispense
    #[fhir(name="function", min="0", max="1", summary="true", modifier="false")]
    pub function: Option<CodeableConcept>,
    /// Who performed the action
    #[fhir(name="actor", min="1", max="1", summary="true", modifier="false")]
    pub actor: Option<Reference>,
}


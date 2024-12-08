use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MedicationDispense {
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
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Plan that is fulfilled by this dispense
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Event that dispense is part of
    #[fhir(name="partOf", min="0", max="*", summary=false, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Why a dispense was not performed
    #[fhir(name="notPerformedReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub not_performed_reason: Option<CodeableReference>,
    /// When the status changed
    #[fhir(name="statusChanged", min="0", max="1", summary=false, modifier=false, choice="")]
    pub status_changed: Option<DateTimeDt>,
    /// Type of medication dispense
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// What medication was supplied
    #[fhir(name="medication", min="1", max="1", summary=true, modifier=false, choice="")]
    pub medication: Option<CodeableReference>,
    /// Who the dispense is for
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Encounter associated with event
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Information that supports the dispensing of the medication
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_information: Option<Vec<Reference>>,
    /// Who performed event
    #[fhir(name="performer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub performer: Option<Vec<MedicationDispensePerformerBackboneElement>>,
    /// Where the dispense occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// Medication order that authorizes the dispense
    #[fhir(name="authorizingPrescription", min="0", max="*", summary=false, modifier=false, choice="")]
    pub authorizing_prescription: Option<Vec<Reference>>,
    /// Trial fill, partial fill, emergency fill, etc
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Amount dispensed
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Amount of medication expressed as a timing amount
    #[fhir(name="daysSupply", min="0", max="1", summary=false, modifier=false, choice="")]
    pub days_supply: Option<Quantity>,
    /// When the recording of the dispense started
    #[fhir(name="recorded", min="0", max="1", summary=false, modifier=false, choice="")]
    pub recorded: Option<DateTimeDt>,
    /// When product was packaged and reviewed
    #[fhir(name="whenPrepared", min="0", max="1", summary=true, modifier=false, choice="")]
    pub when_prepared: Option<DateTimeDt>,
    /// When product was given out
    #[fhir(name="whenHandedOver", min="0", max="1", summary=false, modifier=false, choice="")]
    pub when_handed_over: Option<DateTimeDt>,
    /// Where the medication was/will be sent
    #[fhir(name="destination", min="0", max="1", summary=false, modifier=false, choice="")]
    pub destination: Option<Reference>,
    /// Who collected the medication or where the medication was delivered
    #[fhir(name="receiver", min="0", max="*", summary=false, modifier=false, choice="")]
    pub receiver: Option<Vec<Reference>>,
    /// Information about the dispense
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Full representation of the dosage instructions
    #[fhir(name="renderedDosageInstruction", min="0", max="1", summary=false, modifier=false, choice="")]
    pub rendered_dosage_instruction: Option<MarkdownDt>,
    /// How the medication is to be used by the patient or administered by the caregiver
    #[fhir(name="dosageInstruction", min="0", max="*", summary=false, modifier=false, choice="")]
    pub dosage_instruction: Option<Vec<Dosage>>,
    /// Whether a substitution was performed on the dispense
    #[fhir(name="substitution", min="0", max="1", summary=false, modifier=false, choice="")]
    pub substitution: Option<MedicationDispenseSubstitutionBackboneElement>,
    /// A list of relevant lifecycle events
    #[fhir(name="eventHistory", min="0", max="*", summary=false, modifier=false, choice="")]
    pub event_history: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationDispenseSubstitutionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether a substitution was or was not performed on the dispense
    #[fhir(name="wasSubstituted", min="1", max="1", summary=false, modifier=false, choice="")]
    pub was_substituted: Option<BooleanDt>,
    /// Code signifying whether a different drug was dispensed from what was prescribed
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Why was substitution made
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableConcept>>,
    /// Who is responsible for the substitution
    #[fhir(name="responsibleParty", min="0", max="1", summary=false, modifier=false, choice="")]
    pub responsible_party: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationDispensePerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Who performed the dispense and what they did
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false, choice="")]
    pub function: Option<CodeableConcept>,
    /// Individual who was performing
    #[fhir(name="actor", min="1", max="1", summary=false, modifier=false, choice="")]
    pub actor: Option<Reference>,
}


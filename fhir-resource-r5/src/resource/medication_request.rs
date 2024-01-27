use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MedicationRequest {
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
    /// External ids for this request
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// A plan or request that is fulfilled in whole or in part by this medication request
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// Reference to an order/prescription that is being replaced by this MedicationRequest
    #[fhir(name="priorPrescription", min="0", max="1", summary="false", modifier="false")]
    pub prior_prescription: Option<Reference>,
    /// Composite request this is part of
    #[fhir(name="groupIdentifier", min="0", max="1", summary="true", modifier="false")]
    pub group_identifier: Option<Identifier>,
    /// active | on-hold | ended | stopped | completed | cancelled | entered-in-error | draft | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary="false", modifier="false")]
    pub status_reason: Option<CodeableConcept>,
    /// When the status was changed
    #[fhir(name="statusChanged", min="0", max="1", summary="false", modifier="false")]
    pub status_changed: Option<DateTimeDt>,
    /// proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary="true", modifier="true")]
    pub intent: Option<CodeDt>,
    /// Grouping or category of medication request
    #[fhir(name="category", min="0", max="*", summary="false", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="true", modifier="false")]
    pub priority: Option<CodeDt>,
    /// True if patient is to stop taking or not to start taking the medication
    #[fhir(name="doNotPerform", min="0", max="1", summary="true", modifier="true")]
    pub do_not_perform: Option<BooleanDt>,
    /// Medication to be taken
    #[fhir(name="medication", min="1", max="1", summary="true", modifier="false")]
    pub medication: Option<CodeableReference>,
    /// Individual or group for whom the medication has been requested
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// The person or organization who provided the information about this request, if the source is someone other than the requestor
    #[fhir(name="informationSource", min="0", max="*", summary="false", modifier="false")]
    pub information_source: Option<Vec<Reference>>,
    /// Encounter created as part of encounter/admission/stay
    #[fhir(name="encounter", min="0", max="1", summary="false", modifier="false")]
    pub encounter: Option<Reference>,
    /// Information to support fulfilling of the medication
    #[fhir(name="supportingInformation", min="0", max="*", summary="false", modifier="false")]
    pub supporting_information: Option<Vec<Reference>>,
    /// When request was initially authored
    #[fhir(name="authoredOn", min="0", max="1", summary="true", modifier="false")]
    pub authored_on: Option<DateTimeDt>,
    /// Who/What requested the Request
    #[fhir(name="requester", min="0", max="1", summary="true", modifier="false")]
    pub requester: Option<Reference>,
    /// Reported rather than primary record
    #[fhir(name="reported", min="0", max="1", summary="true", modifier="false")]
    pub reported: Option<BooleanDt>,
    /// Desired kind of performer of the medication administration
    #[fhir(name="performerType", min="0", max="1", summary="true", modifier="false")]
    pub performer_type: Option<CodeableConcept>,
    /// Intended performer of administration
    #[fhir(name="performer", min="0", max="*", summary="false", modifier="false")]
    pub performer: Option<Vec<Reference>>,
    /// Intended type of device for the administration
    #[fhir(name="device", min="0", max="*", summary="false", modifier="false")]
    pub device: Option<Vec<CodeableReference>>,
    /// Person who entered the request
    #[fhir(name="recorder", min="0", max="1", summary="false", modifier="false")]
    pub recorder: Option<Reference>,
    /// Reason or indication for ordering or not ordering the medication
    #[fhir(name="reason", min="0", max="*", summary="false", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Overall pattern of medication administration
    #[fhir(name="courseOfTherapyType", min="0", max="1", summary="false", modifier="false")]
    pub course_of_therapy_type: Option<CodeableConcept>,
    /// Associated insurance coverage
    #[fhir(name="insurance", min="0", max="*", summary="false", modifier="false")]
    pub insurance: Option<Vec<Reference>>,
    /// Information about the prescription
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Full representation of the dosage instructions
    #[fhir(name="renderedDosageInstruction", min="0", max="1", summary="false", modifier="false")]
    pub rendered_dosage_instruction: Option<MarkdownDt>,
    /// Period over which the medication is to be taken
    #[fhir(name="effectiveDosePeriod", min="0", max="1", summary="false", modifier="false")]
    pub effective_dose_period: Option<Period>,
    /// Specific instructions for how the medication should be taken
    #[fhir(name="dosageInstruction", min="0", max="*", summary="false", modifier="false")]
    pub dosage_instruction: Option<Vec<Dosage>>,
    /// Medication supply authorization
    #[fhir(name="dispenseRequest", min="0", max="1", summary="false", modifier="false")]
    pub dispense_request: Option<MedicationRequestDispenseRequestBackboneElement>,
    /// Any restrictions on medication substitution
    #[fhir(name="substitution", min="0", max="1", summary="false", modifier="false")]
    pub substitution: Option<MedicationRequestSubstitutionBackboneElement>,
    /// A list of events of interest in the lifecycle
    #[fhir(name="eventHistory", min="0", max="*", summary="false", modifier="false")]
    pub event_history: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicationRequestDispenseRequestBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// First fill details
    #[fhir(name="initialFill", min="0", max="1", summary="false", modifier="false")]
    pub initial_fill: Option<MedicationRequestDispenseRequestInitialFillBackboneElement>,
    /// Minimum period of time between dispenses
    #[fhir(name="dispenseInterval", min="0", max="1", summary="false", modifier="false")]
    pub dispense_interval: Option<Duration>,
    /// Time period supply is authorized for
    #[fhir(name="validityPeriod", min="0", max="1", summary="false", modifier="false")]
    pub validity_period: Option<Period>,
    /// Number of refills authorized
    #[fhir(name="numberOfRepeatsAllowed", min="0", max="1", summary="false", modifier="false")]
    pub number_of_repeats_allowed: Option<UnsignedIntDt>,
    /// Amount of medication to supply per dispense
    #[fhir(name="quantity", min="0", max="1", summary="false", modifier="false")]
    pub quantity: Option<Quantity>,
    /// Number of days supply per dispense
    #[fhir(name="expectedSupplyDuration", min="0", max="1", summary="false", modifier="false")]
    pub expected_supply_duration: Option<Duration>,
    /// Intended performer of dispense
    #[fhir(name="dispenser", min="0", max="1", summary="false", modifier="false")]
    pub dispenser: Option<Reference>,
    /// Additional information for the dispenser
    #[fhir(name="dispenserInstruction", min="0", max="*", summary="false", modifier="false")]
    pub dispenser_instruction: Option<Vec<Annotation>>,
    /// Type of adherence packaging to use for the dispense
    #[fhir(name="doseAdministrationAid", min="0", max="1", summary="false", modifier="false")]
    pub dose_administration_aid: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicationRequestDispenseRequestInitialFillBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// First fill quantity
    #[fhir(name="quantity", min="0", max="1", summary="false", modifier="false")]
    pub quantity: Option<Quantity>,
    /// First fill duration
    #[fhir(name="duration", min="0", max="1", summary="false", modifier="false")]
    pub duration: Option<Duration>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicationRequestSubstitutionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether substitution is allowed or not
    #[fhir(name="allowed", min="1", max="1", summary="false", modifier="false")]
    pub allowed: Option<CodeableConcept>,
    /// Why should (not) substitution be made
    #[fhir(name="reason", min="0", max="1", summary="false", modifier="false")]
    pub reason: Option<CodeableConcept>,
}


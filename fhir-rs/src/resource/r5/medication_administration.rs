use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MedicationAdministration {
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
    /// Plan this is fulfilled by this administration
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// in-progress | not-done | on-hold | completed | entered-in-error | stopped | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason administration not performed
    #[fhir(name="statusReason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// Type of medication administration
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// What was administered
    #[fhir(name="medication", min="1", max="1", summary=true, modifier=false, choice="")]
    pub medication: Option<CodeableReference>,
    /// Who received medication
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Encounter administered as part of
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Additional information to support administration
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_information: Option<Vec<Reference>>,
    /// Specific date/time or interval of time during which the administration took place (or did not take place)
    #[fhir(name="occurence", min="1", max="1", summary=true, modifier=false, choice="")]
    pub occurence: Option<Timing>,
    /// When the MedicationAdministration was first captured in the subject's record
    #[fhir(name="recorded", min="0", max="1", summary=true, modifier=false, choice="")]
    pub recorded: Option<DateTimeDt>,
    /// Full dose was not administered
    #[fhir(name="isSubPotent", min="0", max="1", summary=false, modifier=false, choice="")]
    pub is_sub_potent: Option<BooleanDt>,
    /// Reason full dose was not administered
    #[fhir(name="subPotentReason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_potent_reason: Option<Vec<CodeableConcept>>,
    /// Who or what performed the medication administration and what type of performance they did
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false, choice="")]
    pub performer: Option<Vec<MedicationAdministrationPerformerBackboneElement>>,
    /// Concept, condition or observation that supports why the medication was administered
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Request administration performed against
    #[fhir(name="request", min="0", max="1", summary=false, modifier=false, choice="")]
    pub request: Option<Reference>,
    /// Device used to administer
    #[fhir(name="device", min="0", max="*", summary=false, modifier=false, choice="")]
    pub device: Option<Vec<CodeableReference>>,
    /// Information about the administration
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Details of how medication was taken
    #[fhir(name="dosage", min="0", max="1", summary=false, modifier=false, choice="")]
    pub dosage: Option<MedicationAdministrationDosageBackboneElement>,
    /// A list of events of interest in the lifecycle
    #[fhir(name="eventHistory", min="0", max="*", summary=false, modifier=false, choice="")]
    pub event_history: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationAdministrationDosageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Free text dosage instructions e.g. SIG
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
    /// Body site administered to
    #[fhir(name="site", min="0", max="1", summary=false, modifier=false, choice="")]
    pub site: Option<CodeableConcept>,
    /// Path of substance into body
    #[fhir(name="route", min="0", max="1", summary=false, modifier=false, choice="")]
    pub route: Option<CodeableConcept>,
    /// How drug was administered
    #[fhir(name="method", min="0", max="1", summary=false, modifier=false, choice="")]
    pub method: Option<CodeableConcept>,
    /// Amount of medication per dose
    #[fhir(name="dose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub dose: Option<Quantity>,
    /// Dose quantity per unit of time
    #[fhir(name="rate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub rate: Option<Quantity>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationAdministrationPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of performance
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false, choice="")]
    pub function: Option<CodeableConcept>,
    /// Who or what performed the medication administration
    #[fhir(name="actor", min="1", max="1", summary=true, modifier=false, choice="")]
    pub actor: Option<CodeableReference>,
}


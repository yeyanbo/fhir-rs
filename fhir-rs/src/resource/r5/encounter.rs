use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Encounter {
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
    /// Identifier(s) by which this encounter is known
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Classification of patient encounter context - e.g. Inpatient, outpatient
    #[fhir(name="class", min="0", max="*", summary=true, modifier=false, choice="")]
    pub class: Option<Vec<CodeableConcept>>,
    /// Indicates the urgency of the encounter
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice="")]
    pub priority: Option<CodeableConcept>,
    /// Specific type of encounter (e.g. e-mail consultation, surgical day-care, ...)
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Specific type of service
    #[fhir(name="serviceType", min="0", max="*", summary=true, modifier=false, choice="")]
    pub service_type: Option<Vec<CodeableReference>>,
    /// The patient or group related to this encounter
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// The current status of the subject in relation to the Encounter
    #[fhir(name="subjectStatus", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subject_status: Option<CodeableConcept>,
    /// Episode(s) of care that this encounter should be recorded against
    #[fhir(name="episodeOfCare", min="0", max="*", summary=true, modifier=false, choice="")]
    pub episode_of_care: Option<Vec<Reference>>,
    /// The request that initiated this encounter
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// The group(s) that are allocated to participate in this encounter
    #[fhir(name="careTeam", min="0", max="*", summary=false, modifier=false, choice="")]
    pub care_team: Option<Vec<Reference>>,
    /// Another Encounter this encounter is part of
    #[fhir(name="partOf", min="0", max="1", summary=false, modifier=false, choice="")]
    pub part_of: Option<Reference>,
    /// The organization (facility) responsible for this encounter
    #[fhir(name="serviceProvider", min="0", max="1", summary=false, modifier=false, choice="")]
    pub service_provider: Option<Reference>,
    /// List of participants involved in the encounter
    #[fhir(name="participant", min="0", max="*", summary=true, modifier=false, choice="")]
    pub participant: Option<Vec<EncounterParticipantBackboneElement>>,
    /// The appointment that scheduled this encounter
    #[fhir(name="appointment", min="0", max="*", summary=true, modifier=false, choice="")]
    pub appointment: Option<Vec<Reference>>,
    /// Connection details of a virtual service (e.g. conference call)
    #[fhir(name="virtualService", min="0", max="*", summary=false, modifier=false, choice="")]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,
    /// The actual start and end time of the encounter
    #[fhir(name="actualPeriod", min="0", max="1", summary=false, modifier=false, choice="")]
    pub actual_period: Option<Period>,
    /// The planned start date/time (or admission date) of the encounter
    #[fhir(name="plannedStartDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub planned_start_date: Option<DateTimeDt>,
    /// The planned end date/time (or discharge date) of the encounter
    #[fhir(name="plannedEndDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub planned_end_date: Option<DateTimeDt>,
    /// Actual quantity of time the encounter lasted (less time absent)
    #[fhir(name="length", min="0", max="1", summary=false, modifier=false, choice="")]
    pub length: Option<Duration>,
    /// The list of medical reasons that are expected to be addressed during the episode of care
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false, choice="")]
    pub reason: Option<Vec<EncounterReasonBackboneElement>>,
    /// The list of diagnosis relevant to this encounter
    #[fhir(name="diagnosis", min="0", max="*", summary=true, modifier=false, choice="")]
    pub diagnosis: Option<Vec<EncounterDiagnosisBackboneElement>>,
    /// The set of accounts that may be used for billing for this Encounter
    #[fhir(name="account", min="0", max="*", summary=false, modifier=false, choice="")]
    pub account: Option<Vec<Reference>>,
    /// Diet preferences reported by the patient
    #[fhir(name="dietPreference", min="0", max="*", summary=false, modifier=false, choice="")]
    pub diet_preference: Option<Vec<CodeableConcept>>,
    /// Wheelchair, translator, stretcher, etc
    #[fhir(name="specialArrangement", min="0", max="*", summary=false, modifier=false, choice="")]
    pub special_arrangement: Option<Vec<CodeableConcept>>,
    /// Special courtesies (VIP, board member)
    #[fhir(name="specialCourtesy", min="0", max="*", summary=false, modifier=false, choice="")]
    pub special_courtesy: Option<Vec<CodeableConcept>>,
    /// Details about the admission to a healthcare service
    #[fhir(name="admission", min="0", max="1", summary=false, modifier=false, choice="")]
    pub admission: Option<EncounterAdmissionBackboneElement>,
    /// List of locations where the patient has been
    #[fhir(name="location", min="0", max="*", summary=false, modifier=false, choice="")]
    pub location: Option<Vec<EncounterLocationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EncounterParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Role of participant in encounter
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Period of time during the encounter that the participant participated
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
    /// The individual, device, or service participating in the encounter
    #[fhir(name="actor", min="0", max="1", summary=true, modifier=false, choice="")]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EncounterLocationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location the encounter takes place
    #[fhir(name="location", min="1", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// planned | active | reserved | completed
    #[fhir(name="status", min="0", max="1", summary=false, modifier=false, choice="")]
    pub status: Option<CodeDt>,
    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    #[fhir(name="form", min="0", max="1", summary=false, modifier=false, choice="")]
    pub form: Option<CodeableConcept>,
    /// Time period during which the patient was present at the location
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EncounterAdmissionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pre-admission identifier
    #[fhir(name="preAdmissionIdentifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub pre_admission_identifier: Option<Identifier>,
    /// The location/organization from which the patient came before admission
    #[fhir(name="origin", min="0", max="1", summary=false, modifier=false, choice="")]
    pub origin: Option<Reference>,
    /// From where patient was admitted (physician referral, transfer)
    #[fhir(name="admitSource", min="0", max="1", summary=false, modifier=false, choice="")]
    pub admit_source: Option<CodeableConcept>,
    /// Indicates that the patient is being re-admitted
    #[fhir(name="reAdmission", min="0", max="1", summary=false, modifier=false, choice="")]
    pub re_admission: Option<CodeableConcept>,
    /// Location/organization to which the patient is discharged
    #[fhir(name="destination", min="0", max="1", summary=false, modifier=false, choice="")]
    pub destination: Option<Reference>,
    /// Category or kind of location after discharge
    #[fhir(name="dischargeDisposition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub discharge_disposition: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EncounterDiagnosisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The diagnosis relevant to the encounter
    #[fhir(name="condition", min="0", max="*", summary=true, modifier=false, choice="")]
    pub condition: Option<Vec<CodeableReference>>,
    /// Role that this diagnosis has within the encounter (e.g. admission, billing, discharge …)
    #[fhir(name="use", min="0", max="*", summary=false, modifier=false, choice="")]
    pub use_: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EncounterReasonBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What the reason value should be used for/as
    #[fhir(name="use", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_: Option<Vec<CodeableConcept>>,
    /// Reason the encounter takes place (core or reference)
    #[fhir(name="value", min="0", max="*", summary=true, modifier=false, choice="")]
    pub value: Option<Vec<CodeableReference>>,
}


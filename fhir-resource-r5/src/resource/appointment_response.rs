use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct AppointmentResponse {
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
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// External Ids for this item
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Appointment this response relates to
    #[fhir(name="appointment", min="1", max="1", summary="true", modifier="false")]
    pub appointment: Option<Reference>,
    /// Indicator for a counter proposal
    #[fhir(name="proposedNewTime", min="0", max="1", summary="true", modifier="false")]
    pub proposed_new_time: Option<BooleanDt>,
    /// Time from appointment, or requested new start time
    #[fhir(name="start", min="0", max="1", summary="false", modifier="false")]
    pub start: Option<InstantDt>,
    /// Time from appointment, or requested new end time
    #[fhir(name="end", min="0", max="1", summary="false", modifier="false")]
    pub end: Option<InstantDt>,
    /// Role of participant in the appointment
    #[fhir(name="participantType", min="0", max="*", summary="true", modifier="false")]
    pub participant_type: Option<Vec<CodeableConcept>>,
    /// Person(s), Location, HealthcareService, or Device
    #[fhir(name="actor", min="0", max="1", summary="true", modifier="false")]
    pub actor: Option<Reference>,
    /// accepted | declined | tentative | needs-action | entered-in-error
    #[fhir(name="participantStatus", min="1", max="1", summary="true", modifier="true")]
    pub participant_status: Option<CodeDt>,
    /// Additional comments
    #[fhir(name="comment", min="0", max="1", summary="false", modifier="false")]
    pub comment: Option<MarkdownDt>,
    /// This response is for all occurrences in a recurring request
    #[fhir(name="recurring", min="0", max="1", summary="false", modifier="false")]
    pub recurring: Option<BooleanDt>,
    /// Original date within a recurring request
    #[fhir(name="occurrenceDate", min="0", max="1", summary="false", modifier="false")]
    pub occurrence_date: Option<DateDt>,
    /// The recurrence ID of the specific recurring request
    #[fhir(name="recurrenceId", min="0", max="1", summary="false", modifier="false")]
    pub recurrence_id: Option<PositiveIntDt>,
}


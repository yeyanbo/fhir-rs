use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Appointment {
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
    /// External Ids for this item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | pending | booked | arrived | fulfilled | cancelled | noshow | entered-in-error | checked-in | waitlist
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// The coded reason for the appointment being cancelled
    #[fhir(name="cancellationReason", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub cancellation_reason: Option<CodeableConcept>,
    /// Classification when becoming an encounter
    #[fhir(name="class", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub class: Option<Vec<CodeableConcept>>,
    /// A broad categorization of the service that is to be performed during this appointment
    #[fhir(name="serviceCategory", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub service_category: Option<Vec<CodeableConcept>>,
    /// The specific service that is to be performed during this appointment
    #[fhir(name="serviceType", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub service_type: Option<Vec<CodeableReference>>,
    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    #[fhir(name="specialty", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The style of appointment or patient that has been booked in the slot (not service type)
    #[fhir(name="appointmentType", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub appointment_type: Option<CodeableConcept>,
    /// Reason this appointment is scheduled
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// Used to make informed decisions if needing to re-prioritize
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub priority: Option<CodeableConcept>,
    /// Shown on a subject line in a meeting request, or appointment list
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<StringDt>,
    /// Appointment replaced by this Appointment
    #[fhir(name="replaces", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub replaces: Option<Vec<Reference>>,
    /// Connection details of a virtual service (e.g. conference call)
    #[fhir(name="virtualService", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,
    /// Additional information to support the appointment
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supporting_information: Option<Vec<Reference>>,
    /// The previous appointment in a series
    #[fhir(name="previousAppointment", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub previous_appointment: Option<Reference>,
    /// The originating appointment in a recurring set of appointments
    #[fhir(name="originatingAppointment", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub originating_appointment: Option<Reference>,
    /// When appointment is to take place
    #[fhir(name="start", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub start: Option<InstantDt>,
    /// When appointment is to conclude
    #[fhir(name="end", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub end: Option<InstantDt>,
    /// Can be less than start/end (e.g. estimate)
    #[fhir(name="minutesDuration", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub minutes_duration: Option<PositiveIntDt>,
    /// Potential date/time interval(s) requested to allocate the appointment within
    #[fhir(name="requestedPeriod", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub requested_period: Option<Vec<Period>>,
    /// The slots that this appointment is filling
    #[fhir(name="slot", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub slot: Option<Vec<Reference>>,
    /// The set of accounts that may be used for billing for this Appointment
    #[fhir(name="account", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub account: Option<Vec<Reference>>,
    /// The date that this appointment was initially created
    #[fhir(name="created", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub created: Option<DateTimeDt>,
    /// When the appointment was cancelled
    #[fhir(name="cancellationDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub cancellation_date: Option<DateTimeDt>,
    /// Additional comments
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub note: Option<Vec<Annotation>>,
    /// Detailed information and instructions for the patient
    #[fhir(name="patientInstruction", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub patient_instruction: Option<Vec<CodeableReference>>,
    /// The request this appointment is allocated to assess
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub based_on: Option<Vec<Reference>>,
    /// The patient or group associated with the appointment
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// Participants involved in appointment
    #[fhir(name="participant", min="1", max="*", summary=false, modifier=false, choice=false)]
    pub participant: Option<Vec<AppointmentParticipantBackboneElement>>,
    /// The sequence number in the recurrence
    #[fhir(name="recurrenceId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub recurrence_id: Option<PositiveIntDt>,
    /// Indicates that this appointment varies from a recurrence pattern
    #[fhir(name="occurrenceChanged", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub occurrence_changed: Option<BooleanDt>,
    /// Details of the recurrence pattern/template used to generate occurrences
    #[fhir(name="recurrenceTemplate", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub recurrence_template: Option<Vec<AppointmentRecurrenceTemplateBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AppointmentParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Role of participant in the appointment
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Participation period of the actor
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub period: Option<Period>,
    /// The individual, device, location, or service participating in the appointment
    #[fhir(name="actor", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub actor: Option<Reference>,
    /// The participant is required to attend (optional when false)
    #[fhir(name="required", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub required: Option<BooleanDt>,
    /// accepted | declined | tentative | needs-action
    #[fhir(name="status", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub status: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AppointmentRecurrenceTemplateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The timezone of the occurrences
    #[fhir(name="timezone", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub timezone: Option<CodeableConcept>,
    /// The frequency of the recurrence
    #[fhir(name="recurrenceType", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub recurrence_type: Option<CodeableConcept>,
    /// The date when the recurrence should end
    #[fhir(name="lastOccurrenceDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub last_occurrence_date: Option<DateDt>,
    /// The number of planned occurrences
    #[fhir(name="occurrenceCount", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub occurrence_count: Option<PositiveIntDt>,
    /// Specific dates for a recurring set of appointments (no template)
    #[fhir(name="occurrenceDate", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub occurrence_date: Option<Vec<DateDt>>,
    /// Information about weekly recurring appointments
    #[fhir(name="weeklyTemplate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub weekly_template: Option<AppointmentRecurrenceTemplateWeeklyTemplateBackboneElement>,
    /// Information about monthly recurring appointments
    #[fhir(name="monthlyTemplate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub monthly_template: Option<AppointmentRecurrenceTemplateMonthlyTemplateBackboneElement>,
    /// Information about yearly recurring appointments
    #[fhir(name="yearlyTemplate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub yearly_template: Option<AppointmentRecurrenceTemplateYearlyTemplateBackboneElement>,
    /// Any dates that should be excluded from the series
    #[fhir(name="excludingDate", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub excluding_date: Option<Vec<DateDt>>,
    /// Any recurrence IDs that should be excluded from the recurrence
    #[fhir(name="excludingRecurrenceId", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub excluding_recurrence_id: Option<Vec<PositiveIntDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AppointmentRecurrenceTemplateMonthlyTemplateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Recurs on a specific day of the month
    #[fhir(name="dayOfMonth", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub day_of_month: Option<PositiveIntDt>,
    /// Indicates which week of the month the appointment should occur
    #[fhir(name="nthWeekOfMonth", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub nth_week_of_month: Option<Coding>,
    /// Indicates which day of the week the appointment should occur
    #[fhir(name="dayOfWeek", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub day_of_week: Option<Coding>,
    /// Recurs every nth month
    #[fhir(name="monthInterval", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub month_interval: Option<PositiveIntDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AppointmentRecurrenceTemplateWeeklyTemplateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Recurs on Mondays
    #[fhir(name="monday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub monday: Option<BooleanDt>,
    /// Recurs on Tuesday
    #[fhir(name="tuesday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub tuesday: Option<BooleanDt>,
    /// Recurs on Wednesday
    #[fhir(name="wednesday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub wednesday: Option<BooleanDt>,
    /// Recurs on Thursday
    #[fhir(name="thursday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub thursday: Option<BooleanDt>,
    /// Recurs on Friday
    #[fhir(name="friday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub friday: Option<BooleanDt>,
    /// Recurs on Saturday
    #[fhir(name="saturday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub saturday: Option<BooleanDt>,
    /// Recurs on Sunday
    #[fhir(name="sunday", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub sunday: Option<BooleanDt>,
    /// Recurs every nth week
    #[fhir(name="weekInterval", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub week_interval: Option<PositiveIntDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AppointmentRecurrenceTemplateYearlyTemplateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Recurs every nth year
    #[fhir(name="yearInterval", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub year_interval: Option<PositiveIntDt>,
}


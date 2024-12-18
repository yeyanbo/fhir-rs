use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Slot {
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
    /// External Ids for this item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// A broad categorization of the service that is to be performed during this appointment
    #[fhir(name="serviceCategory", min="0", max="*", summary=true, modifier=false, choice="")]
    pub service_category: Option<Vec<CodeableConcept>>,
    /// The type of appointments that can be booked into this slot (ideally this would be an identifiable service - which is at a location, rather than the location itself). If provided then this overrides the value provided on the Schedule resource
    #[fhir(name="serviceType", min="0", max="*", summary=true, modifier=false, choice="")]
    pub service_type: Option<Vec<CodeableReference>>,
    /// The specialty of a practitioner that would be required to perform the service requested in this appointment
    #[fhir(name="specialty", min="0", max="*", summary=true, modifier=false, choice="")]
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The style of appointment or patient that may be booked in the slot (not service type)
    #[fhir(name="appointmentType", min="0", max="*", summary=true, modifier=false, choice="")]
    pub appointment_type: Option<Vec<CodeableConcept>>,
    /// The schedule resource that this slot defines an interval of status information
    #[fhir(name="schedule", min="1", max="1", summary=true, modifier=false, choice="")]
    pub schedule: Option<Reference>,
    /// busy | free | busy-unavailable | busy-tentative | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=false, choice="")]
    pub status: Option<CodeDt>,
    /// Date/Time that the slot is to begin
    #[fhir(name="start", min="1", max="1", summary=true, modifier=false, choice="")]
    pub start: Option<InstantDt>,
    /// Date/Time that the slot is to conclude
    #[fhir(name="end", min="1", max="1", summary=true, modifier=false, choice="")]
    pub end: Option<InstantDt>,
    /// This slot has already been overbooked, appointments are unlikely to be accepted for this time
    #[fhir(name="overbooked", min="0", max="1", summary=false, modifier=false, choice="")]
    pub overbooked: Option<BooleanDt>,
    /// Comments on the slot to describe any extended information. Such as custom constraints on the slot
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub comment: Option<StringDt>,
}


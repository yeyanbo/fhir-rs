use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct EncounterHistory {
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
    /// The Encounter associated with this set of historic values
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub encounter: Option<Reference>,
    /// Identifier(s) by which this encounter is known
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// planned | in-progress | on-hold | discharged | completed | cancelled | discontinued | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Classification of patient encounter
    #[fhir(name="class", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub class: Option<CodeableConcept>,
    /// Specific type of encounter
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Specific type of service
    #[fhir(name="serviceType", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub service_type: Option<Vec<CodeableReference>>,
    /// The patient or group related to this encounter
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// The current status of the subject in relation to the Encounter
    #[fhir(name="subjectStatus", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subject_status: Option<CodeableConcept>,
    /// The actual start and end time associated with this set of values associated with the encounter
    #[fhir(name="actualPeriod", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub actual_period: Option<Period>,
    /// The planned start date/time (or admission date) of the encounter
    #[fhir(name="plannedStartDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub planned_start_date: Option<DateTimeDt>,
    /// The planned end date/time (or discharge date) of the encounter
    #[fhir(name="plannedEndDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub planned_end_date: Option<DateTimeDt>,
    /// Actual quantity of time the encounter lasted (less time absent)
    #[fhir(name="length", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub length: Option<Duration>,
    /// Location of the patient at this point in the encounter
    #[fhir(name="location", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub location: Option<Vec<EncounterHistoryLocationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EncounterHistoryLocationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location the encounter takes place
    #[fhir(name="location", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub location: Option<Reference>,
    /// The physical type of the location (usually the level in the location hierarchy - bed, room, ward, virtual etc.)
    #[fhir(name="form", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub form: Option<CodeableConcept>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct AllergyIntolerance {
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
    /// External ids for this item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | resolved
    #[fhir(name="clinicalStatus", min="0", max="1", summary=true, modifier=true)]
    pub clinical_status: Option<CodeableConcept>,
    /// unconfirmed | presumed | confirmed | refuted | entered-in-error
    #[fhir(name="verificationStatus", min="0", max="1", summary=true, modifier=true)]
    pub verification_status: Option<CodeableConcept>,
    /// allergy | intolerance - Underlying mechanism (if known)
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// food | medication | environment | biologic
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeDt>>,
    /// low | high | unable-to-assess
    #[fhir(name="criticality", min="0", max="1", summary=true, modifier=false, choice="")]
    pub criticality: Option<CodeDt>,
    /// Code that identifies the allergy or intolerance
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Who the allergy or intolerance is for
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false, choice="")]
    pub patient: Option<Reference>,
    /// Encounter when the allergy or intolerance was asserted
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// When allergy or intolerance was identified
    #[fhir(name="onset", min="0", max="1", summary=false, modifier=false, choice="")]
    pub onset: Option<StringDt>,
    /// Date allergy or intolerance was first recorded
    #[fhir(name="recordedDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub recorded_date: Option<DateTimeDt>,
    /// Who or what participated in the activities related to the allergy or intolerance and how they were involved
    #[fhir(name="participant", min="0", max="*", summary=true, modifier=false, choice="")]
    pub participant: Option<Vec<AllergyIntoleranceParticipantBackboneElement>>,
    /// Date(/time) of last known occurrence of a reaction
    #[fhir(name="lastOccurrence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub last_occurrence: Option<DateTimeDt>,
    /// Additional text not captured in other fields
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Adverse Reaction Events linked to exposure to substance
    #[fhir(name="reaction", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reaction: Option<Vec<AllergyIntoleranceReactionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AllergyIntoleranceReactionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specific substance or pharmaceutical product considered to be responsible for event
    #[fhir(name="substance", min="0", max="1", summary=false, modifier=false, choice="")]
    pub substance: Option<CodeableConcept>,
    /// Clinical symptoms/signs associated with the Event
    #[fhir(name="manifestation", min="1", max="*", summary=false, modifier=false, choice="")]
    pub manifestation: Option<Vec<CodeableReference>>,
    /// Description of the event as a whole
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// Date(/time) when manifestations showed
    #[fhir(name="onset", min="0", max="1", summary=false, modifier=false, choice="")]
    pub onset: Option<DateTimeDt>,
    /// mild | moderate | severe (of event as a whole)
    #[fhir(name="severity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub severity: Option<CodeDt>,
    /// How the subject was exposed to the substance
    #[fhir(name="exposureRoute", min="0", max="1", summary=false, modifier=false, choice="")]
    pub exposure_route: Option<CodeableConcept>,
    /// Text about event not captured in other fields
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AllergyIntoleranceParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of involvement
    #[fhir(name="function", min="0", max="1", summary=true, modifier=false, choice="")]
    pub function: Option<CodeableConcept>,
    /// Who or what participated in the activities related to the allergy or intolerance
    #[fhir(name="actor", min="1", max="1", summary=true, modifier=false, choice="")]
    pub actor: Option<Reference>,
}


use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct ResearchSubject {
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
    /// Business Identifier for research subject in a study
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Subject status
    #[fhir(name="progress", min="0", max="*", summary="false", modifier="false")]
    pub progress: Option<Vec<ResearchSubjectProgressBackboneElement>>,
    /// Start and end of participation
    #[fhir(name="period", min="0", max="1", summary="true", modifier="false")]
    pub period: Option<Period>,
    /// Study subject is part of
    #[fhir(name="study", min="1", max="1", summary="true", modifier="false")]
    pub study: Option<Reference>,
    /// Who or what is part of study
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// What path should be followed
    #[fhir(name="assignedComparisonGroup", min="0", max="1", summary="false", modifier="false")]
    pub assigned_comparison_group: Option<IdDt>,
    /// What path was followed
    #[fhir(name="actualComparisonGroup", min="0", max="1", summary="false", modifier="false")]
    pub actual_comparison_group: Option<IdDt>,
    /// Agreement to participate in study
    #[fhir(name="consent", min="0", max="*", summary="false", modifier="false")]
    pub consent: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchSubjectProgressBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// state | milestone
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// candidate | eligible | follow-up | ineligible | not-registered | off-study | on-study | on-study-intervention | on-study-observation | pending-on-study | potential-candidate | screening | withdrawn
    #[fhir(name="subjectState", min="0", max="1", summary="false", modifier="false")]
    pub subject_state: Option<CodeableConcept>,
    /// SignedUp | Screened | Randomized
    #[fhir(name="milestone", min="0", max="1", summary="false", modifier="false")]
    pub milestone: Option<CodeableConcept>,
    /// State change reason
    #[fhir(name="reason", min="0", max="1", summary="false", modifier="false")]
    pub reason: Option<CodeableConcept>,
    /// State change date
    #[fhir(name="startDate", min="0", max="1", summary="false", modifier="false")]
    pub start_date: Option<DateTimeDt>,
    /// State change date
    #[fhir(name="endDate", min="0", max="1", summary="false", modifier="false")]
    pub end_date: Option<DateTimeDt>,
}

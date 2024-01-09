use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct Condition {
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
    /// External Ids for this condition
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | recurrence | relapse | inactive | remission | resolved | unknown
    #[fhir(name="clinicalStatus", min="1", max="1", summary="true", modifier="true")]
    pub clinical_status: Option<CodeableConcept>,
    /// unconfirmed | provisional | differential | confirmed | refuted | entered-in-error
    #[fhir(name="verificationStatus", min="0", max="1", summary="true", modifier="true")]
    pub verification_status: Option<CodeableConcept>,
    /// problem-list-item | encounter-diagnosis
    #[fhir(name="category", min="0", max="*", summary="false", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Subjective severity of condition
    #[fhir(name="severity", min="0", max="1", summary="false", modifier="false")]
    pub severity: Option<CodeableConcept>,
    /// Identification of the condition, problem or diagnosis
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Anatomical location, if relevant
    #[fhir(name="bodySite", min="0", max="*", summary="true", modifier="false")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// Who has the condition?
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// The Encounter during which this Condition was created
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// Estimated or actual date,  date-time, or age
    #[fhir(name="onset", min="0", max="1", summary="true", modifier="false")]
    pub onset: Option<StringDt>,
    /// When in resolution/remission
    #[fhir(name="abatement", min="0", max="1", summary="false", modifier="false")]
    pub abatement: Option<StringDt>,
    /// Date condition was first recorded
    #[fhir(name="recordedDate", min="0", max="1", summary="true", modifier="false")]
    pub recorded_date: Option<DateTimeDt>,
    /// Who or what participated in the activities related to the condition and how they were involved
    #[fhir(name="participant", min="0", max="*", summary="true", modifier="false")]
    pub participant: Option<Vec<ConditionParticipantBackboneElement>>,
    /// Stage/grade, usually assessed formally
    #[fhir(name="stage", min="0", max="*", summary="false", modifier="false")]
    pub stage: Option<Vec<ConditionStageBackboneElement>>,
    /// Supporting evidence for the verification status
    #[fhir(name="evidence", min="0", max="*", summary="true", modifier="false")]
    pub evidence: Option<Vec<CodeableReference>>,
    /// Additional information about the Condition
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionStageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Simple summary (disease specific)
    #[fhir(name="summary", min="0", max="1", summary="false", modifier="false")]
    pub summary: Option<CodeableConcept>,
    /// Formal record of assessment
    #[fhir(name="assessment", min="0", max="*", summary="false", modifier="false")]
    pub assessment: Option<Vec<Reference>>,
    /// Kind of staging
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of involvement
    #[fhir(name="function", min="0", max="1", summary="true", modifier="false")]
    pub function: Option<CodeableConcept>,
    /// Who or what participated in the activities related to the condition
    #[fhir(name="actor", min="1", max="1", summary="true", modifier="false")]
    pub actor: Option<Reference>,
}


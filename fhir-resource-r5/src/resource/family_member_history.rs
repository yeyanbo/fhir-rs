use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct FamilyMemberHistory {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// External Id(s) for this record
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=true, modifier=false)]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=true, modifier=false)]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// partial | completed | entered-in-error | health-unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// subject-unknown | withheld | unable-to-obtain | deferred
    #[fhir(name="dataAbsentReason", min="0", max="1", summary=true, modifier=false)]
    pub data_absent_reason: Option<CodeableConcept>,
    /// Patient history is about
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false)]
    pub patient: Option<Reference>,
    /// When history was recorded or last updated
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false)]
    pub date: Option<DateTimeDt>,
    /// Who or what participated in the activities related to the family member history and how they were involved
    #[fhir(name="participant", min="0", max="*", summary=true, modifier=false)]
    pub participant: Option<Vec<FamilyMemberHistoryParticipantBackboneElement>>,
    /// The family member described
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Relationship to the subject
    #[fhir(name="relationship", min="1", max="1", summary=true, modifier=false)]
    pub relationship: Option<CodeableConcept>,
    /// male | female | other | unknown
    #[fhir(name="sex", min="0", max="1", summary=true, modifier=false)]
    pub sex: Option<CodeableConcept>,
    /// (approximate) date of birth
    #[fhir(name="born", min="0", max="1", summary=false, modifier=false)]
    pub born: Option<StringDt>,
    /// (approximate) age
    #[fhir(name="age", min="0", max="1", summary=true, modifier=false)]
    pub age: Option<StringDt>,
    /// Age is estimated?
    #[fhir(name="estimatedAge", min="0", max="1", summary=true, modifier=false)]
    pub estimated_age: Option<BooleanDt>,
    /// Dead? How old/when?
    #[fhir(name="deceased", min="0", max="1", summary=true, modifier=false)]
    pub deceased: Option<StringDt>,
    /// Why was family member history performed?
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// General note about related person
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Condition that the related person had
    #[fhir(name="condition", min="0", max="*", summary=false, modifier=false)]
    pub condition: Option<Vec<FamilyMemberHistoryConditionBackboneElement>>,
    /// Procedures that the related person had
    #[fhir(name="procedure", min="0", max="*", summary=false, modifier=false)]
    pub procedure: Option<Vec<FamilyMemberHistoryProcedureBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct FamilyMemberHistoryParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of involvement
    #[fhir(name="function", min="0", max="1", summary=true, modifier=false)]
    pub function: Option<CodeableConcept>,
    /// Who or what participated in the activities related to the family member history
    #[fhir(name="actor", min="1", max="1", summary=true, modifier=false)]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct FamilyMemberHistoryConditionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Condition suffered by relation
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// deceased | permanent disability | etc
    #[fhir(name="outcome", min="0", max="1", summary=false, modifier=false)]
    pub outcome: Option<CodeableConcept>,
    /// Whether the condition contributed to the cause of death
    #[fhir(name="contributedToDeath", min="0", max="1", summary=false, modifier=false)]
    pub contributed_to_death: Option<BooleanDt>,
    /// When condition first manifested
    #[fhir(name="onset", min="0", max="1", summary=false, modifier=false)]
    pub onset: Option<StringDt>,
    /// Extra information about condition
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct FamilyMemberHistoryProcedureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Procedures performed on the related person
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// What happened following the procedure
    #[fhir(name="outcome", min="0", max="1", summary=false, modifier=false)]
    pub outcome: Option<CodeableConcept>,
    /// Whether the procedure contributed to the cause of death
    #[fhir(name="contributedToDeath", min="0", max="1", summary=false, modifier=false)]
    pub contributed_to_death: Option<BooleanDt>,
    /// When the procedure was performed
    #[fhir(name="performed", min="0", max="1", summary=false, modifier=false)]
    pub performed: Option<DateTimeDt>,
    /// Extra information about the procedure
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
}


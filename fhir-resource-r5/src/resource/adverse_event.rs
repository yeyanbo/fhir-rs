use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct AdverseEvent {
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
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business identifier for the event
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// in-progress | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// actual | potential
    #[fhir(name="actuality", min="1", max="1", summary="true", modifier="true")]
    pub actuality: Option<CodeDt>,
    /// wrong-patient | procedure-mishap | medication-mishap | device | unsafe-physical-environment | hospital-aquired-infection | wrong-body-site
    #[fhir(name="category", min="0", max="*", summary="true", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Event or incident that occurred or was averted
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Subject impacted by event
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// The Encounter associated with the start of the AdverseEvent
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// When the event occurred
    #[fhir(name="occurrence", min="0", max="1", summary="true", modifier="false")]
    pub occurrence: Option<Timing>,
    /// When the event was detected
    #[fhir(name="detected", min="0", max="1", summary="true", modifier="false")]
    pub detected: Option<DateTimeDt>,
    /// When the event was recorded
    #[fhir(name="recordedDate", min="0", max="1", summary="true", modifier="false")]
    pub recorded_date: Option<DateTimeDt>,
    /// Effect on the subject due to this event
    #[fhir(name="resultingEffect", min="0", max="*", summary="true", modifier="false")]
    pub resulting_effect: Option<Vec<Reference>>,
    /// Location where adverse event occurred
    #[fhir(name="location", min="0", max="1", summary="true", modifier="false")]
    pub location: Option<Reference>,
    /// Seriousness or gravity of the event
    #[fhir(name="seriousness", min="0", max="1", summary="true", modifier="false")]
    pub seriousness: Option<CodeableConcept>,
    /// Type of outcome from the adverse event
    #[fhir(name="outcome", min="0", max="*", summary="true", modifier="false")]
    pub outcome: Option<Vec<CodeableConcept>>,
    /// Who recorded the adverse event
    #[fhir(name="recorder", min="0", max="1", summary="true", modifier="false")]
    pub recorder: Option<Reference>,
    /// Who was involved in the adverse event or the potential adverse event and what they did
    #[fhir(name="participant", min="0", max="*", summary="true", modifier="false")]
    pub participant: Option<Vec<AdverseEventParticipantBackboneElement>>,
    /// Research study that the subject is enrolled in
    #[fhir(name="study", min="0", max="*", summary="true", modifier="false")]
    pub study: Option<Vec<Reference>>,
    /// Considered likely or probable or anticipated in the research study
    #[fhir(name="expectedInResearchStudy", min="0", max="1", summary="false", modifier="false")]
    pub expected_in_research_study: Option<BooleanDt>,
    /// The suspected agent causing the adverse event
    #[fhir(name="suspectEntity", min="0", max="*", summary="true", modifier="false")]
    pub suspect_entity: Option<Vec<AdverseEventSuspectEntityBackboneElement>>,
    /// Contributing factors suspected to have increased the probability or severity of the adverse event
    #[fhir(name="contributingFactor", min="0", max="*", summary="true", modifier="false")]
    pub contributing_factor: Option<Vec<AdverseEventContributingFactorBackboneElement>>,
    /// Preventive actions that contributed to avoiding the adverse event
    #[fhir(name="preventiveAction", min="0", max="*", summary="true", modifier="false")]
    pub preventive_action: Option<Vec<AdverseEventPreventiveActionBackboneElement>>,
    /// Ameliorating actions taken after the adverse event occured in order to reduce the extent of harm
    #[fhir(name="mitigatingAction", min="0", max="*", summary="true", modifier="false")]
    pub mitigating_action: Option<Vec<AdverseEventMitigatingActionBackboneElement>>,
    /// Supporting information relevant to the event
    #[fhir(name="supportingInfo", min="0", max="*", summary="true", modifier="false")]
    pub supporting_info: Option<Vec<AdverseEventSupportingInfoBackboneElement>>,
    /// Comment on adverse event
    #[fhir(name="note", min="0", max="*", summary="true", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventMitigatingActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Ameliorating action taken after the adverse event occured in order to reduce the extent of harm
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventParticipantBackboneElement {
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
    /// Who was involved in the adverse event or the potential adverse event
    #[fhir(name="actor", min="1", max="1", summary="true", modifier="false")]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventSuspectEntityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Refers to the specific entity that caused the adverse event
    #[fhir(name="instance", min="1", max="1", summary="true", modifier="false")]
    pub instance: Option<Reference>,
    /// Information on the possible cause of the event
    #[fhir(name="causality", min="0", max="1", summary="true", modifier="false")]
    pub causality: Option<AdverseEventSuspectEntityCausalityBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventSuspectEntityCausalityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Method of evaluating the relatedness of the suspected entity to the event
    #[fhir(name="assessmentMethod", min="0", max="1", summary="true", modifier="false")]
    pub assessment_method: Option<CodeableConcept>,
    /// Result of the assessment regarding the relatedness of the suspected entity to the event
    #[fhir(name="entityRelatedness", min="0", max="1", summary="true", modifier="false")]
    pub entity_relatedness: Option<CodeableConcept>,
    /// Author of the information on the possible cause of the event
    #[fhir(name="author", min="0", max="1", summary="true", modifier="false")]
    pub author: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventContributingFactorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item suspected to have increased the probability or severity of the adverse event
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventSupportingInfoBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Subject medical history or document relevant to this adverse event
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdverseEventPreventiveActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Action that contributed to avoiding the adverse event
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableConcept>,
}


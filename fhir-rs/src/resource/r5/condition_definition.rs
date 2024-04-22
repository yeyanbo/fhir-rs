use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ConditionDefinition {
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
    /// Canonical identifier for this condition definition, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the condition definition
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the condition definition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this condition definition (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this condition definition (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// Subordinate title of the event definition
    #[fhir(name="subtitle", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subtitle: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice="")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice="")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice="")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the condition definition
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for condition definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Identification of the condition, problem or diagnosis
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Subjective severity of condition
    #[fhir(name="severity", min="0", max="1", summary=true, modifier=false, choice="")]
    pub severity: Option<CodeableConcept>,
    /// Anatomical location, if relevant
    #[fhir(name="bodySite", min="0", max="1", summary=true, modifier=false, choice="")]
    pub body_site: Option<CodeableConcept>,
    /// Stage/grade, usually assessed formally
    #[fhir(name="stage", min="0", max="1", summary=true, modifier=false, choice="")]
    pub stage: Option<CodeableConcept>,
    /// Whether Severity is appropriate
    #[fhir(name="hasSeverity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub has_severity: Option<BooleanDt>,
    /// Whether bodySite is appropriate
    #[fhir(name="hasBodySite", min="0", max="1", summary=false, modifier=false, choice="")]
    pub has_body_site: Option<BooleanDt>,
    /// Whether stage is appropriate
    #[fhir(name="hasStage", min="0", max="1", summary=false, modifier=false, choice="")]
    pub has_stage: Option<BooleanDt>,
    /// Formal Definition for the condition
    #[fhir(name="definition", min="0", max="*", summary=false, modifier=false, choice="")]
    pub definition: Option<Vec<UriDt>>,
    /// Observations particularly relevant to this condition
    #[fhir(name="observation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub observation: Option<Vec<ConditionDefinitionObservationBackboneElement>>,
    /// Medications particularly relevant for this condition
    #[fhir(name="medication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub medication: Option<Vec<ConditionDefinitionMedicationBackboneElement>>,
    /// Observation that suggets this condition
    #[fhir(name="precondition", min="0", max="*", summary=false, modifier=false, choice="")]
    pub precondition: Option<Vec<ConditionDefinitionPreconditionBackboneElement>>,
    /// Appropriate team for this condition
    #[fhir(name="team", min="0", max="*", summary=false, modifier=false, choice="")]
    pub team: Option<Vec<Reference>>,
    /// Questionnaire for this condition
    #[fhir(name="questionnaire", min="0", max="*", summary=false, modifier=false, choice="")]
    pub questionnaire: Option<Vec<ConditionDefinitionQuestionnaireBackboneElement>>,
    /// Plan that is appropriate
    #[fhir(name="plan", min="0", max="*", summary=false, modifier=false, choice="")]
    pub plan: Option<Vec<ConditionDefinitionPlanBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionDefinitionQuestionnaireBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// preadmit | diff-diagnosis | outcome
    #[fhir(name="purpose", min="1", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<CodeDt>,
    /// Specific Questionnaire
    #[fhir(name="reference", min="1", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionDefinitionMedicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Category that is relevant
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Code for relevant Medication
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionDefinitionObservationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Category that is relevant
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Code for relevant Observation
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionDefinitionPreconditionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// sensitive | specific
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// Code for relevant Observation
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Value of Observation
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConditionDefinitionPlanBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Use for the plan
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
    /// The actual plan
    #[fhir(name="reference", min="1", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<Reference>,
}


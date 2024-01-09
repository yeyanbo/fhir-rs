use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct RiskAssessment {
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
    /// Unique identifier for the assessment
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Request fulfilled by this assessment
    #[fhir(name="basedOn", min="0", max="1", summary="false", modifier="false")]
    pub based_on: Option<Reference>,
    /// Part of this occurrence
    #[fhir(name="parent", min="0", max="1", summary="false", modifier="false")]
    pub parent: Option<Reference>,
    /// registered | preliminary | final | amended +
    #[fhir(name="status", min="1", max="1", summary="true", modifier="false")]
    pub status: Option<CodeDt>,
    /// Evaluation mechanism
    #[fhir(name="method", min="0", max="1", summary="true", modifier="false")]
    pub method: Option<CodeableConcept>,
    /// Type of assessment
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Who/what does assessment apply to?
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// Where was assessment performed?
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// When was assessment made?
    #[fhir(name="occurrence", min="0", max="1", summary="true", modifier="false")]
    pub occurrence: Option<Period>,
    /// Condition assessed
    #[fhir(name="condition", min="0", max="1", summary="true", modifier="false")]
    pub condition: Option<Reference>,
    /// Who did assessment?
    #[fhir(name="performer", min="0", max="1", summary="true", modifier="false")]
    pub performer: Option<Reference>,
    /// Why the assessment was necessary?
    #[fhir(name="reason", min="0", max="*", summary="false", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Information used in assessment
    #[fhir(name="basis", min="0", max="*", summary="false", modifier="false")]
    pub basis: Option<Vec<Reference>>,
    /// Outcome predicted
    #[fhir(name="prediction", min="0", max="*", summary="false", modifier="false")]
    pub prediction: Option<Vec<RiskAssessmentPredictionBackboneElement>>,
    /// How to reduce risk
    #[fhir(name="mitigation", min="0", max="1", summary="false", modifier="false")]
    pub mitigation: Option<StringDt>,
    /// Comments on the risk assessment
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RiskAssessmentPredictionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Possible outcome for the subject
    #[fhir(name="outcome", min="0", max="1", summary="false", modifier="false")]
    pub outcome: Option<CodeableConcept>,
    /// Likelihood of specified outcome
    #[fhir(name="probability", min="0", max="1", summary="false", modifier="false")]
    pub probability: Option<Range>,
    /// Likelihood of specified outcome as a qualitative value
    #[fhir(name="qualitativeRisk", min="0", max="1", summary="false", modifier="false")]
    pub qualitative_risk: Option<CodeableConcept>,
    /// Relative likelihood
    #[fhir(name="relativeRisk", min="0", max="1", summary="false", modifier="false")]
    pub relative_risk: Option<DecimalDt>,
    /// Timeframe or age range
    #[fhir(name="when", min="0", max="1", summary="false", modifier="false")]
    pub when: Option<Range>,
    /// Explanation of prediction
    #[fhir(name="rationale", min="0", max="1", summary="false", modifier="false")]
    pub rationale: Option<StringDt>,
}


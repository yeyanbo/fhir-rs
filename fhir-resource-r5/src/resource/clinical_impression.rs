use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct ClinicalImpression {
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
    /// Business identifier
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary="false", modifier="false")]
    pub status_reason: Option<CodeableConcept>,
    /// Why/how the assessment was performed
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<StringDt>,
    /// Patient or group assessed
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// The Encounter during which this ClinicalImpression was created
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// Time of assessment
    #[fhir(name="effective", min="0", max="1", summary="true", modifier="false")]
    pub effective: Option<Period>,
    /// When the assessment was documented
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// The clinician performing the assessment
    #[fhir(name="performer", min="0", max="1", summary="true", modifier="false")]
    pub performer: Option<Reference>,
    /// Reference to last assessment
    #[fhir(name="previous", min="0", max="1", summary="false", modifier="false")]
    pub previous: Option<Reference>,
    /// Relevant impressions of patient state
    #[fhir(name="problem", min="0", max="*", summary="true", modifier="false")]
    pub problem: Option<Vec<Reference>>,
    /// Change in the status/pattern of a subject's condition since previously assessed, such as worsening, improving, or no change
    #[fhir(name="changePattern", min="0", max="1", summary="false", modifier="false")]
    pub change_pattern: Option<CodeableConcept>,
    /// Clinical Protocol followed
    #[fhir(name="protocol", min="0", max="*", summary="false", modifier="false")]
    pub protocol: Option<Vec<UriDt>>,
    /// Summary of the assessment
    #[fhir(name="summary", min="0", max="1", summary="false", modifier="false")]
    pub summary: Option<StringDt>,
    /// Possible or likely findings and diagnoses
    #[fhir(name="finding", min="0", max="*", summary="false", modifier="false")]
    pub finding: Option<Vec<ClinicalImpressionFindingBackboneElement>>,
    /// Estimate of likely outcome
    #[fhir(name="prognosisCodeableConcept", min="0", max="*", summary="false", modifier="false")]
    pub prognosis_codeable_concept: Option<Vec<CodeableConcept>>,
    /// RiskAssessment expressing likely outcome
    #[fhir(name="prognosisReference", min="0", max="*", summary="false", modifier="false")]
    pub prognosis_reference: Option<Vec<Reference>>,
    /// Information supporting the clinical impression
    #[fhir(name="supportingInfo", min="0", max="*", summary="false", modifier="false")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Comments made about the ClinicalImpression
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalImpressionFindingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What was found
    #[fhir(name="item", min="0", max="1", summary="false", modifier="false")]
    pub item: Option<CodeableReference>,
    /// Which investigations support finding
    #[fhir(name="basis", min="0", max="1", summary="false", modifier="false")]
    pub basis: Option<StringDt>,
}


use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ImmunizationEvaluation {
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
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// completed | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Who this evaluation is for
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
    /// Date evaluation was performed
    #[fhir(name="date", min="0", max="1", summary="false", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Who is responsible for publishing the recommendations
    #[fhir(name="authority", min="0", max="1", summary="false", modifier="false")]
    pub authority: Option<Reference>,
    /// The vaccine preventable disease schedule being evaluated
    #[fhir(name="targetDisease", min="1", max="1", summary="true", modifier="false")]
    pub target_disease: Option<CodeableConcept>,
    /// Immunization being evaluated
    #[fhir(name="immunizationEvent", min="1", max="1", summary="true", modifier="false")]
    pub immunization_event: Option<Reference>,
    /// Status of the dose relative to published recommendations
    #[fhir(name="doseStatus", min="1", max="1", summary="true", modifier="false")]
    pub dose_status: Option<CodeableConcept>,
    /// Reason why the doese is considered valid, invalid or some other status
    #[fhir(name="doseStatusReason", min="0", max="*", summary="false", modifier="false")]
    pub dose_status_reason: Option<Vec<CodeableConcept>>,
    /// Evaluation notes
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Name of vaccine series
    #[fhir(name="series", min="0", max="1", summary="false", modifier="false")]
    pub series: Option<StringDt>,
    /// Dose number within series
    #[fhir(name="doseNumber", min="0", max="1", summary="false", modifier="false")]
    pub dose_number: Option<StringDt>,
    /// Recommended number of doses for immunity
    #[fhir(name="seriesDoses", min="0", max="1", summary="false", modifier="false")]
    pub series_doses: Option<StringDt>,
}


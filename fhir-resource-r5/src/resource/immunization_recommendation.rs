use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct ImmunizationRecommendation {
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
    /// Business identifier
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Who this profile is for
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
    /// Date recommendation(s) created
    #[fhir(name="date", min="1", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Who is responsible for protocol
    #[fhir(name="authority", min="0", max="1", summary="false", modifier="false")]
    pub authority: Option<Reference>,
    /// Vaccine administration recommendations
    #[fhir(name="recommendation", min="1", max="*", summary="true", modifier="false")]
    pub recommendation: Option<Vec<ImmunizationRecommendationRecommendationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImmunizationRecommendationRecommendationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Vaccine  or vaccine group recommendation applies to
    #[fhir(name="vaccineCode", min="0", max="*", summary="true", modifier="false")]
    pub vaccine_code: Option<Vec<CodeableConcept>>,
    /// Disease to be immunized against
    #[fhir(name="targetDisease", min="0", max="*", summary="true", modifier="false")]
    pub target_disease: Option<Vec<CodeableConcept>>,
    /// Vaccine which is contraindicated to fulfill the recommendation
    #[fhir(name="contraindicatedVaccineCode", min="0", max="*", summary="true", modifier="false")]
    pub contraindicated_vaccine_code: Option<Vec<CodeableConcept>>,
    /// Vaccine recommendation status
    #[fhir(name="forecastStatus", min="1", max="1", summary="true", modifier="true")]
    pub forecast_status: Option<CodeableConcept>,
    /// Vaccine administration status reason
    #[fhir(name="forecastReason", min="0", max="*", summary="true", modifier="false")]
    pub forecast_reason: Option<Vec<CodeableConcept>>,
    /// Dates governing proposed immunization
    #[fhir(name="dateCriterion", min="0", max="*", summary="false", modifier="false")]
    pub date_criterion: Option<Vec<ImmunizationRecommendationRecommendationDateCriterionBackboneElement>>,
    /// Protocol details
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Name of vaccination series
    #[fhir(name="series", min="0", max="1", summary="false", modifier="false")]
    pub series: Option<StringDt>,
    /// Recommended dose number within series
    #[fhir(name="doseNumber", min="0", max="1", summary="true", modifier="false")]
    pub dose_number: Option<StringDt>,
    /// Recommended number of doses for immunity
    #[fhir(name="seriesDoses", min="0", max="1", summary="false", modifier="false")]
    pub series_doses: Option<StringDt>,
    /// Past immunizations supporting recommendation
    #[fhir(name="supportingImmunization", min="0", max="*", summary="false", modifier="false")]
    pub supporting_immunization: Option<Vec<Reference>>,
    /// Patient observations supporting recommendation
    #[fhir(name="supportingPatientInformation", min="0", max="*", summary="false", modifier="false")]
    pub supporting_patient_information: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImmunizationRecommendationRecommendationDateCriterionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of date
    #[fhir(name="code", min="1", max="1", summary="false", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Recommended date
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<DateTimeDt>,
}


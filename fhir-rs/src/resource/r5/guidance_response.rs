use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct GuidanceResponse {
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
    /// The identifier of the request associated with this response, if any
    #[fhir(name="requestIdentifier", min="0", max="1", summary=true, modifier=false)]
    pub request_identifier: Option<Identifier>,
    /// Business identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// What guidance was requested
    #[fhir(name="module", min="1", max="1", summary=true, modifier=false)]
    pub module: Option<CodeableConcept>,
    /// success | data-requested | data-required | in-progress | failure | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Patient the request was performed for
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false)]
    pub subject: Option<Reference>,
    /// Encounter during which the response was returned
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false)]
    pub encounter: Option<Reference>,
    /// When the guidance response was processed
    #[fhir(name="occurrenceDateTime", min="0", max="1", summary=false, modifier=false)]
    pub occurrence_date_time: Option<DateTimeDt>,
    /// Device returning the guidance
    #[fhir(name="performer", min="0", max="1", summary=false, modifier=false)]
    pub performer: Option<Reference>,
    /// Why guidance is needed
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// Additional notes about the response
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Messages resulting from the evaluation of the artifact or artifacts
    #[fhir(name="evaluationMessage", min="0", max="1", summary=false, modifier=false)]
    pub evaluation_message: Option<Reference>,
    /// The output parameters of the evaluation, if any
    #[fhir(name="outputParameters", min="0", max="1", summary=false, modifier=false)]
    pub output_parameters: Option<Reference>,
    /// Proposed actions, if any
    #[fhir(name="result", min="0", max="*", summary=false, modifier=false)]
    pub result: Option<Vec<Reference>>,
    /// Additional required data
    #[fhir(name="dataRequirement", min="0", max="*", summary=false, modifier=false)]
    pub data_requirement: Option<Vec<DataRequirement>>,
}


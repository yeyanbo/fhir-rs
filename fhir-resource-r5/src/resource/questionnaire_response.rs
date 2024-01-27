use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct QuestionnaireResponse {
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
    /// Business identifier for this set of answers
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Request fulfilled by this QuestionnaireResponse
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary="true", modifier="false")]
    pub part_of: Option<Vec<Reference>>,
    /// Canonical URL of Questionnaire being answered
    #[fhir(name="questionnaire", min="1", max="1", summary="true", modifier="false")]
    pub questionnaire: Option<CanonicalDt>,
    /// in-progress | completed | amended | entered-in-error | stopped
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// The subject of the questions
    #[fhir(name="subject", min="0", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// Encounter the questionnaire response is part of
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// Date the answers were gathered
    #[fhir(name="authored", min="0", max="1", summary="true", modifier="false")]
    pub authored: Option<DateTimeDt>,
    /// The individual or device that received and recorded the answers
    #[fhir(name="author", min="0", max="1", summary="true", modifier="false")]
    pub author: Option<Reference>,
    /// The individual or device that answered the questions
    #[fhir(name="source", min="0", max="1", summary="true", modifier="false")]
    pub source: Option<Reference>,
    /// Groups and questions
    #[fhir(name="item", min="0", max="*", summary="false", modifier="false")]
    pub item: Option<Vec<QuestionnaireResponseItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct QuestionnaireResponseItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific item from Questionnaire
    #[fhir(name="linkId", min="1", max="1", summary="false", modifier="false")]
    pub link_id: Option<StringDt>,
    /// ElementDefinition - details for the item
    #[fhir(name="definition", min="0", max="1", summary="false", modifier="false")]
    pub definition: Option<UriDt>,
    /// Name for group or question text
    #[fhir(name="text", min="0", max="1", summary="false", modifier="false")]
    pub text: Option<StringDt>,
    /// The response(s) to the question
    #[fhir(name="answer", min="0", max="*", summary="false", modifier="false")]
    pub answer: Option<Vec<QuestionnaireResponseItemAnswerBackboneElement>>,
    /// Child items of group item
    #[fhir(name="item", min="0", max="*", summary="false", modifier="false")]
    pub item: Option<Vec<QuestionnaireResponseItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct QuestionnaireResponseItemAnswerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Single-valued answer to the question
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<Reference>,
    /// Child items of question
    #[fhir(name="item", min="0", max="*", summary="false", modifier="false")]
    pub item: Option<Vec<QuestionnaireResponseItemBackboneElement>>,
}


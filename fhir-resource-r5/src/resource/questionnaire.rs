use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct Questionnaire {
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
    /// Canonical identifier for this questionnaire, represented as an absolute URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Business identifier for questionnaire
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the questionnaire
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this questionnaire (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this questionnaire (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// Based on Questionnaire
    #[fhir(name="derivedFrom", min="0", max="*", summary="true", modifier="false")]
    pub derived_from: Option<Vec<CanonicalDt>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="true", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Resource that can be subject of QuestionnaireResponse
    #[fhir(name="subjectType", min="0", max="*", summary="true", modifier="false")]
    pub subject_type: Option<Vec<CodeDt>>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the questionnaire
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for questionnaire (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this questionnaire is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// When the questionnaire was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary="false", modifier="false")]
    pub approval_date: Option<DateDt>,
    /// When the questionnaire was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary="false", modifier="false")]
    pub last_review_date: Option<DateDt>,
    /// When the questionnaire is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary="true", modifier="false")]
    pub effective_period: Option<Period>,
    /// Concept that represents the overall questionnaire
    #[fhir(name="code", min="0", max="*", summary="true", modifier="false")]
    pub code: Option<Vec<Coding>>,
    /// Questions and sections within the Questionnaire
    #[fhir(name="item", min="0", max="*", summary="false", modifier="false")]
    pub item: Option<Vec<QuestionnaireItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct QuestionnaireItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for item in questionnaire
    #[fhir(name="linkId", min="1", max="1", summary="false", modifier="false")]
    pub link_id: Option<StringDt>,
    /// ElementDefinition - details for the item
    #[fhir(name="definition", min="0", max="1", summary="false", modifier="false")]
    pub definition: Option<UriDt>,
    /// Corresponding concept for this item in a terminology
    #[fhir(name="code", min="0", max="*", summary="false", modifier="false")]
    pub code: Option<Vec<Coding>>,
    /// E.g. "1(a)", "2.5.3"
    #[fhir(name="prefix", min="0", max="1", summary="false", modifier="false")]
    pub prefix: Option<StringDt>,
    /// Primary text for the item
    #[fhir(name="text", min="0", max="1", summary="false", modifier="false")]
    pub text: Option<StringDt>,
    /// group | display | boolean | decimal | integer | date | dateTime +
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeDt>,
    /// Only allow data when
    #[fhir(name="enableWhen", min="0", max="*", summary="false", modifier="true")]
    pub enable_when: Option<Vec<QuestionnaireItemEnableWhenBackboneElement>>,
    /// all | any
    #[fhir(name="enableBehavior", min="0", max="1", summary="false", modifier="false")]
    pub enable_behavior: Option<CodeDt>,
    /// hidden | protected
    #[fhir(name="disabledDisplay", min="0", max="1", summary="false", modifier="false")]
    pub disabled_display: Option<CodeDt>,
    /// Whether the item must be included in data results
    #[fhir(name="required", min="0", max="1", summary="false", modifier="false")]
    pub required: Option<BooleanDt>,
    /// Whether the item may repeat
    #[fhir(name="repeats", min="0", max="1", summary="false", modifier="false")]
    pub repeats: Option<BooleanDt>,
    /// Don't allow human editing
    #[fhir(name="readOnly", min="0", max="1", summary="false", modifier="false")]
    pub read_only: Option<BooleanDt>,
    /// No more than these many characters
    #[fhir(name="maxLength", min="0", max="1", summary="false", modifier="false")]
    pub max_length: Option<IntegerDt>,
    /// optionsOnly | optionsOrType | optionsOrString
    #[fhir(name="answerConstraint", min="0", max="1", summary="false", modifier="false")]
    pub answer_constraint: Option<CodeDt>,
    /// ValueSet containing permitted answers
    #[fhir(name="answerValueSet", min="0", max="1", summary="false", modifier="false")]
    pub answer_value_set: Option<CanonicalDt>,
    /// Permitted answer
    #[fhir(name="answerOption", min="0", max="*", summary="false", modifier="false")]
    pub answer_option: Option<Vec<QuestionnaireItemAnswerOptionBackboneElement>>,
    /// Initial value(s) when item is first rendered
    #[fhir(name="initial", min="0", max="*", summary="false", modifier="false")]
    pub initial: Option<Vec<QuestionnaireItemInitialBackboneElement>>,
    /// Nested questionnaire items
    #[fhir(name="item", min="0", max="*", summary="false", modifier="false")]
    pub item: Option<Vec<QuestionnaireItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct QuestionnaireItemAnswerOptionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Answer value
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<Reference>,
    /// Whether option is selected by default
    #[fhir(name="initialSelected", min="0", max="1", summary="false", modifier="false")]
    pub initial_selected: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct QuestionnaireItemEnableWhenBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The linkId of question that determines whether item is enabled/disabled
    #[fhir(name="question", min="1", max="1", summary="false", modifier="false")]
    pub question: Option<StringDt>,
    /// exists | = | != | > | < | >= | <=
    #[fhir(name="operator", min="1", max="1", summary="false", modifier="false")]
    pub operator: Option<CodeDt>,
    /// Value for question comparison based on operator
    #[fhir(name="answer", min="1", max="1", summary="false", modifier="false")]
    pub answer: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct QuestionnaireItemInitialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Actual value for initializing the question
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<Reference>,
}


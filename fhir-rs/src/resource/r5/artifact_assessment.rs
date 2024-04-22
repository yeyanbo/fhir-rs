use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ArtifactAssessment {
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
    /// Additional identifier for the artifact assessment
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// A short title for the assessment for use in displaying and selecting
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// How to cite the comment or rating
    #[fhir(name="citeAs", min="0", max="1", summary=false, modifier=false, choice="")]
    pub cite_as: Option<MarkdownDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// When the artifact assessment was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub approval_date: Option<DateDt>,
    /// When the artifact assessment was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub last_review_date: Option<DateDt>,
    /// The artifact assessed, commented upon or rated
    #[fhir(name="artifact", min="1", max="1", summary=true, modifier=false, choice="")]
    pub artifact: Option<UriDt>,
    /// Comment, classifier, or rating content
    #[fhir(name="content", min="0", max="*", summary=false, modifier=false, choice="")]
    pub content: Option<Vec<ArtifactAssessmentContentBackboneElement>>,
    /// submitted | triaged | waiting-for-input | resolved-no-change | resolved-change-required | deferred | duplicate | applied | published | entered-in-error
    #[fhir(name="workflowStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub workflow_status: Option<CodeDt>,
    /// unresolved | not-persuasive | persuasive | persuasive-with-modification | not-persuasive-with-modification
    #[fhir(name="disposition", min="0", max="1", summary=true, modifier=false, choice="")]
    pub disposition: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ArtifactAssessmentContentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// comment | classifier | rating | container | response | change-request
    #[fhir(name="informationType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub information_type: Option<CodeDt>,
    /// Brief summary of the content
    #[fhir(name="summary", min="0", max="1", summary=false, modifier=false, choice="")]
    pub summary: Option<MarkdownDt>,
    /// What type of content
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Rating, classifier, or assessment
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Quantitative rating
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="1", summary=false, modifier=false, choice="")]
    pub author: Option<Reference>,
    /// What the comment is directed to
    #[fhir(name="path", min="0", max="*", summary=false, modifier=false, choice="")]
    pub path: Option<Vec<UriDt>>,
    /// Additional information
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Acceptable to publicly share the resource content
    #[fhir(name="freeToShare", min="0", max="1", summary=false, modifier=false, choice="")]
    pub free_to_share: Option<BooleanDt>,
    /// Contained content
    #[fhir(name="component", min="0", max="*", summary=false, modifier=false, choice="")]
    pub component: Option<Vec<ArtifactAssessmentContentBackboneElement>>,
}


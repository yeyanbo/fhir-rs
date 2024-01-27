use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct RequestOrchestration {
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
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary="true", modifier="false")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary="true", modifier="false")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// Fulfills plan, proposal, or order
    #[fhir(name="basedOn", min="0", max="*", summary="false", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// Request(s) replaced by this request
    #[fhir(name="replaces", min="0", max="*", summary="false", modifier="false")]
    pub replaces: Option<Vec<Reference>>,
    /// Composite request this is part of
    #[fhir(name="groupIdentifier", min="0", max="1", summary="true", modifier="false")]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary="true", modifier="true")]
    pub intent: Option<CodeDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="true", modifier="false")]
    pub priority: Option<CodeDt>,
    /// What's being requested/ordered
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Who the request orchestration is about
    #[fhir(name="subject", min="0", max="1", summary="false", modifier="false")]
    pub subject: Option<Reference>,
    /// Created as part of
    #[fhir(name="encounter", min="0", max="1", summary="false", modifier="false")]
    pub encounter: Option<Reference>,
    /// When the request orchestration was authored
    #[fhir(name="authoredOn", min="0", max="1", summary="false", modifier="false")]
    pub authored_on: Option<DateTimeDt>,
    /// Device or practitioner that authored the request orchestration
    #[fhir(name="author", min="0", max="1", summary="false", modifier="false")]
    pub author: Option<Reference>,
    /// Why the request orchestration is needed
    #[fhir(name="reason", min="0", max="*", summary="false", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// What goals
    #[fhir(name="goal", min="0", max="*", summary="false", modifier="false")]
    pub goal: Option<Vec<Reference>>,
    /// Additional notes about the response
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Proposed actions, if any
    #[fhir(name="action", min="0", max="*", summary="false", modifier="false")]
    pub action: Option<Vec<RequestOrchestrationActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific item from the PlanDefinition
    #[fhir(name="linkId", min="0", max="1", summary="false", modifier="false")]
    pub link_id: Option<StringDt>,
    /// User-visible prefix for the action (e.g. 1. or A.)
    #[fhir(name="prefix", min="0", max="1", summary="false", modifier="false")]
    pub prefix: Option<StringDt>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Short description of the action
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    #[fhir(name="textEquivalent", min="0", max="1", summary="true", modifier="false")]
    pub text_equivalent: Option<MarkdownDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="false", modifier="false")]
    pub priority: Option<CodeDt>,
    /// Code representing the meaning of the action or sub-actions
    #[fhir(name="code", min="0", max="*", summary="false", modifier="false")]
    pub code: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the intended performer of the action
    #[fhir(name="documentation", min="0", max="*", summary="false", modifier="false")]
    pub documentation: Option<Vec<RelatedArtifact>>,
    /// What goals
    #[fhir(name="goal", min="0", max="*", summary="false", modifier="false")]
    pub goal: Option<Vec<Reference>>,
    /// Whether or not the action is applicable
    #[fhir(name="condition", min="0", max="*", summary="false", modifier="false")]
    pub condition: Option<Vec<RequestOrchestrationActionConditionBackboneElement>>,
    /// Input data requirements
    #[fhir(name="input", min="0", max="*", summary="false", modifier="false")]
    pub input: Option<Vec<RequestOrchestrationActionInputBackboneElement>>,
    /// Output data definition
    #[fhir(name="output", min="0", max="*", summary="false", modifier="false")]
    pub output: Option<Vec<RequestOrchestrationActionOutputBackboneElement>>,
    /// Relationship to another action
    #[fhir(name="relatedAction", min="0", max="*", summary="false", modifier="false")]
    pub related_action: Option<Vec<RequestOrchestrationActionRelatedActionBackboneElement>>,
    /// When the action should take place
    #[fhir(name="timing", min="0", max="1", summary="false", modifier="false")]
    pub timing: Option<Timing>,
    /// Where it should happen
    #[fhir(name="location", min="0", max="1", summary="false", modifier="false")]
    pub location: Option<CodeableReference>,
    /// Who should perform the action
    #[fhir(name="participant", min="0", max="*", summary="false", modifier="false")]
    pub participant: Option<Vec<RequestOrchestrationActionParticipantBackboneElement>>,
    /// create | update | remove | fire-event
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// visual-group | logical-group | sentence-group
    #[fhir(name="groupingBehavior", min="0", max="1", summary="false", modifier="false")]
    pub grouping_behavior: Option<CodeDt>,
    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    #[fhir(name="selectionBehavior", min="0", max="1", summary="false", modifier="false")]
    pub selection_behavior: Option<CodeDt>,
    /// must | could | must-unless-documented
    #[fhir(name="requiredBehavior", min="0", max="1", summary="false", modifier="false")]
    pub required_behavior: Option<CodeDt>,
    /// yes | no
    #[fhir(name="precheckBehavior", min="0", max="1", summary="false", modifier="false")]
    pub precheck_behavior: Option<CodeDt>,
    /// single | multiple
    #[fhir(name="cardinalityBehavior", min="0", max="1", summary="false", modifier="false")]
    pub cardinality_behavior: Option<CodeDt>,
    /// The target of the action
    #[fhir(name="resource", min="0", max="1", summary="false", modifier="false")]
    pub resource: Option<Reference>,
    /// Description of the activity to be performed
    #[fhir(name="definition", min="0", max="1", summary="false", modifier="false")]
    pub definition: Option<UriDt>,
    /// Transform to apply the template
    #[fhir(name="transform", min="0", max="1", summary="false", modifier="false")]
    pub transform: Option<CanonicalDt>,
    /// Dynamic aspects of the definition
    #[fhir(name="dynamicValue", min="0", max="*", summary="false", modifier="false")]
    pub dynamic_value: Option<Vec<RequestOrchestrationActionDynamicValueBackboneElement>>,
    /// Sub action
    #[fhir(name="action", min="0", max="*", summary="false", modifier="false")]
    pub action: Option<Vec<RequestOrchestrationActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeDt>,
    /// Who or what can participate
    #[fhir(name="typeCanonical", min="0", max="1", summary="false", modifier="false")]
    pub type_canonical: Option<CanonicalDt>,
    /// Who or what can participate
    #[fhir(name="typeReference", min="0", max="1", summary="false", modifier="false")]
    pub type_reference: Option<Reference>,
    /// E.g. Nurse, Surgeon, Parent, etc
    #[fhir(name="role", min="0", max="1", summary="false", modifier="false")]
    pub role: Option<CodeableConcept>,
    /// E.g. Author, Reviewer, Witness, etc
    #[fhir(name="function", min="0", max="1", summary="false", modifier="false")]
    pub function: Option<CodeableConcept>,
    /// Who/what is participating?
    #[fhir(name="actor", min="0", max="1", summary="false", modifier="false")]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionDynamicValueBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The path to the element to be set dynamically
    #[fhir(name="path", min="0", max="1", summary="false", modifier="false")]
    pub path: Option<StringDt>,
    /// An expression that provides the dynamic value for the customization
    #[fhir(name="expression", min="0", max="1", summary="false", modifier="false")]
    pub expression: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionConditionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// applicability | start | stop
    #[fhir(name="kind", min="1", max="1", summary="false", modifier="false")]
    pub kind: Option<CodeDt>,
    /// Boolean-valued expression
    #[fhir(name="expression", min="0", max="1", summary="false", modifier="false")]
    pub expression: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionOutputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// What data is provided
    #[fhir(name="requirement", min="0", max="1", summary="false", modifier="false")]
    pub requirement: Option<DataRequirement>,
    /// What data is provided
    #[fhir(name="relatedData", min="0", max="1", summary="false", modifier="false")]
    pub related_data: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionInputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// What data is provided
    #[fhir(name="requirement", min="0", max="1", summary="false", modifier="false")]
    pub requirement: Option<DataRequirement>,
    /// What data is provided
    #[fhir(name="relatedData", min="0", max="1", summary="false", modifier="false")]
    pub related_data: Option<IdDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RequestOrchestrationActionRelatedActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What action this is related to
    #[fhir(name="targetId", min="1", max="1", summary="false", modifier="false")]
    pub target_id: Option<IdDt>,
    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    #[fhir(name="relationship", min="1", max="1", summary="false", modifier="false")]
    pub relationship: Option<CodeDt>,
    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    #[fhir(name="endRelationship", min="0", max="1", summary="false", modifier="false")]
    pub end_relationship: Option<CodeDt>,
    /// Time offset for the relationship
    #[fhir(name="offset", min="0", max="1", summary="false", modifier="false")]
    pub offset: Option<Range>,
}


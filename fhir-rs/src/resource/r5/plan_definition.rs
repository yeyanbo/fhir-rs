use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct PlanDefinition {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Canonical identifier for this plan definition, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the plan definition
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the plan definition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this plan definition (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Name for this plan definition (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// Subordinate title of the plan definition
    #[fhir(name="subtitle", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subtitle: Option<StringDt>,
    /// order-set | clinical-protocol | eca-rule | workflow-definition
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub experimental: Option<BooleanDt>,
    /// Type of individual the plan definition is focused on
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subject: Option<CanonicalDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the plan definition
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for plan definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this plan definition is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub purpose: Option<MarkdownDt>,
    /// Describes the clinical usage of the plan
    #[fhir(name="usage", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub usage: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright_label: Option<StringDt>,
    /// When the plan definition was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub approval_date: Option<DateDt>,
    /// When the plan definition was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub last_review_date: Option<DateDt>,
    /// When the plan definition is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment
    #[fhir(name="topic", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Logic used by the plan definition
    #[fhir(name="library", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub library: Option<Vec<CanonicalDt>>,
    /// What the plan is trying to accomplish
    #[fhir(name="goal", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub goal: Option<Vec<PlanDefinitionGoalBackboneElement>>,
    /// Actors within the plan
    #[fhir(name="actor", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub actor: Option<Vec<PlanDefinitionActorBackboneElement>>,
    /// Action defined by the plan
    #[fhir(name="action", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub action: Option<Vec<PlanDefinitionActionBackboneElement>>,
    /// Preconditions for service
    #[fhir(name="asNeeded", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub as_needed: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// Describes the actor
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Who or what can be this actor
    #[fhir(name="option", min="1", max="*", summary=false, modifier=false, choice=false)]
    pub option: Option<Vec<PlanDefinitionActorOptionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActorOptionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeDt>,
    /// Who or what can participate
    #[fhir(name="typeCanonical", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_canonical: Option<CanonicalDt>,
    /// Who or what can participate
    #[fhir(name="typeReference", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_reference: Option<Reference>,
    /// E.g. Nurse, Surgeon, Parent
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub role: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for the action in the PlanDefinition
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub link_id: Option<StringDt>,
    /// User-visible prefix for the action (e.g. 1. or A.)
    #[fhir(name="prefix", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub prefix: Option<StringDt>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// Brief description of the action
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Static text equivalent of the action, used if the dynamic aspects cannot be interpreted by the receiving system
    #[fhir(name="textEquivalent", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text_equivalent: Option<MarkdownDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub priority: Option<CodeDt>,
    /// Code representing the meaning of the action or sub-actions
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// Why the action should be performed
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub reason: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the intended performer of the action
    #[fhir(name="documentation", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub documentation: Option<Vec<RelatedArtifact>>,
    /// What goals this action supports
    #[fhir(name="goalId", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub goal_id: Option<Vec<IdDt>>,
    /// Type of individual the action is focused on
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subject: Option<CanonicalDt>,
    /// When the action should be triggered
    #[fhir(name="trigger", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub trigger: Option<Vec<TriggerDefinition>>,
    /// Whether or not the action is applicable
    #[fhir(name="condition", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub condition: Option<Vec<PlanDefinitionActionConditionBackboneElement>>,
    /// Input data requirements
    #[fhir(name="input", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub input: Option<Vec<PlanDefinitionActionInputBackboneElement>>,
    /// Output data definition
    #[fhir(name="output", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub output: Option<Vec<PlanDefinitionActionOutputBackboneElement>>,
    /// Relationship to another action
    #[fhir(name="relatedAction", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub related_action: Option<Vec<PlanDefinitionActionRelatedActionBackboneElement>>,
    /// When the action should take place
    #[fhir(name="timing", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub timing: Option<Timing>,
    /// Where it should happen
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub location: Option<CodeableReference>,
    /// Who should participate in the action
    #[fhir(name="participant", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub participant: Option<Vec<PlanDefinitionActionParticipantBackboneElement>>,
    /// create | update | remove | fire-event
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// visual-group | logical-group | sentence-group
    #[fhir(name="groupingBehavior", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub grouping_behavior: Option<CodeDt>,
    /// any | all | all-or-none | exactly-one | at-most-one | one-or-more
    #[fhir(name="selectionBehavior", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub selection_behavior: Option<CodeDt>,
    /// must | could | must-unless-documented
    #[fhir(name="requiredBehavior", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub required_behavior: Option<CodeDt>,
    /// yes | no
    #[fhir(name="precheckBehavior", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub precheck_behavior: Option<CodeDt>,
    /// single | multiple
    #[fhir(name="cardinalityBehavior", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub cardinality_behavior: Option<CodeDt>,
    /// Description of the activity to be performed
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub definition: Option<UriDt>,
    /// Transform to apply the template
    #[fhir(name="transform", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub transform: Option<CanonicalDt>,
    /// Dynamic aspects of the definition
    #[fhir(name="dynamicValue", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub dynamic_value: Option<Vec<PlanDefinitionActionDynamicValueBackboneElement>>,
    /// A sub-action
    #[fhir(name="action", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub action: Option<Vec<PlanDefinitionActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionOutputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// What data is provided
    #[fhir(name="requirement", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub requirement: Option<DataRequirement>,
    /// What data is provided
    #[fhir(name="relatedData", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub related_data: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionDynamicValueBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The path to the element to be set dynamically
    #[fhir(name="path", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub path: Option<StringDt>,
    /// An expression that provides the dynamic value for the customization
    #[fhir(name="expression", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub expression: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionConditionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// applicability | start | stop
    #[fhir(name="kind", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub kind: Option<CodeDt>,
    /// Boolean-valued expression
    #[fhir(name="expression", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub expression: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionRelatedActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What action is this related to
    #[fhir(name="targetId", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub target_id: Option<IdDt>,
    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    #[fhir(name="relationship", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub relationship: Option<CodeDt>,
    /// before | before-start | before-end | concurrent | concurrent-with-start | concurrent-with-end | after | after-start | after-end
    #[fhir(name="endRelationship", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub end_relationship: Option<CodeDt>,
    /// Time offset for the relationship
    #[fhir(name="offset", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub offset: Option<Range>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionInputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// User-visible title
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// What data is provided
    #[fhir(name="requirement", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub requirement: Option<DataRequirement>,
    /// What data is provided
    #[fhir(name="relatedData", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub related_data: Option<IdDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionActionParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What actor
    #[fhir(name="actorId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub actor_id: Option<StringDt>,
    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeDt>,
    /// Who or what can participate
    #[fhir(name="typeCanonical", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_canonical: Option<CanonicalDt>,
    /// Who or what can participate
    #[fhir(name="typeReference", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_reference: Option<Reference>,
    /// E.g. Nurse, Surgeon, Parent
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub role: Option<CodeableConcept>,
    /// E.g. Author, Reviewer, Witness, etc
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub function: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionGoalBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// E.g. Treatment, dietary, behavioral
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub category: Option<CodeableConcept>,
    /// Code or text describing the goal
    #[fhir(name="description", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<CodeableConcept>,
    /// high-priority | medium-priority | low-priority
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub priority: Option<CodeableConcept>,
    /// When goal pursuit begins
    #[fhir(name="start", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub start: Option<CodeableConcept>,
    /// What does the goal address
    #[fhir(name="addresses", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub addresses: Option<Vec<CodeableConcept>>,
    /// Supporting documentation for the goal
    #[fhir(name="documentation", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub documentation: Option<Vec<RelatedArtifact>>,
    /// Target outcome for the goal
    #[fhir(name="target", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub target: Option<Vec<PlanDefinitionGoalTargetBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PlanDefinitionGoalTargetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The parameter whose value is to be tracked
    #[fhir(name="measure", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub measure: Option<CodeableConcept>,
    /// The target value to be achieved
    #[fhir(name="detail", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub detail: Option<Ratio>,
    /// Reach goal within
    #[fhir(name="due", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub due: Option<Duration>,
}


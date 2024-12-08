use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Goal {
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
    /// External Ids for this goal
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | planned | accepted | active | on-hold | completed | cancelled | entered-in-error | rejected
    #[fhir(name="lifecycleStatus", min="1", max="1", summary=true, modifier=true)]
    pub lifecycle_status: Option<CodeDt>,
    /// in-progress | improving | worsening | no-change | achieved | sustaining | not-achieved | no-progress | not-attainable
    #[fhir(name="achievementStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub achievement_status: Option<CodeableConcept>,
    /// E.g. Treatment, dietary, behavioral, etc
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// After meeting the goal, ongoing activity is needed to sustain the goal objective
    #[fhir(name="continuous", min="0", max="1", summary=false, modifier=false, choice="")]
    pub continuous: Option<BooleanDt>,
    /// high-priority | medium-priority | low-priority
    #[fhir(name="priority", min="0", max="1", summary=true, modifier=false, choice="")]
    pub priority: Option<CodeableConcept>,
    /// Code or text describing goal
    #[fhir(name="description", min="1", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<CodeableConcept>,
    /// Who this goal is intended for
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// When goal pursuit begins
    #[fhir(name="start", min="0", max="1", summary=true, modifier=false, choice="")]
    pub start: Option<CodeableConcept>,
    /// Target outcome for the goal
    #[fhir(name="target", min="0", max="*", summary=false, modifier=false, choice="")]
    pub target: Option<Vec<GoalTargetBackboneElement>>,
    /// When goal status took effect
    #[fhir(name="statusDate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub status_date: Option<DateDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub status_reason: Option<StringDt>,
    /// Who's responsible for creating Goal?
    #[fhir(name="source", min="0", max="1", summary=true, modifier=false, choice="")]
    pub source: Option<Reference>,
    /// Issues addressed by this goal
    #[fhir(name="addresses", min="0", max="*", summary=false, modifier=false, choice="")]
    pub addresses: Option<Vec<Reference>>,
    /// Comments about the goal
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// What result was achieved regarding the goal?
    #[fhir(name="outcome", min="0", max="*", summary=false, modifier=false, choice="")]
    pub outcome: Option<Vec<CodeableReference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct GoalTargetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The parameter whose value is being tracked
    #[fhir(name="measure", min="0", max="1", summary=true, modifier=false, choice="")]
    pub measure: Option<CodeableConcept>,
    /// The target value to be achieved
    #[fhir(name="detail", min="0", max="1", summary=true, modifier=false, choice="")]
    pub detail: Option<Ratio>,
    /// Reach goal on or before
    #[fhir(name="due", min="0", max="1", summary=true, modifier=false, choice="")]
    pub due: Option<Duration>,
}


use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Task {
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
    /// Task Instance Identifier
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Formal definition of task
    #[fhir(name="instantiatesCanonical", min="0", max="1", summary="true", modifier="false")]
    pub instantiates_canonical: Option<CanonicalDt>,
    /// Formal definition of task
    #[fhir(name="instantiatesUri", min="0", max="1", summary="true", modifier="false")]
    pub instantiates_uri: Option<UriDt>,
    /// Request fulfilled by this task
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// Requisition or grouper id
    #[fhir(name="groupIdentifier", min="0", max="1", summary="true", modifier="false")]
    pub group_identifier: Option<Identifier>,
    /// Composite task
    #[fhir(name="partOf", min="0", max="*", summary="true", modifier="false")]
    pub part_of: Option<Vec<Reference>>,
    /// draft | requested | received | accepted | +
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary="true", modifier="false")]
    pub status_reason: Option<CodeableReference>,
    /// E.g. "Specimen collected", "IV prepped"
    #[fhir(name="businessStatus", min="0", max="1", summary="true", modifier="false")]
    pub business_status: Option<CodeableConcept>,
    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary="true", modifier="false")]
    pub intent: Option<CodeDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="false", modifier="false")]
    pub priority: Option<CodeDt>,
    /// True if Task is prohibiting action
    #[fhir(name="doNotPerform", min="0", max="1", summary="true", modifier="true")]
    pub do_not_perform: Option<BooleanDt>,
    /// Task Type
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Human-readable explanation of task
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<StringDt>,
    /// What task is acting on
    #[fhir(name="focus", min="0", max="1", summary="true", modifier="false")]
    pub focus: Option<Reference>,
    /// Beneficiary of the Task
    #[fhir(name="for", min="0", max="1", summary="true", modifier="false")]
    pub for_: Option<Reference>,
    /// Healthcare event during which this task originated
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// When the task should be performed
    #[fhir(name="requestedPeriod", min="0", max="1", summary="true", modifier="false")]
    pub requested_period: Option<Period>,
    /// Start and end time of execution
    #[fhir(name="executionPeriod", min="0", max="1", summary="true", modifier="false")]
    pub execution_period: Option<Period>,
    /// Task Creation Date
    #[fhir(name="authoredOn", min="0", max="1", summary="false", modifier="false")]
    pub authored_on: Option<DateTimeDt>,
    /// Task Last Modified Date
    #[fhir(name="lastModified", min="0", max="1", summary="true", modifier="false")]
    pub last_modified: Option<DateTimeDt>,
    /// Who is asking for task to be done
    #[fhir(name="requester", min="0", max="1", summary="true", modifier="false")]
    pub requester: Option<Reference>,
    /// Who should perform Task
    #[fhir(name="requestedPerformer", min="0", max="*", summary="false", modifier="false")]
    pub requested_performer: Option<Vec<CodeableReference>>,
    /// Responsible individual
    #[fhir(name="owner", min="0", max="1", summary="true", modifier="false")]
    pub owner: Option<Reference>,
    /// Who or what performed the task
    #[fhir(name="performer", min="0", max="*", summary="true", modifier="false")]
    pub performer: Option<Vec<TaskPerformerBackboneElement>>,
    /// Where task occurs
    #[fhir(name="location", min="0", max="1", summary="true", modifier="false")]
    pub location: Option<Reference>,
    /// Why task is needed
    #[fhir(name="reason", min="0", max="*", summary="false", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Associated insurance coverage
    #[fhir(name="insurance", min="0", max="*", summary="false", modifier="false")]
    pub insurance: Option<Vec<Reference>>,
    /// Comments made about the task
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Key events in history of the Task
    #[fhir(name="relevantHistory", min="0", max="*", summary="false", modifier="false")]
    pub relevant_history: Option<Vec<Reference>>,
    /// Constraints on fulfillment tasks
    #[fhir(name="restriction", min="0", max="1", summary="false", modifier="false")]
    pub restriction: Option<TaskRestrictionBackboneElement>,
    /// Information used to perform task
    #[fhir(name="input", min="0", max="*", summary="false", modifier="false")]
    pub input: Option<Vec<TaskInputBackboneElement>>,
    /// Information produced as part of task
    #[fhir(name="output", min="0", max="*", summary="false", modifier="false")]
    pub output: Option<Vec<TaskOutputBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TaskRestrictionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// How many times to repeat
    #[fhir(name="repetitions", min="0", max="1", summary="false", modifier="false")]
    pub repetitions: Option<PositiveIntDt>,
    /// When fulfillment is sought
    #[fhir(name="period", min="0", max="1", summary="false", modifier="false")]
    pub period: Option<Period>,
    /// For whom is fulfillment sought?
    #[fhir(name="recipient", min="0", max="*", summary="false", modifier="false")]
    pub recipient: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TaskOutputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for output
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Result of output
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<Meta>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TaskInputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for the input
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Content to use in performing the task
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<Meta>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TaskPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of performance
    #[fhir(name="function", min="0", max="1", summary="true", modifier="false")]
    pub function: Option<CodeableConcept>,
    /// Who performed the task
    #[fhir(name="actor", min="1", max="1", summary="true", modifier="false")]
    pub actor: Option<Reference>,
}


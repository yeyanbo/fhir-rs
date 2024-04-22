use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct AuditEvent {
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
    /// Type/identifier of event
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Specific type of event
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Type of action performed during the event
    #[fhir(name="action", min="0", max="1", summary=true, modifier=false, choice="")]
    pub action: Option<CodeDt>,
    /// emergency | alert | critical | error | warning | notice | informational | debug
    #[fhir(name="severity", min="0", max="1", summary=true, modifier=false, choice="")]
    pub severity: Option<CodeDt>,
    /// When the activity occurred
    #[fhir(name="occurred", min="0", max="1", summary=false, modifier=false, choice="")]
    pub occurred: Option<DateTimeDt>,
    /// Time when the event was recorded
    #[fhir(name="recorded", min="1", max="1", summary=true, modifier=false, choice="")]
    pub recorded: Option<InstantDt>,
    /// Whether the event succeeded or failed
    #[fhir(name="outcome", min="0", max="1", summary=true, modifier=false, choice="")]
    pub outcome: Option<AuditEventOutcomeBackboneElement>,
    /// Authorization related to the event
    #[fhir(name="authorization", min="0", max="*", summary=true, modifier=false, choice="")]
    pub authorization: Option<Vec<CodeableConcept>>,
    /// Workflow authorization within which this event occurred
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// The patient is the subject of the data used/created/updated/deleted during the activity
    #[fhir(name="patient", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient: Option<Reference>,
    /// Encounter within which this event occurred or which the event is tightly associated
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Actor involved in the event
    #[fhir(name="agent", min="1", max="*", summary=true, modifier=false, choice="")]
    pub agent: Option<Vec<AuditEventAgentBackboneElement>>,
    /// Audit Event Reporter
    #[fhir(name="source", min="1", max="1", summary=true, modifier=false, choice="")]
    pub source: Option<AuditEventSourceBackboneElement>,
    /// Data or objects used
    #[fhir(name="entity", min="0", max="*", summary=true, modifier=false, choice="")]
    pub entity: Option<Vec<AuditEventEntityBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AuditEventSourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Logical source location within the enterprise
    #[fhir(name="site", min="0", max="1", summary=false, modifier=false, choice="")]
    pub site: Option<Reference>,
    /// The identity of source detecting the event
    #[fhir(name="observer", min="1", max="1", summary=true, modifier=false, choice="")]
    pub observer: Option<Reference>,
    /// The type of source where event originated
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AuditEventOutcomeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether the event succeeded or failed
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<Coding>,
    /// Additional outcome detail
    #[fhir(name="detail", min="0", max="*", summary=true, modifier=false, choice="")]
    pub detail: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AuditEventAgentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// How agent participated
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Agent role in the event
    #[fhir(name="role", min="0", max="*", summary=false, modifier=false, choice="")]
    pub role: Option<Vec<CodeableConcept>>,
    /// Identifier of who
    #[fhir(name="who", min="1", max="1", summary=true, modifier=false, choice="")]
    pub who: Option<Reference>,
    /// Whether user is initiator
    #[fhir(name="requestor", min="0", max="1", summary=true, modifier=false, choice="")]
    pub requestor: Option<BooleanDt>,
    /// The agent location when the event occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// Policy that authorized the agent participation in the event
    #[fhir(name="policy", min="0", max="*", summary=false, modifier=false, choice="")]
    pub policy: Option<Vec<UriDt>>,
    /// This agent network location for the activity
    #[fhir(name="network", min="0", max="1", summary=false, modifier=false, choice="")]
    pub network: Option<StringDt>,
    /// Allowable authorization for this agent
    #[fhir(name="authorization", min="0", max="*", summary=false, modifier=false, choice="")]
    pub authorization: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AuditEventEntityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specific instance of resource
    #[fhir(name="what", min="0", max="1", summary=true, modifier=false, choice="")]
    pub what: Option<Reference>,
    /// What role the entity played
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
    /// Security labels on the entity
    #[fhir(name="securityLabel", min="0", max="*", summary=false, modifier=false, choice="")]
    pub security_label: Option<Vec<CodeableConcept>>,
    /// Query parameters
    #[fhir(name="query", min="0", max="1", summary=true, modifier=false, choice="")]
    pub query: Option<Base64BinaryDt>,
    /// Additional Information about the entity
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail: Option<Vec<AuditEventEntityDetailBackboneElement>>,
    /// Entity is attributed to this agent
    #[fhir(name="agent", min="0", max="*", summary=false, modifier=false, choice="")]
    pub agent: Option<Vec<AuditEventAgentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AuditEventEntityDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of the property
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Property value
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Base64BinaryDt>,
}


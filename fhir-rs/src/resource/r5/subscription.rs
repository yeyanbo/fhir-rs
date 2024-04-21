use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Subscription {
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
    /// Additional identifiers (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Human readable name for this subscription
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// requested | active | error | off | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reference to the subscription topic being subscribed to
    #[fhir(name="topic", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub topic: Option<CanonicalDt>,
    /// Contact details for source (e.g. troubleshooting)
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub contact: Option<Vec<ContactPoint>>,
    /// When to automatically delete the subscription
    #[fhir(name="end", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub end: Option<InstantDt>,
    /// Entity responsible for Subscription changes
    #[fhir(name="managingEntity", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub managing_entity: Option<Reference>,
    /// Description of why this subscription was created
    #[fhir(name="reason", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub reason: Option<StringDt>,
    /// Criteria for narrowing the subscription topic stream
    #[fhir(name="filterBy", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub filter_by: Option<Vec<SubscriptionFilterByBackboneElement>>,
    /// Channel type for notifications
    #[fhir(name="channelType", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub channel_type: Option<Coding>,
    /// Where the channel points to
    #[fhir(name="endpoint", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub endpoint: Option<UrlDt>,
    /// Channel type
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub parameter: Option<Vec<SubscriptionParameterBackboneElement>>,
    /// Interval in seconds to send 'heartbeat' notification
    #[fhir(name="heartbeatPeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub heartbeat_period: Option<UnsignedIntDt>,
    /// Timeout in seconds to attempt notification delivery
    #[fhir(name="timeout", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub timeout: Option<UnsignedIntDt>,
    /// MIME type to send, or omit for no payload
    #[fhir(name="contentType", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub content_type: Option<CodeDt>,
    /// empty | id-only | full-resource
    #[fhir(name="content", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub content: Option<CodeDt>,
    /// Maximum number of events that can be combined in a single notification
    #[fhir(name="maxCount", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub max_count: Option<PositiveIntDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionFilterByBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Allowed Resource (reference to definition) for this Subscription filter
    #[fhir(name="resourceType", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub resource_type: Option<UriDt>,
    /// Filter label defined in SubscriptionTopic
    #[fhir(name="filterParameter", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub filter_parameter: Option<StringDt>,
    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[fhir(name="comparator", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub comparator: Option<CodeDt>,
    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    #[fhir(name="modifier", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub modifier: Option<CodeDt>,
    /// Literal value or resource path
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub value: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name (key) of the parameter
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Value of the parameter to use or pass through
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub value: Option<StringDt>,
}


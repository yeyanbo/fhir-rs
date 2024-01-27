use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct SubscriptionTopic {
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
    /// Canonical identifier for this subscription topic, represented as an absolute URI (globally unique)
    #[fhir(name="url", min="1", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Business identifier for subscription topic
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the subscription topic
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this subscription topic (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this subscription topic (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// Based on FHIR protocol or definition
    #[fhir(name="derivedFrom", min="0", max="*", summary="true", modifier="false")]
    pub derived_from: Option<Vec<CanonicalDt>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// If for testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="true", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Date status first applied
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// The name of the individual or organization that published the SubscriptionTopic
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the SubscriptionTopic
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Content intends to support these contexts
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction of the SubscriptionTopic (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this SubscriptionTopic is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// When SubscriptionTopic is/was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary="false", modifier="false")]
    pub approval_date: Option<DateDt>,
    /// Date the Subscription Topic was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary="false", modifier="false")]
    pub last_review_date: Option<DateDt>,
    /// The effective date range for the SubscriptionTopic
    #[fhir(name="effectivePeriod", min="0", max="1", summary="true", modifier="false")]
    pub effective_period: Option<Period>,
    /// Definition of a resource-based trigger for the subscription topic
    #[fhir(name="resourceTrigger", min="0", max="*", summary="true", modifier="false")]
    pub resource_trigger: Option<Vec<SubscriptionTopicResourceTriggerBackboneElement>>,
    /// Event definitions the SubscriptionTopic
    #[fhir(name="eventTrigger", min="0", max="*", summary="true", modifier="false")]
    pub event_trigger: Option<Vec<SubscriptionTopicEventTriggerBackboneElement>>,
    /// Properties by which a Subscription can filter notifications from the SubscriptionTopic
    #[fhir(name="canFilterBy", min="0", max="*", summary="true", modifier="false")]
    pub can_filter_by: Option<Vec<SubscriptionTopicCanFilterByBackboneElement>>,
    /// Properties for describing the shape of notifications generated by this topic
    #[fhir(name="notificationShape", min="0", max="*", summary="true", modifier="false")]
    pub notification_shape: Option<Vec<SubscriptionTopicNotificationShapeBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionTopicEventTriggerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Text representation of the event trigger
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Event which can trigger a notification from the SubscriptionTopic
    #[fhir(name="event", min="1", max="1", summary="true", modifier="false")]
    pub event: Option<CodeableConcept>,
    /// Data Type or Resource (reference to definition) for this trigger definition
    #[fhir(name="resource", min="1", max="1", summary="true", modifier="false")]
    pub resource: Option<UriDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionTopicCanFilterByBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Description of this filter parameter
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// URL of the triggering Resource that this filter applies to
    #[fhir(name="resource", min="0", max="1", summary="true", modifier="false")]
    pub resource: Option<UriDt>,
    /// Human-readable and computation-friendly name for a filter parameter usable by subscriptions on this topic, via Subscription.filterBy.filterParameter
    #[fhir(name="filterParameter", min="1", max="1", summary="true", modifier="false")]
    pub filter_parameter: Option<StringDt>,
    /// Canonical URL for a filterParameter definition
    #[fhir(name="filterDefinition", min="0", max="1", summary="true", modifier="false")]
    pub filter_definition: Option<UriDt>,
    /// eq | ne | gt | lt | ge | le | sa | eb | ap
    #[fhir(name="comparator", min="0", max="*", summary="false", modifier="false")]
    pub comparator: Option<Vec<CodeDt>>,
    /// missing | exact | contains | not | text | in | not-in | below | above | type | identifier | of-type | code-text | text-advanced | iterate
    #[fhir(name="modifier", min="0", max="*", summary="false", modifier="false")]
    pub modifier: Option<Vec<CodeDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionTopicResourceTriggerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Text representation of the resource trigger
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Data Type or Resource (reference to definition) for this trigger definition
    #[fhir(name="resource", min="1", max="1", summary="true", modifier="false")]
    pub resource: Option<UriDt>,
    /// create | update | delete
    #[fhir(name="supportedInteraction", min="0", max="*", summary="true", modifier="false")]
    pub supported_interaction: Option<Vec<CodeDt>>,
    /// Query based trigger rule
    #[fhir(name="queryCriteria", min="0", max="1", summary="true", modifier="false")]
    pub query_criteria: Option<SubscriptionTopicResourceTriggerQueryCriteriaBackboneElement>,
    /// FHIRPath based trigger rule
    #[fhir(name="fhirPathCriteria", min="0", max="1", summary="true", modifier="false")]
    pub fhir_path_criteria: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionTopicResourceTriggerQueryCriteriaBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Rule applied to previous resource state
    #[fhir(name="previous", min="0", max="1", summary="true", modifier="false")]
    pub previous: Option<StringDt>,
    /// test-passes | test-fails
    #[fhir(name="resultForCreate", min="0", max="1", summary="true", modifier="false")]
    pub result_for_create: Option<CodeDt>,
    /// Rule applied to current resource state
    #[fhir(name="current", min="0", max="1", summary="true", modifier="false")]
    pub current: Option<StringDt>,
    /// test-passes | test-fails
    #[fhir(name="resultForDelete", min="0", max="1", summary="true", modifier="false")]
    pub result_for_delete: Option<CodeDt>,
    /// Both must be true flag
    #[fhir(name="requireBoth", min="0", max="1", summary="true", modifier="false")]
    pub require_both: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionTopicNotificationShapeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// URL of the Resource that is the focus (main) resource in a notification shape
    #[fhir(name="resource", min="1", max="1", summary="true", modifier="false")]
    pub resource: Option<UriDt>,
    /// Include directives, rooted in the resource for this shape
    #[fhir(name="include", min="0", max="*", summary="true", modifier="false")]
    pub include: Option<Vec<StringDt>>,
    /// Reverse include directives, rooted in the resource for this shape
    #[fhir(name="revInclude", min="0", max="*", summary="true", modifier="false")]
    pub rev_include: Option<Vec<StringDt>>,
}


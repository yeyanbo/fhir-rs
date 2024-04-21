use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SubscriptionStatus {
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
    /// requested | active | error | off | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub status: Option<CodeDt>,
    /// handshake | heartbeat | event-notification | query-status | query-event
    #[fhir(name="type", min="1", max="1", summary=true, modifier=true)]
    pub type_: Option<CodeDt>,
    /// Events since the Subscription was created
    #[fhir(name="eventsSinceSubscriptionStart", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub events_since_subscription_start: Option<Integer64Dt>,
    /// Detailed information about any events relevant to this notification
    #[fhir(name="notificationEvent", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub notification_event: Option<Vec<SubscriptionStatusNotificationEventBackboneElement>>,
    /// Reference to the Subscription responsible for this notification
    #[fhir(name="subscription", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub subscription: Option<Reference>,
    /// Reference to the SubscriptionTopic this notification relates to
    #[fhir(name="topic", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub topic: Option<CanonicalDt>,
    /// List of errors on the subscription
    #[fhir(name="error", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub error: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubscriptionStatusNotificationEventBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Sequencing index of this event
    #[fhir(name="eventNumber", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub event_number: Option<Integer64Dt>,
    /// The instant this event occurred
    #[fhir(name="timestamp", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub timestamp: Option<InstantDt>,
    /// Reference to the primary resource or information of this event
    #[fhir(name="focus", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub focus: Option<Reference>,
    /// References related to the focus resource and/or context of this event
    #[fhir(name="additionalContext", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub additional_context: Option<Vec<Reference>>,
}


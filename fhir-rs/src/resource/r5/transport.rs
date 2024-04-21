use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Transport {
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
    /// External identifier
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Formal definition of transport
    #[fhir(name="instantiatesCanonical", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub instantiates_canonical: Option<CanonicalDt>,
    /// Formal definition of transport
    #[fhir(name="instantiatesUri", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub instantiates_uri: Option<UriDt>,
    /// Request fulfilled by this transport
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub based_on: Option<Vec<Reference>>,
    /// Requisition or grouper id
    #[fhir(name="groupIdentifier", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub group_identifier: Option<Identifier>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub part_of: Option<Vec<Reference>>,
    /// in-progress | completed | abandoned | cancelled | planned | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub status_reason: Option<CodeableConcept>,
    /// unknown | proposal | plan | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub intent: Option<CodeDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub priority: Option<CodeDt>,
    /// Transport Type
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// Human-readable explanation of transport
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub description: Option<StringDt>,
    /// What transport is acting on
    #[fhir(name="focus", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub focus: Option<Reference>,
    /// Beneficiary of the Transport
    #[fhir(name="for", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub for_: Option<Reference>,
    /// Healthcare event during which this transport originated
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub encounter: Option<Reference>,
    /// Completion time of the event (the occurrence)
    #[fhir(name="completionTime", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub completion_time: Option<DateTimeDt>,
    /// Transport Creation Date
    #[fhir(name="authoredOn", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub authored_on: Option<DateTimeDt>,
    /// Transport Last Modified Date
    #[fhir(name="lastModified", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub last_modified: Option<DateTimeDt>,
    /// Who is asking for transport to be done
    #[fhir(name="requester", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub requester: Option<Reference>,
    /// Requested performer
    #[fhir(name="performerType", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub performer_type: Option<Vec<CodeableConcept>>,
    /// Responsible individual
    #[fhir(name="owner", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub owner: Option<Reference>,
    /// Where transport occurs
    #[fhir(name="location", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub location: Option<Reference>,
    /// Associated insurance coverage
    #[fhir(name="insurance", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub insurance: Option<Vec<Reference>>,
    /// Comments made about the transport
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub note: Option<Vec<Annotation>>,
    /// Key events in history of the Transport
    #[fhir(name="relevantHistory", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub relevant_history: Option<Vec<Reference>>,
    /// Constraints on fulfillment transports
    #[fhir(name="restriction", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub restriction: Option<TransportRestrictionBackboneElement>,
    /// Information used to perform transport
    #[fhir(name="input", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub input: Option<Vec<TransportInputBackboneElement>>,
    /// Information produced as part of transport
    #[fhir(name="output", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub output: Option<Vec<TransportOutputBackboneElement>>,
    /// The desired location
    #[fhir(name="requestedLocation", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub requested_location: Option<Reference>,
    /// The entity current location
    #[fhir(name="currentLocation", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub current_location: Option<Reference>,
    /// Why transport is needed
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub reason: Option<CodeableReference>,
    /// Parent (or preceding) transport
    #[fhir(name="history", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub history: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TransportOutputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for output
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Result of output
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub value: Option<Meta>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TransportInputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for the input
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Content to use in performing the transport
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub value: Option<Meta>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TransportRestrictionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// How many times to repeat
    #[fhir(name="repetitions", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub repetitions: Option<PositiveIntDt>,
    /// When fulfillment sought
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub period: Option<Period>,
    /// For whom is fulfillment sought?
    #[fhir(name="recipient", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub recipient: Option<Vec<Reference>>,
}


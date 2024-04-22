use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DeviceRequest {
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
    /// External Request identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// What request fulfills
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// What request replaces
    #[fhir(name="replaces", min="0", max="*", summary=true, modifier=false, choice="")]
    pub replaces: Option<Vec<Reference>>,
    /// Identifier of composite request
    #[fhir(name="groupIdentifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub group_identifier: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="1", max="1", summary=true, modifier=true)]
    pub intent: Option<CodeDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary=true, modifier=false, choice="")]
    pub priority: Option<CodeDt>,
    /// True if the request is to stop or not to start using the device
    #[fhir(name="doNotPerform", min="0", max="1", summary=true, modifier=true)]
    pub do_not_perform: Option<BooleanDt>,
    /// Device requested
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableReference>,
    /// Quantity of devices to supply
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<IntegerDt>,
    /// Device details
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub parameter: Option<Vec<DeviceRequestParameterBackboneElement>>,
    /// Focus of request
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Encounter motivating request
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Desired time or schedule for use
    #[fhir(name="occurrence", min="0", max="1", summary=true, modifier=false, choice="")]
    pub occurrence: Option<Timing>,
    /// When recorded
    #[fhir(name="authoredOn", min="0", max="1", summary=true, modifier=false, choice="")]
    pub authored_on: Option<DateTimeDt>,
    /// Who/what submitted the device request
    #[fhir(name="requester", min="0", max="1", summary=true, modifier=false, choice="")]
    pub requester: Option<Reference>,
    /// Requested Filler
    #[fhir(name="performer", min="0", max="1", summary=true, modifier=false, choice="")]
    pub performer: Option<CodeableReference>,
    /// Coded/Linked Reason for request
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// PRN status of request
    #[fhir(name="asNeeded", min="0", max="1", summary=false, modifier=false, choice="")]
    pub as_needed: Option<BooleanDt>,
    /// Device usage reason
    #[fhir(name="asNeededFor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub as_needed_for: Option<CodeableConcept>,
    /// Associated insurance coverage
    #[fhir(name="insurance", min="0", max="*", summary=false, modifier=false, choice="")]
    pub insurance: Option<Vec<Reference>>,
    /// Additional clinical information
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Notes or comments
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Request provenance
    #[fhir(name="relevantHistory", min="0", max="*", summary=false, modifier=false, choice="")]
    pub relevant_history: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceRequestParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Device detail
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Value of detail
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<BooleanDt>,
}


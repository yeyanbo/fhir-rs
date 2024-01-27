use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ServiceRequest {
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
    /// Identifiers assigned to this order
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary="true", modifier="false")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary="true", modifier="false")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// What request fulfills
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// What request replaces
    #[fhir(name="replaces", min="0", max="*", summary="true", modifier="false")]
    pub replaces: Option<Vec<Reference>>,
    /// Composite Request ID
    #[fhir(name="requisition", min="0", max="1", summary="true", modifier="false")]
    pub requisition: Option<Identifier>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// proposal | plan | directive | order +
    #[fhir(name="intent", min="1", max="1", summary="true", modifier="true")]
    pub intent: Option<CodeDt>,
    /// Classification of service
    #[fhir(name="category", min="0", max="*", summary="true", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="true", modifier="false")]
    pub priority: Option<CodeDt>,
    /// True if service/procedure should not be performed
    #[fhir(name="doNotPerform", min="0", max="1", summary="true", modifier="true")]
    pub do_not_perform: Option<BooleanDt>,
    /// What is being requested/ordered
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableReference>,
    /// Additional order information
    #[fhir(name="orderDetail", min="0", max="*", summary="true", modifier="false")]
    pub order_detail: Option<Vec<ServiceRequestOrderDetailBackboneElement>>,
    /// Service amount
    #[fhir(name="quantity", min="0", max="1", summary="true", modifier="false")]
    pub quantity: Option<Range>,
    /// Individual or Entity the service is ordered for
    #[fhir(name="subject", min="1", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// What the service request is about, when it is not about the subject of record
    #[fhir(name="focus", min="0", max="*", summary="true", modifier="false")]
    pub focus: Option<Vec<Reference>>,
    /// Encounter in which the request was created
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// When service should occur
    #[fhir(name="occurrence", min="0", max="1", summary="true", modifier="false")]
    pub occurrence: Option<Timing>,
    /// Preconditions for service
    #[fhir(name="asNeeded", min="0", max="1", summary="true", modifier="false")]
    pub as_needed: Option<CodeableConcept>,
    /// Date request signed
    #[fhir(name="authoredOn", min="0", max="1", summary="true", modifier="false")]
    pub authored_on: Option<DateTimeDt>,
    /// Who/what is requesting service
    #[fhir(name="requester", min="0", max="1", summary="true", modifier="false")]
    pub requester: Option<Reference>,
    /// Performer role
    #[fhir(name="performerType", min="0", max="1", summary="true", modifier="false")]
    pub performer_type: Option<CodeableConcept>,
    /// Requested performer
    #[fhir(name="performer", min="0", max="*", summary="true", modifier="false")]
    pub performer: Option<Vec<Reference>>,
    /// Requested location
    #[fhir(name="location", min="0", max="*", summary="true", modifier="false")]
    pub location: Option<Vec<CodeableReference>>,
    /// Explanation/Justification for procedure or service
    #[fhir(name="reason", min="0", max="*", summary="true", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Associated insurance coverage
    #[fhir(name="insurance", min="0", max="*", summary="false", modifier="false")]
    pub insurance: Option<Vec<Reference>>,
    /// Additional clinical information
    #[fhir(name="supportingInfo", min="0", max="*", summary="false", modifier="false")]
    pub supporting_info: Option<Vec<CodeableReference>>,
    /// Procedure Samples
    #[fhir(name="specimen", min="0", max="*", summary="true", modifier="false")]
    pub specimen: Option<Vec<Reference>>,
    /// Coded location on Body
    #[fhir(name="bodySite", min="0", max="*", summary="true", modifier="false")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// BodyStructure-based location on the body
    #[fhir(name="bodyStructure", min="0", max="1", summary="true", modifier="false")]
    pub body_structure: Option<Reference>,
    /// Comments
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Patient or consumer-oriented instructions
    #[fhir(name="patientInstruction", min="0", max="*", summary="false", modifier="false")]
    pub patient_instruction: Option<Vec<ServiceRequestPatientInstructionBackboneElement>>,
    /// Request provenance
    #[fhir(name="relevantHistory", min="0", max="*", summary="false", modifier="false")]
    pub relevant_history: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ServiceRequestOrderDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The context of the order details by reference
    #[fhir(name="parameterFocus", min="0", max="1", summary="false", modifier="false")]
    pub parameter_focus: Option<CodeableReference>,
    /// The parameter details for the service being requested
    #[fhir(name="parameter", min="1", max="*", summary="true", modifier="false")]
    pub parameter: Option<Vec<ServiceRequestOrderDetailParameterBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ServiceRequestOrderDetailParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The detail of the order being requested
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// The value for the order detail
    #[fhir(name="value", min="1", max="1", summary="true", modifier="false")]
    pub value: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ServiceRequestPatientInstructionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Patient or consumer-oriented instructions
    #[fhir(name="instruction", min="0", max="1", summary="true", modifier="false")]
    pub instruction: Option<Reference>,
}


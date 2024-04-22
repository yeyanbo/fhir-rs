use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Procedure {
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
    /// External Identifiers for this procedure
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// A request for this procedure
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | not-done | on-hold | stopped | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary=true, modifier=false, choice="")]
    pub status_reason: Option<CodeableConcept>,
    /// Classification of the procedure
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Identification of the procedure
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Individual or entity the procedure was performed on
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Who is the target of the procedure when it is not the subject of record only
    #[fhir(name="focus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub focus: Option<Reference>,
    /// The Encounter during which this Procedure was created
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// When the procedure occurred or is occurring
    #[fhir(name="occurrence", min="0", max="1", summary=true, modifier=false, choice="")]
    pub occurrence: Option<Timing>,
    /// When the procedure was first captured in the subject's record
    #[fhir(name="recorded", min="0", max="1", summary=true, modifier=false, choice="")]
    pub recorded: Option<DateTimeDt>,
    /// Who recorded the procedure
    #[fhir(name="recorder", min="0", max="1", summary=true, modifier=false, choice="")]
    pub recorder: Option<Reference>,
    /// Reported rather than primary record
    #[fhir(name="reported", min="0", max="1", summary=true, modifier=false, choice="")]
    pub reported: Option<Reference>,
    /// Who performed the procedure and what they did
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false, choice="")]
    pub performer: Option<Vec<ProcedurePerformerBackboneElement>>,
    /// Where the procedure happened
    #[fhir(name="location", min="0", max="1", summary=true, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// The justification that the procedure was performed
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Target body sites
    #[fhir(name="bodySite", min="0", max="*", summary=true, modifier=false, choice="")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// The result of procedure
    #[fhir(name="outcome", min="0", max="1", summary=true, modifier=false, choice="")]
    pub outcome: Option<CodeableConcept>,
    /// Any report resulting from the procedure
    #[fhir(name="report", min="0", max="*", summary=false, modifier=false, choice="")]
    pub report: Option<Vec<Reference>>,
    /// Complication following the procedure
    #[fhir(name="complication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub complication: Option<Vec<CodeableReference>>,
    /// Instructions for follow up
    #[fhir(name="followUp", min="0", max="*", summary=false, modifier=false, choice="")]
    pub follow_up: Option<Vec<CodeableConcept>>,
    /// Additional information about the procedure
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Manipulated, implanted, or removed device
    #[fhir(name="focalDevice", min="0", max="*", summary=false, modifier=false, choice="")]
    pub focal_device: Option<Vec<ProcedureFocalDeviceBackboneElement>>,
    /// Items used during procedure
    #[fhir(name="used", min="0", max="*", summary=false, modifier=false, choice="")]
    pub used: Option<Vec<CodeableReference>>,
    /// Extra information relevant to the procedure
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_info: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ProcedureFocalDeviceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Kind of change to device
    #[fhir(name="action", min="0", max="1", summary=false, modifier=false, choice="")]
    pub action: Option<CodeableConcept>,
    /// Device that was changed
    #[fhir(name="manipulated", min="1", max="1", summary=false, modifier=false, choice="")]
    pub manipulated: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ProcedurePerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of performance
    #[fhir(name="function", min="0", max="1", summary=true, modifier=false, choice="")]
    pub function: Option<CodeableConcept>,
    /// Who performed the procedure
    #[fhir(name="actor", min="1", max="1", summary=true, modifier=false, choice="")]
    pub actor: Option<Reference>,
    /// Organization the device or practitioner was acting for
    #[fhir(name="onBehalfOf", min="0", max="1", summary=false, modifier=false, choice="")]
    pub on_behalf_of: Option<Reference>,
    /// When the performer performed the procedure
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
}


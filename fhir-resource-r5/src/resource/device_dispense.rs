use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DeviceDispense {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business identifier for this dispensation
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// The order or request that this dispense is fulfilling
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false)]
    pub based_on: Option<Vec<Reference>>,
    /// The bigger event that this dispense is a part of
    #[fhir(name="partOf", min="0", max="*", summary=false, modifier=false)]
    pub part_of: Option<Vec<Reference>>,
    /// preparation | in-progress | cancelled | on-hold | completed | entered-in-error | stopped | declined | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Why a dispense was or was not performed
    #[fhir(name="statusReason", min="0", max="1", summary=false, modifier=false)]
    pub status_reason: Option<CodeableReference>,
    /// Type of device dispense
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// What device was supplied
    #[fhir(name="device", min="1", max="1", summary=true, modifier=false)]
    pub device: Option<CodeableReference>,
    /// Who the dispense is for
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// Who collected the device or where the medication was delivered
    #[fhir(name="receiver", min="0", max="1", summary=false, modifier=false)]
    pub receiver: Option<Reference>,
    /// Encounter associated with event
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false)]
    pub encounter: Option<Reference>,
    /// Information that supports the dispensing of the device
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false)]
    pub supporting_information: Option<Vec<Reference>>,
    /// Who performed event
    #[fhir(name="performer", min="0", max="*", summary=false, modifier=false)]
    pub performer: Option<Vec<DeviceDispensePerformerBackboneElement>>,
    /// Where the dispense occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false)]
    pub location: Option<Reference>,
    /// Trial fill, partial fill, emergency fill, etc
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Amount dispensed
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false)]
    pub quantity: Option<Quantity>,
    /// When product was packaged and reviewed
    #[fhir(name="preparedDate", min="0", max="1", summary=true, modifier=false)]
    pub prepared_date: Option<DateTimeDt>,
    /// When product was given out
    #[fhir(name="whenHandedOver", min="0", max="1", summary=false, modifier=false)]
    pub when_handed_over: Option<DateTimeDt>,
    /// Where the device was sent or should be sent
    #[fhir(name="destination", min="0", max="1", summary=false, modifier=false)]
    pub destination: Option<Reference>,
    /// Information about the dispense
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Full representation of the usage instructions
    #[fhir(name="usageInstruction", min="0", max="1", summary=false, modifier=false)]
    pub usage_instruction: Option<MarkdownDt>,
    /// A list of relevant lifecycle events
    #[fhir(name="eventHistory", min="0", max="*", summary=false, modifier=false)]
    pub event_history: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDispensePerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Who performed the dispense and what they did
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false)]
    pub function: Option<CodeableConcept>,
    /// Individual who was performing
    #[fhir(name="actor", min="1", max="1", summary=false, modifier=false)]
    pub actor: Option<Reference>,
}


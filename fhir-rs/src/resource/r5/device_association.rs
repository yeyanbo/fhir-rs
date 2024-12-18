use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DeviceAssociation {
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
    /// Instance identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Reference to the devices associated with the patient or group
    #[fhir(name="device", min="1", max="1", summary=true, modifier=false, choice="")]
    pub device: Option<Reference>,
    /// Describes the relationship between the device and subject
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// implanted | explanted | attached | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=false, choice="")]
    pub status: Option<CodeableConcept>,
    /// The reasons given for the current association status
    #[fhir(name="statusReason", min="0", max="*", summary=true, modifier=false, choice="")]
    pub status_reason: Option<Vec<CodeableConcept>>,
    /// The individual, group of individuals or device that the device is on or associated with
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Current anatomical location of the device in/on subject
    #[fhir(name="bodyStructure", min="0", max="1", summary=true, modifier=false, choice="")]
    pub body_structure: Option<Reference>,
    /// Begin and end dates and times for the device association
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// The details about the device when it is in use to describe its operation
    #[fhir(name="operation", min="0", max="*", summary=true, modifier=false, choice="")]
    pub operation: Option<Vec<DeviceAssociationOperationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct DeviceAssociationOperationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Device operational condition
    #[fhir(name="status", min="1", max="1", summary=true, modifier=false, choice="")]
    pub status: Option<CodeableConcept>,
    /// The individual performing the action enabled by the device
    #[fhir(name="operator", min="0", max="*", summary=true, modifier=false, choice="")]
    pub operator: Option<Vec<Reference>>,
    /// Begin and end dates and times for the device's operation
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
}


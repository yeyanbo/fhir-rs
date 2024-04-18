use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Device {
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
    /// Instance identifier
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// The name used to display by default when the device is referenced
    #[fhir(name="displayName", min="0", max="1", summary=false, modifier=false)]
    pub display_name: Option<StringDt>,
    /// The reference to the definition for the device
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false)]
    pub definition: Option<CodeableReference>,
    /// Unique Device Identifier (UDI) Barcode string
    #[fhir(name="udiCarrier", min="0", max="*", summary=true, modifier=false)]
    pub udi_carrier: Option<Vec<DeviceUdiCarrierBackboneElement>>,
    /// active | inactive | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// lost | damaged | destroyed | available
    #[fhir(name="availabilityStatus", min="0", max="1", summary=false, modifier=false)]
    pub availability_status: Option<CodeableConcept>,
    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[fhir(name="biologicalSourceEvent", min="0", max="1", summary=false, modifier=false)]
    pub biological_source_event: Option<Identifier>,
    /// Name of device manufacturer
    #[fhir(name="manufacturer", min="0", max="1", summary=false, modifier=false)]
    pub manufacturer: Option<StringDt>,
    /// Date when the device was made
    #[fhir(name="manufactureDate", min="0", max="1", summary=false, modifier=false)]
    pub manufacture_date: Option<DateTimeDt>,
    /// Date and time of expiry of this device (if applicable)
    #[fhir(name="expirationDate", min="0", max="1", summary=false, modifier=false)]
    pub expiration_date: Option<DateTimeDt>,
    /// Lot number of manufacture
    #[fhir(name="lotNumber", min="0", max="1", summary=false, modifier=false)]
    pub lot_number: Option<StringDt>,
    /// Serial number assigned by the manufacturer
    #[fhir(name="serialNumber", min="0", max="1", summary=false, modifier=false)]
    pub serial_number: Option<StringDt>,
    /// The name or names of the device as known to the manufacturer and/or patient
    #[fhir(name="name", min="0", max="*", summary=false, modifier=false)]
    pub name: Option<Vec<DeviceNameBackboneElement>>,
    /// The manufacturer's model number for the device
    #[fhir(name="modelNumber", min="0", max="1", summary=false, modifier=false)]
    pub model_number: Option<StringDt>,
    /// The part number or catalog number of the device
    #[fhir(name="partNumber", min="0", max="1", summary=false, modifier=false)]
    pub part_number: Option<StringDt>,
    /// Indicates a high-level grouping of the device
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// The kind or type of device
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The actual design of the device or software version running on the device
    #[fhir(name="version", min="0", max="*", summary=false, modifier=false)]
    pub version: Option<Vec<DeviceVersionBackboneElement>>,
    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[fhir(name="conformsTo", min="0", max="*", summary=false, modifier=false)]
    pub conforms_to: Option<Vec<DeviceConformsToBackboneElement>>,
    /// Inherent, essentially fixed, characteristics of the device.  e.g., time properties, size, material, etc.
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false)]
    pub property: Option<Vec<DevicePropertyBackboneElement>>,
    /// The designated condition for performing a task
    #[fhir(name="mode", min="0", max="1", summary=false, modifier=false)]
    pub mode: Option<CodeableConcept>,
    /// The series of occurrences that repeats during the operation of the device
    #[fhir(name="cycle", min="0", max="1", summary=false, modifier=false)]
    pub cycle: Option<Count>,
    /// A measurement of time during the device's operation (e.g., days, hours, mins, etc.)
    #[fhir(name="duration", min="0", max="1", summary=false, modifier=false)]
    pub duration: Option<Duration>,
    /// Organization responsible for device
    #[fhir(name="owner", min="0", max="1", summary=false, modifier=false)]
    pub owner: Option<Reference>,
    /// Details for human/organization for support
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false)]
    pub contact: Option<Vec<ContactPoint>>,
    /// Where the device is found
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false)]
    pub location: Option<Reference>,
    /// Network address to contact device
    #[fhir(name="url", min="0", max="1", summary=false, modifier=false)]
    pub url: Option<UriDt>,
    /// Technical endpoints providing access to electronic services provided by the device
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false)]
    pub endpoint: Option<Vec<Reference>>,
    /// Linked device acting as a communication/data collector, translator or controller
    #[fhir(name="gateway", min="0", max="*", summary=false, modifier=false)]
    pub gateway: Option<Vec<CodeableReference>>,
    /// Device notes and comments
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Safety Characteristics of Device
    #[fhir(name="safety", min="0", max="*", summary=true, modifier=false)]
    pub safety: Option<Vec<CodeableConcept>>,
    /// The higher level or encompassing device that this device is a logical part of
    #[fhir(name="parent", min="0", max="1", summary=false, modifier=false)]
    pub parent: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DevicePropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that specifies the property being represented
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Value of the property
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false)]
    pub value: Option<Attachment>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceUdiCarrierBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Mandatory fixed portion of UDI
    #[fhir(name="deviceIdentifier", min="1", max="1", summary=true, modifier=false)]
    pub device_identifier: Option<StringDt>,
    /// UDI Issuing Organization
    #[fhir(name="issuer", min="1", max="1", summary=true, modifier=false)]
    pub issuer: Option<UriDt>,
    /// Regional UDI authority
    #[fhir(name="jurisdiction", min="0", max="1", summary=false, modifier=false)]
    pub jurisdiction: Option<UriDt>,
    /// UDI Machine Readable Barcode String
    #[fhir(name="carrierAIDC", min="0", max="1", summary=true, modifier=false)]
    pub carrier_aidc: Option<Base64BinaryDt>,
    /// UDI Human Readable Barcode String
    #[fhir(name="carrierHRF", min="0", max="1", summary=true, modifier=false)]
    pub carrier_hrf: Option<StringDt>,
    /// barcode | rfid | manual | card | self-reported | electronic-transmission | unknown
    #[fhir(name="entryType", min="0", max="1", summary=false, modifier=false)]
    pub entry_type: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceConformsToBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Describes the common type of the standard, specification, or formal guidance.  communication | performance | measurement
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false)]
    pub category: Option<CodeableConcept>,
    /// Identifies the standard, specification, or formal guidance that the device adheres to
    #[fhir(name="specification", min="1", max="1", summary=false, modifier=false)]
    pub specification: Option<CodeableConcept>,
    /// Specific form or variant of the standard
    #[fhir(name="version", min="0", max="1", summary=false, modifier=false)]
    pub version: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceNameBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The term that names the device
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false)]
    pub value: Option<StringDt>,
    /// registered-name | user-friendly-name | patient-reported-name
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// The preferred device name
    #[fhir(name="display", min="0", max="1", summary=true, modifier=true)]
    pub display: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceVersionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of the device version, e.g. manufacturer, approved, internal
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The hardware or software module of the device to which the version applies
    #[fhir(name="component", min="0", max="1", summary=false, modifier=false)]
    pub component: Option<Identifier>,
    /// The date the version was installed on the device
    #[fhir(name="installDate", min="0", max="1", summary=false, modifier=false)]
    pub install_date: Option<DateTimeDt>,
    /// The version text
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false)]
    pub value: Option<StringDt>,
}


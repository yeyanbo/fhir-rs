use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DeviceDefinition {
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
    /// Additional information to describe the device
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Instance identifier
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Unique Device Identifier (UDI) Barcode string
    #[fhir(name="udiDeviceIdentifier", min="0", max="*", summary="false", modifier="false")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifierBackboneElement>>,
    /// Regulatory identifier(s) associated with this device
    #[fhir(name="regulatoryIdentifier", min="0", max="*", summary="false", modifier="false")]
    pub regulatory_identifier: Option<Vec<DeviceDefinitionRegulatoryIdentifierBackboneElement>>,
    /// The part number or catalog number of the device
    #[fhir(name="partNumber", min="0", max="1", summary="false", modifier="false")]
    pub part_number: Option<StringDt>,
    /// Name of device manufacturer
    #[fhir(name="manufacturer", min="0", max="1", summary="true", modifier="false")]
    pub manufacturer: Option<Reference>,
    /// The name or names of the device as given by the manufacturer
    #[fhir(name="deviceName", min="0", max="*", summary="true", modifier="false")]
    pub device_name: Option<Vec<DeviceDefinitionDeviceNameBackboneElement>>,
    /// The catalog or model number for the device for example as defined by the manufacturer
    #[fhir(name="modelNumber", min="0", max="1", summary="true", modifier="false")]
    pub model_number: Option<StringDt>,
    /// What kind of device or device system this is
    #[fhir(name="classification", min="0", max="*", summary="true", modifier="false")]
    pub classification: Option<Vec<DeviceDefinitionClassificationBackboneElement>>,
    /// Identifies the standards, specifications, or formal guidances for the capabilities supported by the device
    #[fhir(name="conformsTo", min="0", max="*", summary="true", modifier="false")]
    pub conforms_to: Option<Vec<DeviceDefinitionConformsToBackboneElement>>,
    /// A device, part of the current one
    #[fhir(name="hasPart", min="0", max="*", summary="true", modifier="false")]
    pub has_part: Option<Vec<DeviceDefinitionHasPartBackboneElement>>,
    /// Information about the packaging of the device, i.e. how the device is packaged
    #[fhir(name="packaging", min="0", max="*", summary="false", modifier="false")]
    pub packaging: Option<Vec<DeviceDefinitionPackagingBackboneElement>>,
    /// The version of the device or software
    #[fhir(name="version", min="0", max="*", summary="false", modifier="false")]
    pub version: Option<Vec<DeviceDefinitionVersionBackboneElement>>,
    /// Safety characteristics of the device
    #[fhir(name="safety", min="0", max="*", summary="true", modifier="false")]
    pub safety: Option<Vec<CodeableConcept>>,
    /// Shelf Life and storage information
    #[fhir(name="shelfLifeStorage", min="0", max="*", summary="false", modifier="false")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,
    /// Language code for the human-readable text strings produced by the device (all supported)
    #[fhir(name="languageCode", min="0", max="*", summary="false", modifier="false")]
    pub language_code: Option<Vec<CodeableConcept>>,
    /// Inherent, essentially fixed, characteristics of this kind of device, e.g., time properties, size, etc
    #[fhir(name="property", min="0", max="*", summary="false", modifier="false")]
    pub property: Option<Vec<DeviceDefinitionPropertyBackboneElement>>,
    /// Organization responsible for device
    #[fhir(name="owner", min="0", max="1", summary="false", modifier="false")]
    pub owner: Option<Reference>,
    /// Details for human/organization for support
    #[fhir(name="contact", min="0", max="*", summary="false", modifier="false")]
    pub contact: Option<Vec<ContactPoint>>,
    /// An associated device, attached to, used with, communicating with or linking a previous or new device model to the focal device
    #[fhir(name="link", min="0", max="*", summary="false", modifier="false")]
    pub link: Option<Vec<DeviceDefinitionLinkBackboneElement>>,
    /// Device notes and comments
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// A substance used to create the material(s) of which the device is made
    #[fhir(name="material", min="0", max="*", summary="false", modifier="false")]
    pub material: Option<Vec<DeviceDefinitionMaterialBackboneElement>>,
    /// lot-number | manufactured-date | serial-number | expiration-date | biological-source | software-version
    #[fhir(name="productionIdentifierInUDI", min="0", max="*", summary="false", modifier="false")]
    pub production_identifier_in_udi: Option<Vec<CodeDt>>,
    /// Information aimed at providing directions for the usage of this model of device
    #[fhir(name="guideline", min="0", max="1", summary="false", modifier="false")]
    pub guideline: Option<DeviceDefinitionGuidelineBackboneElement>,
    /// Tracking of latest field safety corrective action
    #[fhir(name="correctiveAction", min="0", max="1", summary="false", modifier="false")]
    pub corrective_action: Option<DeviceDefinitionCorrectiveActionBackboneElement>,
    /// Billing code or reference associated with the device
    #[fhir(name="chargeItem", min="0", max="*", summary="false", modifier="false")]
    pub charge_item: Option<Vec<DeviceDefinitionChargeItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionUdiDeviceIdentifierBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The identifier that is to be associated with every Device that references this DeviceDefintiion for the issuer and jurisdiction provided in the DeviceDefinition.udiDeviceIdentifier
    #[fhir(name="deviceIdentifier", min="1", max="1", summary="false", modifier="false")]
    pub device_identifier: Option<StringDt>,
    /// The organization that assigns the identifier algorithm
    #[fhir(name="issuer", min="1", max="1", summary="false", modifier="false")]
    pub issuer: Option<UriDt>,
    /// The jurisdiction to which the deviceIdentifier applies
    #[fhir(name="jurisdiction", min="1", max="1", summary="false", modifier="false")]
    pub jurisdiction: Option<UriDt>,
    /// Indicates whether and when the device is available on the market
    #[fhir(name="marketDistribution", min="0", max="*", summary="false", modifier="false")]
    pub market_distribution: Option<Vec<DeviceDefinitionUdiDeviceIdentifierMarketDistributionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionUdiDeviceIdentifierMarketDistributionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Begin and end dates for the commercial distribution of the device
    #[fhir(name="marketPeriod", min="1", max="1", summary="false", modifier="false")]
    pub market_period: Option<Period>,
    /// National state or territory where the device is commercialized
    #[fhir(name="subJurisdiction", min="1", max="1", summary="false", modifier="false")]
    pub sub_jurisdiction: Option<UriDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionDeviceNameBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A name that is used to refer to the device
    #[fhir(name="name", min="1", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// registered-name | user-friendly-name | patient-reported-name
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionClassificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A classification or risk class of the device model
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Further information qualifying this classification of the device model
    #[fhir(name="justification", min="0", max="*", summary="false", modifier="false")]
    pub justification: Option<Vec<RelatedArtifact>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionHasPartBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to the part
    #[fhir(name="reference", min="1", max="1", summary="true", modifier="false")]
    pub reference: Option<Reference>,
    /// Number of occurrences of the part
    #[fhir(name="count", min="0", max="1", summary="false", modifier="false")]
    pub count: Option<IntegerDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionChargeItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The code or reference for the charge item
    #[fhir(name="chargeItemCode", min="1", max="1", summary="false", modifier="false")]
    pub charge_item_code: Option<CodeableReference>,
    /// Coefficient applicable to the billing code
    #[fhir(name="count", min="1", max="1", summary="false", modifier="false")]
    pub count: Option<Quantity>,
    /// A specific time period in which this charge item applies
    #[fhir(name="effectivePeriod", min="0", max="1", summary="false", modifier="false")]
    pub effective_period: Option<Period>,
    /// The context to which this charge item applies
    #[fhir(name="useContext", min="0", max="*", summary="false", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionConformsToBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Describes the common type of the standard, specification, or formal guidance
    #[fhir(name="category", min="0", max="1", summary="true", modifier="false")]
    pub category: Option<CodeableConcept>,
    /// Identifies the standard, specification, or formal guidance that the device adheres to the Device Specification type
    #[fhir(name="specification", min="1", max="1", summary="true", modifier="false")]
    pub specification: Option<CodeableConcept>,
    /// The specific form or variant of the standard, specification or formal guidance
    #[fhir(name="version", min="0", max="*", summary="true", modifier="false")]
    pub version: Option<Vec<StringDt>>,
    /// Standard, regulation, certification, or guidance website, document, or other publication, or similar, supporting the conformance
    #[fhir(name="source", min="0", max="*", summary="false", modifier="false")]
    pub source: Option<Vec<RelatedArtifact>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionRegulatoryIdentifierBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// basic | master | license
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeDt>,
    /// The identifier itself
    #[fhir(name="deviceIdentifier", min="1", max="1", summary="false", modifier="false")]
    pub device_identifier: Option<StringDt>,
    /// The organization that issued this identifier
    #[fhir(name="issuer", min="1", max="1", summary="false", modifier="false")]
    pub issuer: Option<UriDt>,
    /// The jurisdiction to which the deviceIdentifier applies
    #[fhir(name="jurisdiction", min="1", max="1", summary="false", modifier="false")]
    pub jurisdiction: Option<UriDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionPackagingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business identifier of the packaged medication
    #[fhir(name="identifier", min="0", max="1", summary="false", modifier="false")]
    pub identifier: Option<Identifier>,
    /// A code that defines the specific type of packaging
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The number of items contained in the package (devices or sub-packages)
    #[fhir(name="count", min="0", max="1", summary="false", modifier="false")]
    pub count: Option<IntegerDt>,
    /// An organization that distributes the packaged device
    #[fhir(name="distributor", min="0", max="*", summary="false", modifier="false")]
    pub distributor: Option<Vec<DeviceDefinitionPackagingDistributorBackboneElement>>,
    /// Unique Device Identifier (UDI) Barcode string on the packaging
    #[fhir(name="udiDeviceIdentifier", min="0", max="*", summary="false", modifier="false")]
    pub udi_device_identifier: Option<Vec<DeviceDefinitionUdiDeviceIdentifierBackboneElement>>,
    /// Allows packages within packages
    #[fhir(name="packaging", min="0", max="*", summary="false", modifier="false")]
    pub packaging: Option<Vec<DeviceDefinitionPackagingBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionPackagingDistributorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Distributor's human-readable name
    #[fhir(name="name", min="0", max="1", summary="false", modifier="false")]
    pub name: Option<StringDt>,
    /// Distributor as an Organization resource
    #[fhir(name="organizationReference", min="0", max="*", summary="false", modifier="false")]
    pub organization_reference: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionLinkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type indicates the relationship of the related device to the device instance
    #[fhir(name="relation", min="1", max="1", summary="false", modifier="false")]
    pub relation: Option<Coding>,
    /// A reference to the linked device
    #[fhir(name="relatedDevice", min="1", max="1", summary="false", modifier="false")]
    pub related_device: Option<CodeableReference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionMaterialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A relevant substance that the device contains, may contain, or is made of
    #[fhir(name="substance", min="1", max="1", summary="false", modifier="false")]
    pub substance: Option<CodeableConcept>,
    /// Indicates an alternative material of the device
    #[fhir(name="alternate", min="0", max="1", summary="false", modifier="false")]
    pub alternate: Option<BooleanDt>,
    /// Whether the substance is a known or suspected allergen
    #[fhir(name="allergenicIndicator", min="0", max="1", summary="false", modifier="false")]
    pub allergenic_indicator: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that specifies the property being represented
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Value of the property
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<Attachment>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionVersionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of the device version, e.g. manufacturer, approved, internal
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The hardware or software module of the device to which the version applies
    #[fhir(name="component", min="0", max="1", summary="false", modifier="false")]
    pub component: Option<Identifier>,
    /// The version text
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionGuidelineBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The circumstances that form the setting for using the device
    #[fhir(name="useContext", min="0", max="*", summary="false", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Detailed written and visual directions for the user on how to use the device
    #[fhir(name="usageInstruction", min="0", max="1", summary="false", modifier="false")]
    pub usage_instruction: Option<MarkdownDt>,
    /// A source of information or reference for this guideline
    #[fhir(name="relatedArtifact", min="0", max="*", summary="false", modifier="false")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// A clinical condition for which the device was designed to be used
    #[fhir(name="indication", min="0", max="*", summary="false", modifier="false")]
    pub indication: Option<Vec<CodeableConcept>>,
    /// A specific situation when a device should not be used because it may cause harm
    #[fhir(name="contraindication", min="0", max="*", summary="false", modifier="false")]
    pub contraindication: Option<Vec<CodeableConcept>>,
    /// Specific hazard alert information that a user needs to know before using the device
    #[fhir(name="warning", min="0", max="*", summary="false", modifier="false")]
    pub warning: Option<Vec<CodeableConcept>>,
    /// A description of the general purpose or medical use of the device or its function
    #[fhir(name="intendedUse", min="0", max="1", summary="false", modifier="false")]
    pub intended_use: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceDefinitionCorrectiveActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether the corrective action was a recall
    #[fhir(name="recall", min="1", max="1", summary="false", modifier="false")]
    pub recall: Option<BooleanDt>,
    /// model | lot-numbers | serial-numbers
    #[fhir(name="scope", min="0", max="1", summary="false", modifier="false")]
    pub scope: Option<CodeDt>,
    /// Start and end dates of the  corrective action
    #[fhir(name="period", min="1", max="1", summary="false", modifier="false")]
    pub period: Option<Period>,
}


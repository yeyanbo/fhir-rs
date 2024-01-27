use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct DeviceUsage {
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
    /// External identifier for this record
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan, proposal or order
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// active | completed | not-done | entered-in-error +
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// The category of the statement - classifying how the statement is made
    #[fhir(name="category", min="0", max="*", summary="false", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Patient using device
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
    /// Supporting information
    #[fhir(name="derivedFrom", min="0", max="*", summary="true", modifier="false")]
    pub derived_from: Option<Vec<Reference>>,
    /// The encounter or episode of care that establishes the context for this device use statement
    #[fhir(name="context", min="0", max="1", summary="true", modifier="false")]
    pub context: Option<Reference>,
    /// How often  the device was used
    #[fhir(name="timing", min="0", max="1", summary="true", modifier="false")]
    pub timing: Option<DateTimeDt>,
    /// When the statement was made (and recorded)
    #[fhir(name="dateAsserted", min="0", max="1", summary="true", modifier="false")]
    pub date_asserted: Option<DateTimeDt>,
    /// The status of the device usage, for example always, sometimes, never. This is not the same as the status of the statement
    #[fhir(name="usageStatus", min="0", max="1", summary="false", modifier="false")]
    pub usage_status: Option<CodeableConcept>,
    /// The reason for asserting the usage status - for example forgot, lost, stolen, broken
    #[fhir(name="usageReason", min="0", max="*", summary="false", modifier="false")]
    pub usage_reason: Option<Vec<CodeableConcept>>,
    /// How device is being used
    #[fhir(name="adherence", min="0", max="1", summary="false", modifier="false")]
    pub adherence: Option<DeviceUsageAdherenceBackboneElement>,
    /// Who made the statement
    #[fhir(name="informationSource", min="0", max="1", summary="true", modifier="false")]
    pub information_source: Option<Reference>,
    /// Code or Reference to device used
    #[fhir(name="device", min="1", max="1", summary="true", modifier="false")]
    pub device: Option<CodeableReference>,
    /// Why device was used
    #[fhir(name="reason", min="0", max="*", summary="true", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Target body site
    #[fhir(name="bodySite", min="0", max="1", summary="true", modifier="false")]
    pub body_site: Option<CodeableReference>,
    /// Addition details (comments, instructions)
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DeviceUsageAdherenceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// always | never | sometimes
    #[fhir(name="code", min="1", max="1", summary="false", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// lost | stolen | prescribed | broken | burned | forgot
    #[fhir(name="reason", min="1", max="*", summary="false", modifier="false")]
    pub reason: Option<Vec<CodeableConcept>>,
}


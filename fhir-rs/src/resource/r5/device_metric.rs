use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DeviceMetric {
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
    /// Identity of metric, for example Heart Rate or PEEP Setting
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Unit of Measure for the Metric
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false, choice="")]
    pub unit: Option<CodeableConcept>,
    /// Describes the link to the Device
    #[fhir(name="device", min="1", max="1", summary=true, modifier=false, choice="")]
    pub device: Option<Reference>,
    /// on | off | standby | entered-in-error
    #[fhir(name="operationalStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub operational_status: Option<CodeDt>,
    /// Color name (from CSS4) or #RRGGBB code
    #[fhir(name="color", min="0", max="1", summary=false, modifier=false, choice="")]
    pub color: Option<CodeDt>,
    /// measurement | setting | calculation | unspecified
    #[fhir(name="category", min="1", max="1", summary=true, modifier=false, choice="")]
    pub category: Option<CodeDt>,
    /// Indicates how often the metric is taken or recorded
    #[fhir(name="measurementFrequency", min="0", max="1", summary=false, modifier=false, choice="")]
    pub measurement_frequency: Option<Quantity>,
    /// Describes the calibrations that have been performed or that are required to be performed
    #[fhir(name="calibration", min="0", max="*", summary=false, modifier=false, choice="")]
    pub calibration: Option<Vec<DeviceMetricCalibrationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct DeviceMetricCalibrationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// unspecified | offset | gain | two-point
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// not-calibrated | calibration-required | calibrated | unspecified
    #[fhir(name="state", min="0", max="1", summary=false, modifier=false, choice="")]
    pub state: Option<CodeDt>,
    /// Describes the time last calibration has been performed
    #[fhir(name="time", min="0", max="1", summary=false, modifier=false, choice="")]
    pub time: Option<InstantDt>,
}


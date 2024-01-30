use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ImagingStudy {
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
    /// Identifiers for the whole study
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// registered | available | cancelled | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// All of the distinct values for series' modalities
    #[fhir(name="modality", min="0", max="*", summary=true, modifier=false)]
    pub modality: Option<Vec<CodeableConcept>>,
    /// Who or what is the subject of the study
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// Encounter with which this imaging study is associated
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false)]
    pub encounter: Option<Reference>,
    /// When the study was started
    #[fhir(name="started", min="0", max="1", summary=true, modifier=false)]
    pub started: Option<DateTimeDt>,
    /// Request fulfilled
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false)]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false)]
    pub part_of: Option<Vec<Reference>>,
    /// Referring physician
    #[fhir(name="referrer", min="0", max="1", summary=true, modifier=false)]
    pub referrer: Option<Reference>,
    /// Study access endpoint
    #[fhir(name="endpoint", min="0", max="*", summary=true, modifier=false)]
    pub endpoint: Option<Vec<Reference>>,
    /// Number of Study Related Series
    #[fhir(name="numberOfSeries", min="0", max="1", summary=true, modifier=false)]
    pub number_of_series: Option<UnsignedIntDt>,
    /// Number of Study Related Instances
    #[fhir(name="numberOfInstances", min="0", max="1", summary=true, modifier=false)]
    pub number_of_instances: Option<UnsignedIntDt>,
    /// The performed procedure or code
    #[fhir(name="procedure", min="0", max="*", summary=true, modifier=false)]
    pub procedure: Option<Vec<CodeableReference>>,
    /// Where ImagingStudy occurred
    #[fhir(name="location", min="0", max="1", summary=true, modifier=false)]
    pub location: Option<Reference>,
    /// Why the study was requested / performed
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// User-defined comments
    #[fhir(name="note", min="0", max="*", summary=true, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Institution-generated description
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<StringDt>,
    /// Each study has one or more series of instances
    #[fhir(name="series", min="0", max="*", summary=true, modifier=false)]
    pub series: Option<Vec<ImagingStudySeriesBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImagingStudySeriesBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// DICOM Series Instance UID for the series
    #[fhir(name="uid", min="1", max="1", summary=true, modifier=false)]
    pub uid: Option<IdDt>,
    /// Numeric identifier of this series
    #[fhir(name="number", min="0", max="1", summary=true, modifier=false)]
    pub number: Option<UnsignedIntDt>,
    /// The modality used for this series
    #[fhir(name="modality", min="1", max="1", summary=true, modifier=false)]
    pub modality: Option<CodeableConcept>,
    /// A short human readable summary of the series
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<StringDt>,
    /// Number of Series Related Instances
    #[fhir(name="numberOfInstances", min="0", max="1", summary=true, modifier=false)]
    pub number_of_instances: Option<UnsignedIntDt>,
    /// Series access endpoint
    #[fhir(name="endpoint", min="0", max="*", summary=true, modifier=false)]
    pub endpoint: Option<Vec<Reference>>,
    /// Body part examined
    #[fhir(name="bodySite", min="0", max="1", summary=true, modifier=false)]
    pub body_site: Option<CodeableReference>,
    /// Body part laterality
    #[fhir(name="laterality", min="0", max="1", summary=true, modifier=false)]
    pub laterality: Option<CodeableConcept>,
    /// Specimen imaged
    #[fhir(name="specimen", min="0", max="*", summary=true, modifier=false)]
    pub specimen: Option<Vec<Reference>>,
    /// When the series started
    #[fhir(name="started", min="0", max="1", summary=true, modifier=false)]
    pub started: Option<DateTimeDt>,
    /// Who performed the series
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false)]
    pub performer: Option<Vec<ImagingStudySeriesPerformerBackboneElement>>,
    /// A single SOP instance from the series
    #[fhir(name="instance", min="0", max="*", summary=false, modifier=false)]
    pub instance: Option<Vec<ImagingStudySeriesInstanceBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImagingStudySeriesPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of performance
    #[fhir(name="function", min="0", max="1", summary=true, modifier=false)]
    pub function: Option<CodeableConcept>,
    /// Who performed the series
    #[fhir(name="actor", min="1", max="1", summary=true, modifier=false)]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImagingStudySeriesInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// DICOM SOP Instance UID
    #[fhir(name="uid", min="1", max="1", summary=false, modifier=false)]
    pub uid: Option<IdDt>,
    /// DICOM class type
    #[fhir(name="sopClass", min="1", max="1", summary=false, modifier=false)]
    pub sop_class: Option<Coding>,
    /// The number of this instance in the series
    #[fhir(name="number", min="0", max="1", summary=false, modifier=false)]
    pub number: Option<UnsignedIntDt>,
    /// Description of instance
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false)]
    pub title: Option<StringDt>,
}


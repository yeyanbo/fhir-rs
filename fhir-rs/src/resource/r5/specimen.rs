use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Specimen {
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
    /// External Identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Identifier assigned by the lab
    #[fhir(name="accessionIdentifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub accession_identifier: Option<Identifier>,
    /// available | unavailable | unsatisfactory | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Kind of material that forms the specimen
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Where the specimen came from. This may be from patient(s), from a location (e.g., the source of an environmental sample), or a sampling of a substance, a biologically-derived product, or a device
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// The time when specimen is received by the testing laboratory
    #[fhir(name="receivedTime", min="0", max="1", summary=true, modifier=false, choice="")]
    pub received_time: Option<DateTimeDt>,
    /// Specimen from which this specimen originated
    #[fhir(name="parent", min="0", max="*", summary=false, modifier=false, choice="")]
    pub parent: Option<Vec<Reference>>,
    /// Why the specimen was collected
    #[fhir(name="request", min="0", max="*", summary=false, modifier=false, choice="")]
    pub request: Option<Vec<Reference>>,
    /// grouped | pooled
    #[fhir(name="combined", min="0", max="1", summary=true, modifier=false, choice="")]
    pub combined: Option<CodeDt>,
    /// The role the specimen serves
    #[fhir(name="role", min="0", max="*", summary=false, modifier=false, choice="")]
    pub role: Option<Vec<CodeableConcept>>,
    /// The physical feature of a specimen
    #[fhir(name="feature", min="0", max="*", summary=false, modifier=false, choice="")]
    pub feature: Option<Vec<SpecimenFeatureBackboneElement>>,
    /// Collection details
    #[fhir(name="collection", min="0", max="1", summary=false, modifier=false, choice="")]
    pub collection: Option<SpecimenCollectionBackboneElement>,
    /// Processing and processing step details
    #[fhir(name="processing", min="0", max="*", summary=false, modifier=false, choice="")]
    pub processing: Option<Vec<SpecimenProcessingBackboneElement>>,
    /// Direct container of specimen (tube/slide, etc.)
    #[fhir(name="container", min="0", max="*", summary=false, modifier=false, choice="")]
    pub container: Option<Vec<SpecimenContainerBackboneElement>>,
    /// State of the specimen
    #[fhir(name="condition", min="0", max="*", summary=true, modifier=false, choice="")]
    pub condition: Option<Vec<CodeableConcept>>,
    /// Comments
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenContainerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Device resource for the container
    #[fhir(name="device", min="1", max="1", summary=false, modifier=false, choice="")]
    pub device: Option<Reference>,
    /// Where the container is
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// Quantity of specimen within container
    #[fhir(name="specimenQuantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub specimen_quantity: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenCollectionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Who collected the specimen
    #[fhir(name="collector", min="0", max="1", summary=true, modifier=false, choice="")]
    pub collector: Option<Reference>,
    /// Collection time
    #[fhir(name="collected", min="0", max="1", summary=true, modifier=false, choice="")]
    pub collected: Option<Period>,
    /// How long it took to collect specimen
    #[fhir(name="duration", min="0", max="1", summary=true, modifier=false, choice="")]
    pub duration: Option<Duration>,
    /// The quantity of specimen collected
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Technique used to perform collection
    #[fhir(name="method", min="0", max="1", summary=false, modifier=false, choice="")]
    pub method: Option<CodeableConcept>,
    /// Device used to perform collection
    #[fhir(name="device", min="0", max="1", summary=false, modifier=false, choice="")]
    pub device: Option<CodeableReference>,
    /// The procedure that collects the specimen
    #[fhir(name="procedure", min="0", max="1", summary=false, modifier=false, choice="")]
    pub procedure: Option<Reference>,
    /// Anatomical collection site
    #[fhir(name="bodySite", min="0", max="1", summary=false, modifier=false, choice="")]
    pub body_site: Option<CodeableReference>,
    /// Whether or how long patient abstained from food and/or drink
    #[fhir(name="fastingStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub fasting_status: Option<Duration>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenFeatureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Highlighted feature
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Information about the feature
    #[fhir(name="description", min="1", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenProcessingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Textual description of procedure
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// Indicates the treatment step  applied to the specimen
    #[fhir(name="method", min="0", max="1", summary=false, modifier=false, choice="")]
    pub method: Option<CodeableConcept>,
    /// Material used in the processing step
    #[fhir(name="additive", min="0", max="*", summary=false, modifier=false, choice="")]
    pub additive: Option<Vec<Reference>>,
    /// Date and time of specimen processing
    #[fhir(name="time", min="0", max="1", summary=false, modifier=false, choice="")]
    pub time: Option<Period>,
}


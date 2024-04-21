use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SpecimenDefinition {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Logical canonical URL to reference this SpecimenDefinition (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub url: Option<UriDt>,
    /// Business identifier
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Identifier>,
    /// Business version of the SpecimenDefinition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this {{title}} (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Name for this SpecimenDefinition (Human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// Based on FHIR definition of another SpecimenDefinition
    #[fhir(name="derivedFromCanonical", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub derived_from_canonical: Option<Vec<CanonicalDt>>,
    /// Based on external definition
    #[fhir(name="derivedFromUri", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub derived_from_uri: Option<Vec<UriDt>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// If this SpecimenDefinition is not for real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub experimental: Option<BooleanDt>,
    /// Type of subject for specimen collection
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// Date status first applied
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub date: Option<DateTimeDt>,
    /// The name of the individual or organization that published the SpecimenDefinition
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the SpecimenDefinition
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Content intends to support these contexts
    #[fhir(name="useContext", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for this SpecimenDefinition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this SpecimenDefinition is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright_label: Option<StringDt>,
    /// When SpecimenDefinition was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub approval_date: Option<DateDt>,
    /// The date on which the asset content was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub last_review_date: Option<DateDt>,
    /// The effective date range for the SpecimenDefinition
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub effective_period: Option<Period>,
    /// Kind of material to collect
    #[fhir(name="typeCollected", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub type_collected: Option<CodeableConcept>,
    /// Patient preparation for collection
    #[fhir(name="patientPreparation", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub patient_preparation: Option<Vec<CodeableConcept>>,
    /// Time aspect for collection
    #[fhir(name="timeAspect", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub time_aspect: Option<StringDt>,
    /// Specimen collection procedure
    #[fhir(name="collection", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub collection: Option<Vec<CodeableConcept>>,
    /// Specimen in container intended for testing by lab
    #[fhir(name="typeTested", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub type_tested: Option<Vec<SpecimenDefinitionTypeTestedBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenDefinitionTypeTestedBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Primary or secondary specimen
    #[fhir(name="isDerived", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub is_derived: Option<BooleanDt>,
    /// Type of intended specimen
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// preferred | alternate
    #[fhir(name="preference", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub preference: Option<CodeDt>,
    /// The specimen's container
    #[fhir(name="container", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub container: Option<SpecimenDefinitionTypeTestedContainerBackboneElement>,
    /// Requirements for specimen delivery and special handling
    #[fhir(name="requirement", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub requirement: Option<MarkdownDt>,
    /// The usual time for retaining this kind of specimen
    #[fhir(name="retentionTime", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub retention_time: Option<Duration>,
    /// Specimen for single use only
    #[fhir(name="singleUse", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub single_use: Option<BooleanDt>,
    /// Criterion specified for specimen rejection
    #[fhir(name="rejectionCriterion", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub rejection_criterion: Option<Vec<CodeableConcept>>,
    /// Specimen handling before testing
    #[fhir(name="handling", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub handling: Option<Vec<SpecimenDefinitionTypeTestedHandlingBackboneElement>>,
    /// Where the specimen will be tested
    #[fhir(name="testingDestination", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub testing_destination: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenDefinitionTypeTestedContainerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The material type used for the container
    #[fhir(name="material", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub material: Option<CodeableConcept>,
    /// Kind of container associated with the kind of specimen
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Color of container cap
    #[fhir(name="cap", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub cap: Option<CodeableConcept>,
    /// The description of the kind of container
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// The capacity of this kind of container
    #[fhir(name="capacity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub capacity: Option<Quantity>,
    /// Minimum volume
    #[fhir(name="minimumVolume", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub minimum_volume: Option<StringDt>,
    /// Additive associated with container
    #[fhir(name="additive", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub additive: Option<Vec<SpecimenDefinitionTypeTestedContainerAdditiveBackboneElement>>,
    /// Special processing applied to the container for this specimen type
    #[fhir(name="preparation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub preparation: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenDefinitionTypeTestedContainerAdditiveBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Additive associated with container
    #[fhir(name="additive", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub additive: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SpecimenDefinitionTypeTestedHandlingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Qualifies the interval of temperature
    #[fhir(name="temperatureQualifier", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub temperature_qualifier: Option<CodeableConcept>,
    /// Temperature range for these handling instructions
    #[fhir(name="temperatureRange", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub temperature_range: Option<Range>,
    /// Maximum preservation time
    #[fhir(name="maxDuration", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub max_duration: Option<Duration>,
    /// Preservation instruction
    #[fhir(name="instruction", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub instruction: Option<MarkdownDt>,
}


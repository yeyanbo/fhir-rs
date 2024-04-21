use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Measure {
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
    /// Canonical identifier for this measure, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the measure
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the measure
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this measure (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Name for this measure (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// Subordinate title of the measure
    #[fhir(name="subtitle", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subtitle: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub experimental: Option<BooleanDt>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// Population basis
    #[fhir(name="basis", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub basis: Option<CodeDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the measure
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for measure (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this measure is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub purpose: Option<MarkdownDt>,
    /// Describes the clinical usage of the measure
    #[fhir(name="usage", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub usage: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright_label: Option<StringDt>,
    /// When the measure was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub approval_date: Option<DateDt>,
    /// When the measure was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub last_review_date: Option<DateDt>,
    /// When the measure is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub effective_period: Option<Period>,
    /// The category of the measure, such as Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Logic used by the measure
    #[fhir(name="library", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub library: Option<Vec<CanonicalDt>>,
    /// Disclaimer for use of the measure or its referenced content
    #[fhir(name="disclaimer", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub disclaimer: Option<MarkdownDt>,
    /// proportion | ratio | continuous-variable | cohort
    #[fhir(name="scoring", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub scoring: Option<CodeableConcept>,
    /// What units?
    #[fhir(name="scoringUnit", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub scoring_unit: Option<CodeableConcept>,
    /// opportunity | all-or-nothing | linear | weighted
    #[fhir(name="compositeScoring", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub composite_scoring: Option<CodeableConcept>,
    /// process | outcome | structure | patient-reported-outcome | composite
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// How risk adjustment is applied for this measure
    #[fhir(name="riskAdjustment", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub risk_adjustment: Option<MarkdownDt>,
    /// How is rate aggregation performed for this measure
    #[fhir(name="rateAggregation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub rate_aggregation: Option<MarkdownDt>,
    /// Detailed description of why the measure exists
    #[fhir(name="rationale", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub rationale: Option<MarkdownDt>,
    /// Summary of clinical guidelines
    #[fhir(name="clinicalRecommendationStatement", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub clinical_recommendation_statement: Option<MarkdownDt>,
    /// increase | decrease
    #[fhir(name="improvementNotation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub improvement_notation: Option<CodeableConcept>,
    /// Defined terms used in the measure documentation
    #[fhir(name="term", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub term: Option<Vec<MeasureTermBackboneElement>>,
    /// Additional guidance for implementers (deprecated)
    #[fhir(name="guidance", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub guidance: Option<MarkdownDt>,
    /// Population criteria group
    #[fhir(name="group", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub group: Option<Vec<MeasureGroupBackboneElement>>,
    /// What other data should be reported with the measure
    #[fhir(name="supplementalData", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supplemental_data: Option<Vec<MeasureSupplementalDataBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MeasureGroupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for group in measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub link_id: Option<StringDt>,
    /// Meaning of the group
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// Summary description
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// process | outcome | structure | patient-reported-outcome | composite
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    #[fhir(name="subject", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// Population basis
    #[fhir(name="basis", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub basis: Option<CodeDt>,
    /// proportion | ratio | continuous-variable | cohort
    #[fhir(name="scoring", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub scoring: Option<CodeableConcept>,
    /// What units?
    #[fhir(name="scoringUnit", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub scoring_unit: Option<CodeableConcept>,
    /// How is rate aggregation performed for this measure
    #[fhir(name="rateAggregation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub rate_aggregation: Option<MarkdownDt>,
    /// increase | decrease
    #[fhir(name="improvementNotation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub improvement_notation: Option<CodeableConcept>,
    /// Logic used by the measure group
    #[fhir(name="library", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub library: Option<Vec<CanonicalDt>>,
    /// Population criteria
    #[fhir(name="population", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub population: Option<Vec<MeasureGroupPopulationBackboneElement>>,
    /// Stratifier criteria for the measure
    #[fhir(name="stratifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub stratifier: Option<Vec<MeasureGroupStratifierBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MeasureGroupPopulationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for population in measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub link_id: Option<StringDt>,
    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// The human readable description of this population criteria
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// The criteria that defines this population
    #[fhir(name="criteria", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub criteria: Option<Expression>,
    /// A group resource that defines this population
    #[fhir(name="groupDefinition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub group_definition: Option<Reference>,
    /// Which population
    #[fhir(name="inputPopulationId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub input_population_id: Option<StringDt>,
    /// Aggregation method for a measure score (e.g. sum, average, median, minimum, maximum, count)
    #[fhir(name="aggregateMethod", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub aggregate_method: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MeasureGroupStratifierBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for stratifier in measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub link_id: Option<StringDt>,
    /// Meaning of the stratifier
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// The human readable description of this stratifier
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// How the measure should be stratified
    #[fhir(name="criteria", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub criteria: Option<Expression>,
    /// A group resource that defines this population
    #[fhir(name="groupDefinition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub group_definition: Option<Reference>,
    /// Stratifier criteria component for the measure
    #[fhir(name="component", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub component: Option<Vec<MeasureGroupStratifierComponentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MeasureGroupStratifierComponentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for stratifier component in measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub link_id: Option<StringDt>,
    /// Meaning of the stratifier component
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// The human readable description of this stratifier component
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Component of how the measure should be stratified
    #[fhir(name="criteria", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub criteria: Option<Expression>,
    /// A group resource that defines this population
    #[fhir(name="groupDefinition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub group_definition: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MeasureSupplementalDataBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique id for supplementalData in measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub link_id: Option<StringDt>,
    /// Meaning of the supplemental data
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// supplemental-data | risk-adjustment-factor
    #[fhir(name="usage", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub usage: Option<Vec<CodeableConcept>>,
    /// The human readable description of this supplemental data
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Expression describing additional data to be reported
    #[fhir(name="criteria", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub criteria: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MeasureTermBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What term?
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// Meaning of the term
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub definition: Option<MarkdownDt>,
}


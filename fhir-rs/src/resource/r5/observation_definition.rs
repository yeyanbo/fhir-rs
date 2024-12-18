use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ObservationDefinition {
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
    /// Logical canonical URL to reference this ObservationDefinition (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Business identifier of the ObservationDefinition
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Business version of the ObservationDefinition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this ObservationDefinition (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this ObservationDefinition (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// If for testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice="")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// The name of the individual or organization that published the ObservationDefinition
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice="")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice="")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the ObservationDefinition
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Content intends to support these contexts
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for this ObservationDefinition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this ObservationDefinition is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// When ObservationDefinition was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub approval_date: Option<DateDt>,
    /// Date on which the asset content was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub last_review_date: Option<DateDt>,
    /// The effective date range for the ObservationDefinition
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false, choice="")]
    pub effective_period: Option<Period>,
    /// Based on FHIR definition of another observation
    #[fhir(name="derivedFromCanonical", min="0", max="*", summary=true, modifier=false, choice="")]
    pub derived_from_canonical: Option<Vec<CanonicalDt>>,
    /// Based on external definition
    #[fhir(name="derivedFromUri", min="0", max="*", summary=true, modifier=false, choice="")]
    pub derived_from_uri: Option<Vec<UriDt>>,
    /// Type of subject for the defined observation
    #[fhir(name="subject", min="0", max="*", summary=true, modifier=false, choice="")]
    pub subject: Option<Vec<CodeableConcept>>,
    /// Desired kind of performer for such kind of observation
    #[fhir(name="performerType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub performer_type: Option<CodeableConcept>,
    /// General type of observation
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of observation
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[fhir(name="permittedDataType", min="0", max="*", summary=false, modifier=false, choice="")]
    pub permitted_data_type: Option<Vec<CodeDt>>,
    /// Multiple results allowed for conforming observations
    #[fhir(name="multipleResultsAllowed", min="0", max="1", summary=false, modifier=false, choice="")]
    pub multiple_results_allowed: Option<BooleanDt>,
    /// Body part to be observed
    #[fhir(name="bodySite", min="0", max="1", summary=false, modifier=false, choice="")]
    pub body_site: Option<CodeableConcept>,
    /// Method used to produce the observation
    #[fhir(name="method", min="0", max="1", summary=false, modifier=false, choice="")]
    pub method: Option<CodeableConcept>,
    /// Kind of specimen used by this type of observation
    #[fhir(name="specimen", min="0", max="*", summary=false, modifier=false, choice="")]
    pub specimen: Option<Vec<Reference>>,
    /// Measurement device or model of device
    #[fhir(name="device", min="0", max="*", summary=false, modifier=false, choice="")]
    pub device: Option<Vec<Reference>>,
    /// The preferred name to be used when reporting the observation results
    #[fhir(name="preferredReportName", min="0", max="1", summary=false, modifier=false, choice="")]
    pub preferred_report_name: Option<StringDt>,
    /// Unit for quantitative results
    #[fhir(name="permittedUnit", min="0", max="*", summary=false, modifier=false, choice="")]
    pub permitted_unit: Option<Vec<Coding>>,
    /// Set of qualified values for observation results
    #[fhir(name="qualifiedValue", min="0", max="*", summary=false, modifier=false, choice="")]
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedValueBackboneElement>>,
    /// Definitions of related resources belonging to this kind of observation group
    #[fhir(name="hasMember", min="0", max="*", summary=false, modifier=false, choice="")]
    pub has_member: Option<Vec<Reference>>,
    /// Component results
    #[fhir(name="component", min="0", max="*", summary=false, modifier=false, choice="")]
    pub component: Option<Vec<ObservationDefinitionComponentBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ObservationDefinitionQualifiedValueBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Context qualifier for the set of qualified values
    #[fhir(name="context", min="0", max="1", summary=false, modifier=false, choice="")]
    pub context: Option<CodeableConcept>,
    /// Targetted population for the set of qualified values
    #[fhir(name="appliesTo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub applies_to: Option<Vec<CodeableConcept>>,
    /// male | female | other | unknown
    #[fhir(name="gender", min="0", max="1", summary=false, modifier=false, choice="")]
    pub gender: Option<CodeDt>,
    /// Applicable age range for the set of qualified values
    #[fhir(name="age", min="0", max="1", summary=false, modifier=false, choice="")]
    pub age: Option<Range>,
    /// Applicable gestational age range for the set of qualified values
    #[fhir(name="gestationalAge", min="0", max="1", summary=false, modifier=false, choice="")]
    pub gestational_age: Option<Range>,
    /// Condition associated with the set of qualified values
    #[fhir(name="condition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub condition: Option<StringDt>,
    /// reference | critical | absolute
    #[fhir(name="rangeCategory", min="0", max="1", summary=false, modifier=false, choice="")]
    pub range_category: Option<CodeDt>,
    /// The range for continuous or ordinal observations
    #[fhir(name="range", min="0", max="1", summary=false, modifier=false, choice="")]
    pub range: Option<Range>,
    /// Value set of valid coded values as part of this set of qualified values
    #[fhir(name="validCodedValueSet", min="0", max="1", summary=false, modifier=false, choice="")]
    pub valid_coded_value_set: Option<CanonicalDt>,
    /// Value set of normal coded values as part of this set of qualified values
    #[fhir(name="normalCodedValueSet", min="0", max="1", summary=false, modifier=false, choice="")]
    pub normal_coded_value_set: Option<CanonicalDt>,
    /// Value set of abnormal coded values as part of this set of qualified values
    #[fhir(name="abnormalCodedValueSet", min="0", max="1", summary=false, modifier=false, choice="")]
    pub abnormal_coded_value_set: Option<CanonicalDt>,
    /// Value set of critical coded values as part of this set of qualified values
    #[fhir(name="criticalCodedValueSet", min="0", max="1", summary=false, modifier=false, choice="")]
    pub critical_coded_value_set: Option<CanonicalDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ObservationDefinitionComponentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of observation
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Quantity | CodeableConcept | string | boolean | integer | Range | Ratio | SampledData | time | dateTime | Period
    #[fhir(name="permittedDataType", min="0", max="*", summary=false, modifier=false, choice="")]
    pub permitted_data_type: Option<Vec<CodeDt>>,
    /// Unit for quantitative results
    #[fhir(name="permittedUnit", min="0", max="*", summary=false, modifier=false, choice="")]
    pub permitted_unit: Option<Vec<Coding>>,
    /// Set of qualified values for observation results
    #[fhir(name="qualifiedValue", min="0", max="*", summary=false, modifier=false, choice="")]
    pub qualified_value: Option<Vec<ObservationDefinitionQualifiedValueBackboneElement>>,
}


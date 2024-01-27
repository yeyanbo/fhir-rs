use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Evidence {
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
    /// Canonical identifier for this evidence, represented as a globally unique URI
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the summary
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of this summary
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this summary (machine friendly)
    #[fhir(name="name", min="0", max="1", summary="false", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this summary (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// Citation for this evidence
    #[fhir(name="citeAs", min="0", max="1", summary="false", modifier="false")]
    pub cite_as: Option<MarkdownDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="false", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// When the summary was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary="false", modifier="false")]
    pub approval_date: Option<DateDt>,
    /// When the summary was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary="false", modifier="false")]
    pub last_review_date: Option<DateDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary="true", modifier="false")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary="false", modifier="false")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary="false", modifier="false")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary="true", modifier="false")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Why this Evidence is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// Link or citation to artifact associated with the summary
    #[fhir(name="relatedArtifact", min="0", max="*", summary="false", modifier="false")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Description of the particular summary
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Declarative description of the Evidence
    #[fhir(name="assertion", min="0", max="1", summary="false", modifier="false")]
    pub assertion: Option<MarkdownDt>,
    /// Footnotes and/or explanatory notes
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Evidence variable such as population, exposure, or outcome
    #[fhir(name="variableDefinition", min="1", max="*", summary="false", modifier="false")]
    pub variable_definition: Option<Vec<EvidenceVariableDefinitionBackboneElement>>,
    /// The method to combine studies
    #[fhir(name="synthesisType", min="0", max="1", summary="false", modifier="false")]
    pub synthesis_type: Option<CodeableConcept>,
    /// The design of the study that produced this evidence
    #[fhir(name="studyDesign", min="0", max="*", summary="false", modifier="false")]
    pub study_design: Option<Vec<CodeableConcept>>,
    /// Values and parameters for a single statistic
    #[fhir(name="statistic", min="0", max="*", summary="false", modifier="false")]
    pub statistic: Option<Vec<EvidenceStatisticBackboneElement>>,
    /// Certainty or quality of the evidence
    #[fhir(name="certainty", min="0", max="*", summary="false", modifier="false")]
    pub certainty: Option<Vec<EvidenceCertaintyBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceCertaintyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Textual description of certainty
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Footnotes and/or explanatory notes
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Aspect of certainty being rated
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Assessment or judgement of the aspect
    #[fhir(name="rating", min="0", max="1", summary="false", modifier="false")]
    pub rating: Option<CodeableConcept>,
    /// Individual or group who did the rating
    #[fhir(name="rater", min="0", max="1", summary="false", modifier="false")]
    pub rater: Option<StringDt>,
    /// A domain or subdomain of certainty
    #[fhir(name="subcomponent", min="0", max="*", summary="false", modifier="false")]
    pub subcomponent: Option<Vec<EvidenceCertaintyBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceVariableDefinitionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A text description or summary of the variable
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Footnotes and/or explanatory notes
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// population | subpopulation | exposure | referenceExposure | measuredVariable | confounder
    #[fhir(name="variableRole", min="1", max="1", summary="true", modifier="false")]
    pub variable_role: Option<CodeableConcept>,
    /// Definition of the actual variable related to the statistic(s)
    #[fhir(name="observed", min="0", max="1", summary="true", modifier="false")]
    pub observed: Option<Reference>,
    /// Definition of the intended variable related to the Evidence
    #[fhir(name="intended", min="0", max="1", summary="false", modifier="false")]
    pub intended: Option<Reference>,
    /// low | moderate | high | exact
    #[fhir(name="directnessMatch", min="0", max="1", summary="false", modifier="false")]
    pub directness_match: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceStatisticBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Description of content
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Footnotes and/or explanatory notes
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Type of statistic, e.g., relative risk
    #[fhir(name="statisticType", min="0", max="1", summary="false", modifier="false")]
    pub statistic_type: Option<CodeableConcept>,
    /// Associated category for categorical variable
    #[fhir(name="category", min="0", max="1", summary="false", modifier="false")]
    pub category: Option<CodeableConcept>,
    /// Statistic value
    #[fhir(name="quantity", min="0", max="1", summary="false", modifier="false")]
    pub quantity: Option<Quantity>,
    /// The number of events associated with the statistic
    #[fhir(name="numberOfEvents", min="0", max="1", summary="false", modifier="false")]
    pub number_of_events: Option<UnsignedIntDt>,
    /// The number of participants affected
    #[fhir(name="numberAffected", min="0", max="1", summary="false", modifier="false")]
    pub number_affected: Option<UnsignedIntDt>,
    /// Number of samples in the statistic
    #[fhir(name="sampleSize", min="0", max="1", summary="false", modifier="false")]
    pub sample_size: Option<EvidenceStatisticSampleSizeBackboneElement>,
    /// An attribute of the Statistic
    #[fhir(name="attributeEstimate", min="0", max="*", summary="false", modifier="false")]
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimateBackboneElement>>,
    /// An aspect of the statistical model
    #[fhir(name="modelCharacteristic", min="0", max="*", summary="false", modifier="false")]
    pub model_characteristic: Option<Vec<EvidenceStatisticModelCharacteristicBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceStatisticSampleSizeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Textual description of sample size for statistic
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Footnote or explanatory note about the sample size
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Number of contributing studies
    #[fhir(name="numberOfStudies", min="0", max="1", summary="false", modifier="false")]
    pub number_of_studies: Option<UnsignedIntDt>,
    /// Cumulative number of participants
    #[fhir(name="numberOfParticipants", min="0", max="1", summary="false", modifier="false")]
    pub number_of_participants: Option<UnsignedIntDt>,
    /// Number of participants with known results for measured variables
    #[fhir(name="knownDataCount", min="0", max="1", summary="false", modifier="false")]
    pub known_data_count: Option<UnsignedIntDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceStatisticModelCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Model specification
    #[fhir(name="code", min="1", max="1", summary="false", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Numerical value to complete model specification
    #[fhir(name="value", min="0", max="1", summary="false", modifier="false")]
    pub value: Option<Quantity>,
    /// A variable adjusted for in the adjusted analysis
    #[fhir(name="variable", min="0", max="*", summary="false", modifier="false")]
    pub variable: Option<Vec<EvidenceStatisticModelCharacteristicVariableBackboneElement>>,
    /// An attribute of the statistic used as a model characteristic
    #[fhir(name="attributeEstimate", min="0", max="*", summary="false", modifier="false")]
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimateBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceStatisticModelCharacteristicVariableBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Description of the variable
    #[fhir(name="variableDefinition", min="1", max="1", summary="false", modifier="false")]
    pub variable_definition: Option<Reference>,
    /// continuous | dichotomous | ordinal | polychotomous
    #[fhir(name="handling", min="0", max="1", summary="false", modifier="false")]
    pub handling: Option<CodeDt>,
    /// Description for grouping of ordinal or polychotomous variables
    #[fhir(name="valueCategory", min="0", max="*", summary="false", modifier="false")]
    pub value_category: Option<Vec<CodeableConcept>>,
    /// Discrete value for grouping of ordinal or polychotomous variables
    #[fhir(name="valueQuantity", min="0", max="*", summary="false", modifier="false")]
    pub value_quantity: Option<Vec<Quantity>>,
    /// Range of values for grouping of ordinal or polychotomous variables
    #[fhir(name="valueRange", min="0", max="*", summary="false", modifier="false")]
    pub value_range: Option<Vec<Range>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct EvidenceStatisticAttributeEstimateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Textual description of the attribute estimate
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Footnote or explanatory note about the estimate
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// The type of attribute estimate, e.g., confidence interval or p value
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The singular quantity of the attribute estimate, for attribute estimates represented as single values; also used to report unit of measure
    #[fhir(name="quantity", min="0", max="1", summary="false", modifier="false")]
    pub quantity: Option<Quantity>,
    /// Level of confidence interval, e.g., 0.95 for 95% confidence interval
    #[fhir(name="level", min="0", max="1", summary="false", modifier="false")]
    pub level: Option<DecimalDt>,
    /// Lower and upper bound values of the attribute estimate
    #[fhir(name="range", min="0", max="1", summary="false", modifier="false")]
    pub range: Option<Range>,
    /// A nested attribute estimate; which is the attribute estimate of an attribute estimate
    #[fhir(name="attributeEstimate", min="0", max="*", summary="false", modifier="false")]
    pub attribute_estimate: Option<Vec<EvidenceStatisticAttributeEstimateBackboneElement>>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MeasureReport {
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
    /// Additional identifier for the MeasureReport
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// complete | pending | error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// individual | subject-list | summary | data-exchange
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// incremental | snapshot
    #[fhir(name="dataUpdateType", min="0", max="1", summary=true, modifier=true)]
    pub data_update_type: Option<CodeDt>,
    /// What measure was calculated
    #[fhir(name="measure", min="0", max="1", summary=true, modifier=false, choice="")]
    pub measure: Option<CanonicalDt>,
    /// What individual(s) the report is for
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// When the measure was calculated
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Who is reporting the data
    #[fhir(name="reporter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub reporter: Option<Reference>,
    /// What vendor prepared the data
    #[fhir(name="reportingVendor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reporting_vendor: Option<Reference>,
    /// Where the reported data is from
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// What period the report covers
    #[fhir(name="period", min="1", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// What parameters were provided to the report
    #[fhir(name="inputParameters", min="0", max="1", summary=false, modifier=false, choice="")]
    pub input_parameters: Option<Reference>,
    /// What scoring method (e.g. proportion, ratio, continuous-variable)
    #[fhir(name="scoring", min="0", max="1", summary=true, modifier=true)]
    pub scoring: Option<CodeableConcept>,
    /// increase | decrease
    #[fhir(name="improvementNotation", min="0", max="1", summary=true, modifier=true)]
    pub improvement_notation: Option<CodeableConcept>,
    /// Measure results for each group
    #[fhir(name="group", min="0", max="*", summary=false, modifier=false, choice="")]
    pub group: Option<Vec<MeasureReportGroupBackboneElement>>,
    /// Additional information collected for the report
    #[fhir(name="supplementalData", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supplemental_data: Option<Vec<Reference>>,
    /// What data was used to calculate the measure score
    #[fhir(name="evaluatedResource", min="0", max="*", summary=false, modifier=false, choice="")]
    pub evaluated_resource: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MeasureReportGroupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific group from Measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link_id: Option<StringDt>,
    /// Meaning of the group
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// What individual(s) the report is for
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// The populations in the group
    #[fhir(name="population", min="0", max="*", summary=false, modifier=false, choice="")]
    pub population: Option<Vec<MeasureReportGroupPopulationBackboneElement>>,
    /// What score this group achieved
    #[fhir(name="measureScore", min="0", max="1", summary=true, modifier=false, choice="")]
    pub measure_score: Option<Duration>,
    /// Stratification results
    #[fhir(name="stratifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub stratifier: Option<Vec<MeasureReportGroupStratifierBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MeasureReportGroupPopulationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific population from Measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link_id: Option<StringDt>,
    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Size of the population
    #[fhir(name="count", min="0", max="1", summary=false, modifier=false, choice="")]
    pub count: Option<IntegerDt>,
    /// For subject-list reports, the subject results in this population
    #[fhir(name="subjectResults", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subject_results: Option<Reference>,
    /// For subject-list reports, a subject result in this population
    #[fhir(name="subjectReport", min="0", max="*", summary=false, modifier=false, choice="")]
    pub subject_report: Option<Vec<Reference>>,
    /// What individual(s) in the population
    #[fhir(name="subjects", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subjects: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MeasureReportGroupStratifierBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific stratifier from Measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link_id: Option<StringDt>,
    /// What stratifier of the group
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Stratum results, one for each unique value, or set of values, in the stratifier, or stratifier components
    #[fhir(name="stratum", min="0", max="*", summary=false, modifier=false, choice="")]
    pub stratum: Option<Vec<MeasureReportGroupStratifierStratumBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MeasureReportGroupStratifierStratumBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The stratum value, e.g. male
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Reference>,
    /// Stratifier component values
    #[fhir(name="component", min="0", max="*", summary=false, modifier=false, choice="")]
    pub component: Option<Vec<MeasureReportGroupStratifierStratumComponentBackboneElement>>,
    /// Population results in this stratum
    #[fhir(name="population", min="0", max="*", summary=false, modifier=false, choice="")]
    pub population: Option<Vec<MeasureReportGroupStratifierStratumPopulationBackboneElement>>,
    /// What score this stratum achieved
    #[fhir(name="measureScore", min="0", max="1", summary=false, modifier=false, choice="")]
    pub measure_score: Option<Duration>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MeasureReportGroupStratifierStratumPopulationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific population from Measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link_id: Option<StringDt>,
    /// initial-population | numerator | numerator-exclusion | denominator | denominator-exclusion | denominator-exception | measure-population | measure-population-exclusion | measure-observation
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Size of the population
    #[fhir(name="count", min="0", max="1", summary=false, modifier=false, choice="")]
    pub count: Option<IntegerDt>,
    /// For subject-list reports, the subject results in this population
    #[fhir(name="subjectResults", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subject_results: Option<Reference>,
    /// For subject-list reports, a subject result in this population
    #[fhir(name="subjectReport", min="0", max="*", summary=false, modifier=false, choice="")]
    pub subject_report: Option<Vec<Reference>>,
    /// What individual(s) in the population
    #[fhir(name="subjects", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subjects: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MeasureReportGroupStratifierStratumComponentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Pointer to specific stratifier component from Measure
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link_id: Option<StringDt>,
    /// What stratifier component of the group
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// The stratum component value, e.g. male
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Reference>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DiagnosticReport {
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
    /// Business identifier for report
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// What was requested
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub based_on: Option<Vec<Reference>>,
    /// registered | partial | preliminary | modified | final | amended | corrected | appended | cancelled | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Service category
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// Name/Code for this diagnostic report
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// The subject of the report - usually, but not always, the patient
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub subject: Option<Reference>,
    /// Health care event when test ordered
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub encounter: Option<Reference>,
    /// Clinically relevant time/time-period for report
    #[fhir(name="effective", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub effective: Option<Period>,
    /// DateTime this version was made
    #[fhir(name="issued", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub issued: Option<InstantDt>,
    /// Responsible Diagnostic Service
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub performer: Option<Vec<Reference>>,
    /// Primary result interpreter
    #[fhir(name="resultsInterpreter", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub results_interpreter: Option<Vec<Reference>>,
    /// Specimens this report is based on
    #[fhir(name="specimen", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub specimen: Option<Vec<Reference>>,
    /// Observations
    #[fhir(name="result", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub result: Option<Vec<Reference>>,
    /// Comments about the diagnostic report
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub note: Option<Vec<Annotation>>,
    /// Reference to full details of an analysis associated with the diagnostic report
    #[fhir(name="study", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub study: Option<Vec<Reference>>,
    /// Additional information supporting the diagnostic report
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supporting_info: Option<Vec<DiagnosticReportSupportingInfoBackboneElement>>,
    /// Key images or data associated with this report
    #[fhir(name="media", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub media: Option<Vec<DiagnosticReportMediaBackboneElement>>,
    /// Reference to a Composition resource for the DiagnosticReport structure
    #[fhir(name="composition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub composition: Option<Reference>,
    /// Clinical conclusion (interpretation) of test results
    #[fhir(name="conclusion", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub conclusion: Option<MarkdownDt>,
    /// Codes for the clinical conclusion of test results
    #[fhir(name="conclusionCode", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub conclusion_code: Option<Vec<CodeableConcept>>,
    /// Entire report as issued
    #[fhir(name="presentedForm", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub presented_form: Option<Vec<Attachment>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DiagnosticReportSupportingInfoBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Supporting information role code
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Supporting information reference
    #[fhir(name="reference", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub reference: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DiagnosticReportMediaBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Comment about the image or data (e.g. explanation)
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub comment: Option<StringDt>,
    /// Reference to the image or data source
    #[fhir(name="link", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub link: Option<Reference>,
}


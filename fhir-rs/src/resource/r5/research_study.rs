use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ResearchStudy {
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
    /// Canonical identifier for this study resource
    #[fhir(name="url", min="0", max="1", summary=false, modifier=false)]
    pub url: Option<UriDt>,
    /// Business Identifier for study
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// The business version for the study record
    #[fhir(name="version", min="0", max="1", summary=false, modifier=false)]
    pub version: Option<StringDt>,
    /// Name for this study (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false)]
    pub name: Option<StringDt>,
    /// Human readable name of the study
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
    /// Additional names for the study
    #[fhir(name="label", min="0", max="*", summary=false, modifier=false)]
    pub label: Option<Vec<ResearchStudyLabelBackboneElement>>,
    /// Steps followed in executing study
    #[fhir(name="protocol", min="0", max="*", summary=true, modifier=false)]
    pub protocol: Option<Vec<Reference>>,
    /// Part of larger study
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false)]
    pub part_of: Option<Vec<Reference>>,
    /// References, URLs, and attachments
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false)]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Date the resource last changed
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false)]
    pub date: Option<DateTimeDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// treatment | prevention | diagnostic | supportive-care | screening | health-services-research | basic-science | device-feasibility
    #[fhir(name="primaryPurposeType", min="0", max="1", summary=true, modifier=false)]
    pub primary_purpose_type: Option<CodeableConcept>,
    /// n-a | early-phase-1 | phase-1 | phase-1-phase-2 | phase-2 | phase-2-phase-3 | phase-3 | phase-4
    #[fhir(name="phase", min="0", max="1", summary=true, modifier=false)]
    pub phase: Option<CodeableConcept>,
    /// Classifications of the study design characteristics
    #[fhir(name="studyDesign", min="0", max="*", summary=true, modifier=false)]
    pub study_design: Option<Vec<CodeableConcept>>,
    /// Drugs, devices, etc. under study
    #[fhir(name="focus", min="0", max="*", summary=false, modifier=false)]
    pub focus: Option<Vec<CodeableReference>>,
    /// Condition being studied
    #[fhir(name="condition", min="0", max="*", summary=true, modifier=false)]
    pub condition: Option<Vec<CodeableConcept>>,
    /// Used to search for the study
    #[fhir(name="keyword", min="0", max="*", summary=true, modifier=false)]
    pub keyword: Option<Vec<CodeableConcept>>,
    /// Geographic area for the study
    #[fhir(name="region", min="0", max="*", summary=true, modifier=false)]
    pub region: Option<Vec<CodeableConcept>>,
    /// Brief text explaining the study
    #[fhir(name="descriptionSummary", min="0", max="1", summary=false, modifier=false)]
    pub description_summary: Option<MarkdownDt>,
    /// Detailed narrative of the study
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// When the study began and ended
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
    /// Facility where study activities are conducted
    #[fhir(name="site", min="0", max="*", summary=true, modifier=false)]
    pub site: Option<Vec<Reference>>,
    /// Comments made about the study
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Classification for the study
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Sponsors, collaborators, and other parties
    #[fhir(name="associatedParty", min="0", max="*", summary=false, modifier=false)]
    pub associated_party: Option<Vec<ResearchStudyAssociatedPartyBackboneElement>>,
    /// Status of study with time for that status
    #[fhir(name="progressStatus", min="0", max="*", summary=false, modifier=false)]
    pub progress_status: Option<Vec<ResearchStudyProgressStatusBackboneElement>>,
    /// accrual-goal-met | closed-due-to-toxicity | closed-due-to-lack-of-study-progress | temporarily-closed-per-study-design
    #[fhir(name="whyStopped", min="0", max="1", summary=true, modifier=false)]
    pub why_stopped: Option<CodeableConcept>,
    /// Target or actual group of participants enrolled in study
    #[fhir(name="recruitment", min="0", max="1", summary=true, modifier=false)]
    pub recruitment: Option<ResearchStudyRecruitmentBackboneElement>,
    /// Defined path through the study for a subject
    #[fhir(name="comparisonGroup", min="0", max="*", summary=false, modifier=false)]
    pub comparison_group: Option<Vec<ResearchStudyComparisonGroupBackboneElement>>,
    /// A goal for the study
    #[fhir(name="objective", min="0", max="*", summary=false, modifier=false)]
    pub objective: Option<Vec<ResearchStudyObjectiveBackboneElement>>,
    /// A variable measured during the study
    #[fhir(name="outcomeMeasure", min="0", max="*", summary=false, modifier=false)]
    pub outcome_measure: Option<Vec<ResearchStudyOutcomeMeasureBackboneElement>>,
    /// Link to results generated during the study
    #[fhir(name="result", min="0", max="*", summary=true, modifier=false)]
    pub result: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyAssociatedPartyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of associated party
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false)]
    pub name: Option<StringDt>,
    /// sponsor | lead-sponsor | sponsor-investigator | primary-investigator | collaborator | funding-source | general-contact | recruitment-contact | sub-investigator | study-director | study-chair
    #[fhir(name="role", min="1", max="1", summary=false, modifier=false)]
    pub role: Option<CodeableConcept>,
    /// When active in the role
    #[fhir(name="period", min="0", max="*", summary=false, modifier=false)]
    pub period: Option<Vec<Period>>,
    /// nih | fda | government | nonprofit | academic | industry
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Individual or organization associated with study (use practitionerRole to specify their organisation)
    #[fhir(name="party", min="0", max="1", summary=false, modifier=false)]
    pub party: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyComparisonGroupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Allows the comparisonGroup for the study and the comparisonGroup for the subject to be linked easily
    #[fhir(name="linkId", min="0", max="1", summary=false, modifier=false)]
    pub link_id: Option<IdDt>,
    /// Label for study comparisonGroup
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false)]
    pub name: Option<StringDt>,
    /// Categorization of study comparisonGroup
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Short explanation of study path
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Interventions or exposures in this comparisonGroup or cohort
    #[fhir(name="intendedExposure", min="0", max="*", summary=false, modifier=false)]
    pub intended_exposure: Option<Vec<Reference>>,
    /// Group of participants who were enrolled in study comparisonGroup
    #[fhir(name="observedGroup", min="0", max="1", summary=false, modifier=false)]
    pub observed_group: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyObjectiveBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for the objective
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false)]
    pub name: Option<StringDt>,
    /// primary | secondary | exploratory
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Description of the objective
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyLabelBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// primary | official | scientific | plain-language | subtitle | short-title | acronym | earlier-title | language | auto-translated | human-use | machine-use | duplicate-uid
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The name
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyProgressStatusBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for status or state (e.g. recruitment status)
    #[fhir(name="state", min="1", max="1", summary=false, modifier=false)]
    pub state: Option<CodeableConcept>,
    /// Actual if true else anticipated
    #[fhir(name="actual", min="0", max="1", summary=false, modifier=false)]
    pub actual: Option<BooleanDt>,
    /// Date range
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false)]
    pub period: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyRecruitmentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Estimated total number of participants to be enrolled
    #[fhir(name="targetNumber", min="0", max="1", summary=false, modifier=false)]
    pub target_number: Option<UnsignedIntDt>,
    /// Actual total number of participants enrolled in study
    #[fhir(name="actualNumber", min="0", max="1", summary=false, modifier=false)]
    pub actual_number: Option<UnsignedIntDt>,
    /// Inclusion and exclusion criteria
    #[fhir(name="eligibility", min="0", max="1", summary=false, modifier=false)]
    pub eligibility: Option<Reference>,
    /// Group of participants who were enrolled in study
    #[fhir(name="actualGroup", min="0", max="1", summary=true, modifier=false)]
    pub actual_group: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ResearchStudyOutcomeMeasureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for the outcome
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false)]
    pub name: Option<StringDt>,
    /// primary | secondary | exploratory
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Description of the outcome
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Structured outcome definition
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false)]
    pub reference: Option<Reference>,
}


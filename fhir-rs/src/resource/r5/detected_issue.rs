use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DetectedIssue {
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
    /// Unique id for the detected issue
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// preliminary | final | entered-in-error | mitigated
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Type of detected issue, e.g. drug-drug, duplicate therapy, etc
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Specific type of detected issue, e.g. drug-drug, duplicate therapy, etc
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// high | moderate | low
    #[fhir(name="severity", min="0", max="1", summary=true, modifier=false, choice="")]
    pub severity: Option<CodeDt>,
    /// Associated subject
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Encounter detected issue is part of
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// When identified
    #[fhir(name="identified", min="0", max="1", summary=true, modifier=false, choice="")]
    pub identified: Option<Period>,
    /// The provider or device that identified the issue
    #[fhir(name="author", min="0", max="1", summary=true, modifier=false, choice="")]
    pub author: Option<Reference>,
    /// Problem resource
    #[fhir(name="implicated", min="0", max="*", summary=true, modifier=false, choice="")]
    pub implicated: Option<Vec<Reference>>,
    /// Supporting evidence
    #[fhir(name="evidence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub evidence: Option<Vec<DetectedIssueEvidenceBackboneElement>>,
    /// Description and context
    #[fhir(name="detail", min="0", max="1", summary=false, modifier=false, choice="")]
    pub detail: Option<MarkdownDt>,
    /// Authority for issue
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<UriDt>,
    /// Step taken to address
    #[fhir(name="mitigation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub mitigation: Option<Vec<DetectedIssueMitigationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct DetectedIssueMitigationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What mitigation?
    #[fhir(name="action", min="1", max="1", summary=false, modifier=false, choice="")]
    pub action: Option<CodeableConcept>,
    /// Date committed
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Who is committing?
    #[fhir(name="author", min="0", max="1", summary=false, modifier=false, choice="")]
    pub author: Option<Reference>,
    /// Additional notes about the mitigation
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct DetectedIssueEvidenceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Manifestation
    #[fhir(name="code", min="0", max="*", summary=false, modifier=false, choice="")]
    pub code: Option<Vec<CodeableConcept>>,
    /// Supporting information
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail: Option<Vec<Reference>>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct OperationOutcome {
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
    /// A single issue associated with the action
    #[fhir(name="issue", min="1", max="*", summary=true, modifier=false, choice=false)]
    pub issue: Option<Vec<OperationOutcomeIssueBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct OperationOutcomeIssueBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// fatal | error | warning | information | success
    #[fhir(name="severity", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub severity: Option<CodeDt>,
    /// Error or warning code
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub code: Option<CodeDt>,
    /// Additional details about the error
    #[fhir(name="details", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub details: Option<CodeableConcept>,
    /// Additional diagnostic information about the issue
    #[fhir(name="diagnostics", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub diagnostics: Option<StringDt>,
    /// Deprecated: Path of element(s) related to issue
    #[fhir(name="location", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub location: Option<Vec<StringDt>>,
    /// FHIRPath of element(s) related to issue
    #[fhir(name="expression", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub expression: Option<Vec<StringDt>>,
}


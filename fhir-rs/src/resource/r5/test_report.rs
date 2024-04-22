use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct TestReport {
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
    /// External identifier
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Informal name of the executed TestReport
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// completed | in-progress | waiting | stopped | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Canonical URL to the  version-specific TestScript that was executed to produce this TestReport
    #[fhir(name="testScript", min="1", max="1", summary=true, modifier=false, choice="")]
    pub test_script: Option<CanonicalDt>,
    /// pass | fail | pending
    #[fhir(name="result", min="1", max="1", summary=true, modifier=false, choice="")]
    pub result: Option<CodeDt>,
    /// The final score (percentage of tests passed) resulting from the execution of the TestScript
    #[fhir(name="score", min="0", max="1", summary=true, modifier=false, choice="")]
    pub score: Option<DecimalDt>,
    /// Name of the tester producing this report (Organization or individual)
    #[fhir(name="tester", min="0", max="1", summary=true, modifier=false, choice="")]
    pub tester: Option<StringDt>,
    /// When the TestScript was executed and this TestReport was generated
    #[fhir(name="issued", min="0", max="1", summary=true, modifier=false, choice="")]
    pub issued: Option<DateTimeDt>,
    /// A participant in the test execution, either the execution engine, a client, or a server
    #[fhir(name="participant", min="0", max="*", summary=false, modifier=false, choice="")]
    pub participant: Option<Vec<TestReportParticipantBackboneElement>>,
    /// The results of the series of required setup operations before the tests were executed
    #[fhir(name="setup", min="0", max="1", summary=false, modifier=false, choice="")]
    pub setup: Option<TestReportSetupBackboneElement>,
    /// A test executed from the test script
    #[fhir(name="test", min="0", max="*", summary=false, modifier=false, choice="")]
    pub test: Option<Vec<TestReportTestBackboneElement>>,
    /// The results of running the series of required clean up steps
    #[fhir(name="teardown", min="0", max="1", summary=false, modifier=false, choice="")]
    pub teardown: Option<TestReportTeardownBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportSetupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A setup operation or assert that was executed
    #[fhir(name="action", min="1", max="*", summary=false, modifier=false, choice="")]
    pub action: Option<Vec<TestReportSetupActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportSetupActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The operation to perform
    #[fhir(name="operation", min="0", max="1", summary=false, modifier=false, choice="")]
    pub operation: Option<TestReportSetupActionOperationBackboneElement>,
    /// The assertion to perform
    #[fhir(name="assert", min="0", max="1", summary=false, modifier=false, choice="")]
    pub assert: Option<TestReportSetupActionAssertBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportSetupActionOperationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// pass | skip | fail | warning | error
    #[fhir(name="result", min="1", max="1", summary=false, modifier=false, choice="")]
    pub result: Option<CodeDt>,
    /// A message associated with the result
    #[fhir(name="message", min="0", max="1", summary=false, modifier=false, choice="")]
    pub message: Option<MarkdownDt>,
    /// A link to further details on the result
    #[fhir(name="detail", min="0", max="1", summary=false, modifier=false, choice="")]
    pub detail: Option<UriDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportSetupActionAssertBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// pass | skip | fail | warning | error
    #[fhir(name="result", min="1", max="1", summary=false, modifier=false, choice="")]
    pub result: Option<CodeDt>,
    /// A message associated with the result
    #[fhir(name="message", min="0", max="1", summary=false, modifier=false, choice="")]
    pub message: Option<MarkdownDt>,
    /// A link to further details on the result
    #[fhir(name="detail", min="0", max="1", summary=false, modifier=false, choice="")]
    pub detail: Option<StringDt>,
    /// Links or references to the testing requirements
    #[fhir(name="requirement", min="0", max="*", summary=false, modifier=false, choice="")]
    pub requirement: Option<Vec<TestReportSetupActionAssertRequirementBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportSetupActionAssertRequirementBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Link or reference to the testing requirement
    #[fhir(name="link", min="0", max="1", summary=false, modifier=false, choice="")]
    pub link: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportTeardownBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// One or more teardown operations performed
    #[fhir(name="action", min="1", max="*", summary=false, modifier=false, choice="")]
    pub action: Option<Vec<TestReportTeardownActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportTeardownActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The teardown operation performed
    #[fhir(name="operation", min="1", max="1", summary=false, modifier=false, choice="")]
    pub operation: Option<TestReportSetupActionOperationBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportTestBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Tracking/logging name of this test
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Tracking/reporting short description of the test
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// A test operation or assert that was performed
    #[fhir(name="action", min="1", max="*", summary=false, modifier=false, choice="")]
    pub action: Option<Vec<TestReportTestActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportTestActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The operation performed
    #[fhir(name="operation", min="0", max="1", summary=false, modifier=false, choice="")]
    pub operation: Option<TestReportSetupActionOperationBackboneElement>,
    /// The assertion performed
    #[fhir(name="assert", min="0", max="1", summary=false, modifier=false, choice="")]
    pub assert: Option<TestReportSetupActionAssertBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestReportParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// test-engine | client | server
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// The uri of the participant. An absolute URL is preferred
    #[fhir(name="uri", min="1", max="1", summary=false, modifier=false, choice="")]
    pub uri: Option<UriDt>,
    /// The display name of the participant
    #[fhir(name="display", min="0", max="1", summary=false, modifier=false, choice="")]
    pub display: Option<StringDt>,
}


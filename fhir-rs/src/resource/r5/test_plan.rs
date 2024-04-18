use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct TestPlan {
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
    /// Canonical identifier for this test plan, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false)]
    pub url: Option<UriDt>,
    /// Business identifier identifier for the test plan
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the test plan
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this test plan (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name for this test plan (human friendly)
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false)]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false)]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false)]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the test plan
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction where the test plan applies (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this test plan is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false)]
    pub copyright_label: Option<StringDt>,
    /// The category of the Test Plan - can be acceptance, unit, performance
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// What is being tested with this Test Plan - a conformance resource, or narrative criteria, or an external reference
    #[fhir(name="scope", min="0", max="*", summary=false, modifier=false)]
    pub scope: Option<Vec<Reference>>,
    /// A description of test tools to be used in the test plan - narrative for now
    #[fhir(name="testTools", min="0", max="1", summary=false, modifier=false)]
    pub test_tools: Option<MarkdownDt>,
    /// The required criteria to execute the test plan - e.g. preconditions, previous tests
    #[fhir(name="dependency", min="0", max="*", summary=false, modifier=false)]
    pub dependency: Option<Vec<TestPlanDependencyBackboneElement>>,
    /// The threshold or criteria for the test plan to be considered successfully executed - narrative
    #[fhir(name="exitCriteria", min="0", max="1", summary=false, modifier=false)]
    pub exit_criteria: Option<MarkdownDt>,
    /// The test cases that constitute this plan
    #[fhir(name="testCase", min="0", max="*", summary=false, modifier=false)]
    pub test_case: Option<Vec<TestPlanTestCaseBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanDependencyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Description of the dependency criterium
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Link to predecessor test plans
    #[fhir(name="predecessor", min="0", max="1", summary=false, modifier=false)]
    pub predecessor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanTestCaseBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Sequence of test case in the test plan
    #[fhir(name="sequence", min="0", max="1", summary=false, modifier=false)]
    pub sequence: Option<IntegerDt>,
    /// The scope or artifact covered by the case
    #[fhir(name="scope", min="0", max="*", summary=false, modifier=false)]
    pub scope: Option<Vec<Reference>>,
    /// Required criteria to execute the test case
    #[fhir(name="dependency", min="0", max="*", summary=false, modifier=false)]
    pub dependency: Option<Vec<TestPlanTestCaseDependencyBackboneElement>>,
    /// The actual test to be executed
    #[fhir(name="testRun", min="0", max="*", summary=false, modifier=false)]
    pub test_run: Option<Vec<TestPlanTestCaseTestRunBackboneElement>>,
    /// The test data used in the test case
    #[fhir(name="testData", min="0", max="*", summary=false, modifier=false)]
    pub test_data: Option<Vec<TestPlanTestCaseTestDataBackboneElement>>,
    /// Test assertions or expectations
    #[fhir(name="assertion", min="0", max="*", summary=false, modifier=false)]
    pub assertion: Option<Vec<TestPlanTestCaseAssertionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanTestCaseDependencyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Description of the criteria
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Link to predecessor test plans
    #[fhir(name="predecessor", min="0", max="1", summary=false, modifier=false)]
    pub predecessor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanTestCaseTestDataBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of test data description, e.g. 'synthea'
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false)]
    pub type_: Option<Coding>,
    /// The actual test resources when they exist
    #[fhir(name="content", min="0", max="1", summary=false, modifier=false)]
    pub content: Option<Reference>,
    /// Pointer to a definition of test resources - narrative or structured e.g. synthetic data generation, etc
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false)]
    pub source: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanTestCaseTestRunBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The narrative description of the tests
    #[fhir(name="narrative", min="0", max="1", summary=false, modifier=false)]
    pub narrative: Option<MarkdownDt>,
    /// The test cases in a structured language e.g. gherkin, Postman, or FHIR TestScript
    #[fhir(name="script", min="0", max="1", summary=false, modifier=false)]
    pub script: Option<TestPlanTestCaseTestRunScriptBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanTestCaseTestRunScriptBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The language for the test cases e.g. 'gherkin', 'testscript'
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeableConcept>,
    /// The actual content of the cases - references to TestScripts or externally defined content
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false)]
    pub source: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestPlanTestCaseAssertionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Assertion type - for example 'informative' or 'required' 
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The focus or object of the assertion
    #[fhir(name="object", min="0", max="*", summary=false, modifier=false)]
    pub object: Option<Vec<CodeableReference>>,
    /// The actual result assertion
    #[fhir(name="result", min="0", max="*", summary=false, modifier=false)]
    pub result: Option<Vec<CodeableReference>>,
}


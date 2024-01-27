use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct TestScript {
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
    /// Canonical identifier for this test script, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the test script
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the test script
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this test script (computer friendly)
    #[fhir(name="name", min="1", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this test script (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="true", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the test script
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for test script (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this test script is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// An abstract server representing a client or sender in a message exchange
    #[fhir(name="origin", min="0", max="*", summary="false", modifier="false")]
    pub origin: Option<Vec<TestScriptOriginBackboneElement>>,
    /// An abstract server representing a destination or receiver in a message exchange
    #[fhir(name="destination", min="0", max="*", summary="false", modifier="false")]
    pub destination: Option<Vec<TestScriptDestinationBackboneElement>>,
    /// Required capability that is assumed to function correctly on the FHIR server being tested
    #[fhir(name="metadata", min="0", max="1", summary="false", modifier="false")]
    pub metadata: Option<TestScriptMetadataBackboneElement>,
    /// Indication of the artifact(s) that are tested by this test case
    #[fhir(name="scope", min="0", max="*", summary="false", modifier="false")]
    pub scope: Option<Vec<TestScriptScopeBackboneElement>>,
    /// Fixture in the test script - by reference (uri)
    #[fhir(name="fixture", min="0", max="*", summary="false", modifier="false")]
    pub fixture: Option<Vec<TestScriptFixtureBackboneElement>>,
    /// Reference of the validation profile
    #[fhir(name="profile", min="0", max="*", summary="false", modifier="false")]
    pub profile: Option<Vec<CanonicalDt>>,
    /// Placeholder for evaluated elements
    #[fhir(name="variable", min="0", max="*", summary="false", modifier="false")]
    pub variable: Option<Vec<TestScriptVariableBackboneElement>>,
    /// A series of required setup operations before tests are executed
    #[fhir(name="setup", min="0", max="1", summary="false", modifier="false")]
    pub setup: Option<TestScriptSetupBackboneElement>,
    /// A test in this script
    #[fhir(name="test", min="0", max="*", summary="false", modifier="false")]
    pub test: Option<Vec<TestScriptTestBackboneElement>>,
    /// A series of required clean up steps
    #[fhir(name="teardown", min="0", max="1", summary="false", modifier="false")]
    pub teardown: Option<TestScriptTeardownBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptMetadataBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Links to the FHIR specification
    #[fhir(name="link", min="0", max="*", summary="false", modifier="false")]
    pub link: Option<Vec<TestScriptMetadataLinkBackboneElement>>,
    /// Capabilities  that are assumed to function correctly on the FHIR server being tested
    #[fhir(name="capability", min="1", max="*", summary="false", modifier="false")]
    pub capability: Option<Vec<TestScriptMetadataCapabilityBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptMetadataCapabilityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Are the capabilities required?
    #[fhir(name="required", min="1", max="1", summary="false", modifier="false")]
    pub required: Option<BooleanDt>,
    /// Are the capabilities validated?
    #[fhir(name="validated", min="1", max="1", summary="false", modifier="false")]
    pub validated: Option<BooleanDt>,
    /// The expected capabilities of the server
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// Which origin server these requirements apply to
    #[fhir(name="origin", min="0", max="*", summary="false", modifier="false")]
    pub origin: Option<Vec<IntegerDt>>,
    /// Which server these requirements apply to
    #[fhir(name="destination", min="0", max="1", summary="false", modifier="false")]
    pub destination: Option<IntegerDt>,
    /// Links to the FHIR specification
    #[fhir(name="link", min="0", max="*", summary="false", modifier="false")]
    pub link: Option<Vec<UriDt>>,
    /// Required Capability Statement
    #[fhir(name="capabilities", min="1", max="1", summary="false", modifier="false")]
    pub capabilities: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptMetadataLinkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// URL to the specification
    #[fhir(name="url", min="1", max="1", summary="false", modifier="false")]
    pub url: Option<UriDt>,
    /// Short description
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptSetupBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A setup operation or assert to perform
    #[fhir(name="action", min="1", max="*", summary="false", modifier="false")]
    pub action: Option<Vec<TestScriptSetupActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptSetupActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The setup operation to perform
    #[fhir(name="operation", min="0", max="1", summary="false", modifier="false")]
    pub operation: Option<TestScriptSetupActionOperationBackboneElement>,
    /// The assertion to perform
    #[fhir(name="assert", min="0", max="1", summary="false", modifier="false")]
    pub assert: Option<TestScriptSetupActionAssertBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptSetupActionOperationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The operation code type that will be executed
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<Coding>,
    /// Resource type
    #[fhir(name="resource", min="0", max="1", summary="false", modifier="false")]
    pub resource: Option<UriDt>,
    /// Tracking/logging operation label
    #[fhir(name="label", min="0", max="1", summary="false", modifier="false")]
    pub label: Option<StringDt>,
    /// Tracking/reporting operation description
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// Mime type to accept in the payload of the response, with charset etc
    #[fhir(name="accept", min="0", max="1", summary="false", modifier="false")]
    pub accept: Option<CodeDt>,
    /// Mime type of the request payload contents, with charset etc
    #[fhir(name="contentType", min="0", max="1", summary="false", modifier="false")]
    pub content_type: Option<CodeDt>,
    /// Server responding to the request
    #[fhir(name="destination", min="0", max="1", summary="false", modifier="false")]
    pub destination: Option<IntegerDt>,
    /// Whether or not to send the request url in encoded format
    #[fhir(name="encodeRequestUrl", min="1", max="1", summary="false", modifier="false")]
    pub encode_request_url: Option<BooleanDt>,
    /// delete | get | options | patch | post | put | head
    #[fhir(name="method", min="0", max="1", summary="false", modifier="false")]
    pub method: Option<CodeDt>,
    /// Server initiating the request
    #[fhir(name="origin", min="0", max="1", summary="false", modifier="false")]
    pub origin: Option<IntegerDt>,
    /// Explicitly defined path parameters
    #[fhir(name="params", min="0", max="1", summary="false", modifier="false")]
    pub params: Option<StringDt>,
    /// Each operation can have one or more header elements
    #[fhir(name="requestHeader", min="0", max="*", summary="false", modifier="false")]
    pub request_header: Option<Vec<TestScriptSetupActionOperationRequestHeaderBackboneElement>>,
    /// Fixture Id of mapped request
    #[fhir(name="requestId", min="0", max="1", summary="false", modifier="false")]
    pub request_id: Option<IdDt>,
    /// Fixture Id of mapped response
    #[fhir(name="responseId", min="0", max="1", summary="false", modifier="false")]
    pub response_id: Option<IdDt>,
    /// Fixture Id of body for PUT and POST requests
    #[fhir(name="sourceId", min="0", max="1", summary="false", modifier="false")]
    pub source_id: Option<IdDt>,
    /// Id of fixture used for extracting the [id],  [type], and [vid] for GET requests
    #[fhir(name="targetId", min="0", max="1", summary="false", modifier="false")]
    pub target_id: Option<IdDt>,
    /// Request URL
    #[fhir(name="url", min="0", max="1", summary="false", modifier="false")]
    pub url: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptSetupActionOperationRequestHeaderBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// HTTP header field name
    #[fhir(name="field", min="1", max="1", summary="false", modifier="false")]
    pub field: Option<StringDt>,
    /// HTTP headerfield value
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptSetupActionAssertBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Tracking/logging assertion label
    #[fhir(name="label", min="0", max="1", summary="false", modifier="false")]
    pub label: Option<StringDt>,
    /// Tracking/reporting assertion description
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// response | request
    #[fhir(name="direction", min="0", max="1", summary="false", modifier="false")]
    pub direction: Option<CodeDt>,
    /// Id of the source fixture to be evaluated
    #[fhir(name="compareToSourceId", min="0", max="1", summary="false", modifier="false")]
    pub compare_to_source_id: Option<StringDt>,
    /// The FHIRPath expression to evaluate against the source fixture
    #[fhir(name="compareToSourceExpression", min="0", max="1", summary="false", modifier="false")]
    pub compare_to_source_expression: Option<StringDt>,
    /// XPath or JSONPath expression to evaluate against the source fixture
    #[fhir(name="compareToSourcePath", min="0", max="1", summary="false", modifier="false")]
    pub compare_to_source_path: Option<StringDt>,
    /// Mime type to compare against the 'Content-Type' header
    #[fhir(name="contentType", min="0", max="1", summary="false", modifier="false")]
    pub content_type: Option<CodeDt>,
    /// fail | pass | skip | stop
    #[fhir(name="defaultManualCompletion", min="0", max="1", summary="false", modifier="false")]
    pub default_manual_completion: Option<CodeDt>,
    /// The FHIRPath expression to be evaluated
    #[fhir(name="expression", min="0", max="1", summary="false", modifier="false")]
    pub expression: Option<StringDt>,
    /// HTTP header field name
    #[fhir(name="headerField", min="0", max="1", summary="false", modifier="false")]
    pub header_field: Option<StringDt>,
    /// Fixture Id of minimum content resource
    #[fhir(name="minimumId", min="0", max="1", summary="false", modifier="false")]
    pub minimum_id: Option<StringDt>,
    /// Perform validation on navigation links?
    #[fhir(name="navigationLinks", min="0", max="1", summary="false", modifier="false")]
    pub navigation_links: Option<BooleanDt>,
    /// equals | notEquals | in | notIn | greaterThan | lessThan | empty | notEmpty | contains | notContains | eval | manualEval
    #[fhir(name="operator", min="0", max="1", summary="false", modifier="false")]
    pub operator: Option<CodeDt>,
    /// XPath or JSONPath expression
    #[fhir(name="path", min="0", max="1", summary="false", modifier="false")]
    pub path: Option<StringDt>,
    /// delete | get | options | patch | post | put | head
    #[fhir(name="requestMethod", min="0", max="1", summary="false", modifier="false")]
    pub request_method: Option<CodeDt>,
    /// Request URL comparison value
    #[fhir(name="requestURL", min="0", max="1", summary="false", modifier="false")]
    pub request_url: Option<StringDt>,
    /// Resource type
    #[fhir(name="resource", min="0", max="1", summary="false", modifier="false")]
    pub resource: Option<UriDt>,
    /// continue | switchingProtocols | okay | created | accepted | nonAuthoritativeInformation | noContent | resetContent | partialContent | multipleChoices | movedPermanently | found | seeOther | notModified | useProxy | temporaryRedirect | permanentRedirect | badRequest | unauthorized | paymentRequired | forbidden | notFound | methodNotAllowed | notAcceptable | proxyAuthenticationRequired | requestTimeout | conflict | gone | lengthRequired | preconditionFailed | contentTooLarge | uriTooLong | unsupportedMediaType | rangeNotSatisfiable | expectationFailed | misdirectedRequest | unprocessableContent | upgradeRequired | internalServerError | notImplemented | badGateway | serviceUnavailable | gatewayTimeout | httpVersionNotSupported
    #[fhir(name="response", min="0", max="1", summary="false", modifier="false")]
    pub response: Option<CodeDt>,
    /// HTTP response code to test
    #[fhir(name="responseCode", min="0", max="1", summary="false", modifier="false")]
    pub response_code: Option<StringDt>,
    /// Fixture Id of source expression or headerField
    #[fhir(name="sourceId", min="0", max="1", summary="false", modifier="false")]
    pub source_id: Option<IdDt>,
    /// If this assert fails, will the current test execution stop?
    #[fhir(name="stopTestOnFail", min="1", max="1", summary="false", modifier="false")]
    pub stop_test_on_fail: Option<BooleanDt>,
    /// Profile Id of validation profile reference
    #[fhir(name="validateProfileId", min="0", max="1", summary="false", modifier="false")]
    pub validate_profile_id: Option<IdDt>,
    /// The value to compare to
    #[fhir(name="value", min="0", max="1", summary="false", modifier="false")]
    pub value: Option<StringDt>,
    /// Will this assert produce a warning only on error?
    #[fhir(name="warningOnly", min="1", max="1", summary="false", modifier="false")]
    pub warning_only: Option<BooleanDt>,
    /// Links or references to the testing requirements
    #[fhir(name="requirement", min="0", max="*", summary="false", modifier="false")]
    pub requirement: Option<Vec<TestScriptSetupActionAssertRequirementBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptSetupActionAssertRequirementBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Link or reference to the testing requirement
    #[fhir(name="link", min="0", max="1", summary="false", modifier="false")]
    pub link: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptTeardownBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// One or more teardown operations to perform
    #[fhir(name="action", min="1", max="*", summary="false", modifier="false")]
    pub action: Option<Vec<TestScriptTeardownActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptTeardownActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The teardown operation to perform
    #[fhir(name="operation", min="1", max="1", summary="false", modifier="false")]
    pub operation: Option<TestScriptSetupActionOperationBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptDestinationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The index of the abstract destination server starting at 1
    #[fhir(name="index", min="1", max="1", summary="false", modifier="false")]
    pub index: Option<IntegerDt>,
    /// FHIR-Server | FHIR-SDC-FormManager | FHIR-SDC-FormReceiver | FHIR-SDC-FormProcessor
    #[fhir(name="profile", min="1", max="1", summary="false", modifier="false")]
    pub profile: Option<Coding>,
    /// The url path of the destination server
    #[fhir(name="url", min="0", max="1", summary="false", modifier="false")]
    pub url: Option<UrlDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptVariableBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Descriptive name for this variable
    #[fhir(name="name", min="1", max="1", summary="false", modifier="false")]
    pub name: Option<StringDt>,
    /// Default, hard-coded, or user-defined value for this variable
    #[fhir(name="defaultValue", min="0", max="1", summary="false", modifier="false")]
    pub default_value: Option<StringDt>,
    /// Natural language description of the variable
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// The FHIRPath expression against the fixture body
    #[fhir(name="expression", min="0", max="1", summary="false", modifier="false")]
    pub expression: Option<StringDt>,
    /// HTTP header field name for source
    #[fhir(name="headerField", min="0", max="1", summary="false", modifier="false")]
    pub header_field: Option<StringDt>,
    /// Hint help text for default value to enter
    #[fhir(name="hint", min="0", max="1", summary="false", modifier="false")]
    pub hint: Option<StringDt>,
    /// XPath or JSONPath against the fixture body
    #[fhir(name="path", min="0", max="1", summary="false", modifier="false")]
    pub path: Option<StringDt>,
    /// Fixture Id of source expression or headerField within this variable
    #[fhir(name="sourceId", min="0", max="1", summary="false", modifier="false")]
    pub source_id: Option<IdDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptFixtureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether or not to implicitly create the fixture during setup
    #[fhir(name="autocreate", min="1", max="1", summary="false", modifier="false")]
    pub autocreate: Option<BooleanDt>,
    /// Whether or not to implicitly delete the fixture during teardown
    #[fhir(name="autodelete", min="1", max="1", summary="false", modifier="false")]
    pub autodelete: Option<BooleanDt>,
    /// Reference of the resource
    #[fhir(name="resource", min="0", max="1", summary="false", modifier="false")]
    pub resource: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptOriginBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The index of the abstract origin server starting at 1
    #[fhir(name="index", min="1", max="1", summary="false", modifier="false")]
    pub index: Option<IntegerDt>,
    /// FHIR-Client | FHIR-SDC-FormFiller
    #[fhir(name="profile", min="1", max="1", summary="false", modifier="false")]
    pub profile: Option<Coding>,
    /// The url path of the origin server
    #[fhir(name="url", min="0", max="1", summary="false", modifier="false")]
    pub url: Option<UrlDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptScopeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The specific conformance artifact being tested
    #[fhir(name="artifact", min="1", max="1", summary="false", modifier="false")]
    pub artifact: Option<CanonicalDt>,
    /// required | optional | strict
    #[fhir(name="conformance", min="0", max="1", summary="false", modifier="false")]
    pub conformance: Option<CodeableConcept>,
    /// unit | integration | production
    #[fhir(name="phase", min="0", max="1", summary="false", modifier="false")]
    pub phase: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptTestBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Tracking/logging name of this test
    #[fhir(name="name", min="0", max="1", summary="false", modifier="false")]
    pub name: Option<StringDt>,
    /// Tracking/reporting short description of the test
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
    /// A test operation or assert to perform
    #[fhir(name="action", min="1", max="*", summary="false", modifier="false")]
    pub action: Option<Vec<TestScriptTestActionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct TestScriptTestActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The setup operation to perform
    #[fhir(name="operation", min="0", max="1", summary="false", modifier="false")]
    pub operation: Option<TestScriptSetupActionOperationBackboneElement>,
    /// The setup assertion to perform
    #[fhir(name="assert", min="0", max="1", summary="false", modifier="false")]
    pub assert: Option<TestScriptSetupActionAssertBackboneElement>,
}


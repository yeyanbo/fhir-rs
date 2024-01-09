use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct ExampleScenario {
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
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Canonical identifier for this example scenario, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the example scenario
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the example scenario
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// To be removed?
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this example scenario (human friendly)
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
    /// Natural language description of the ExampleScenario
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for example scenario (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// The purpose of the example, e.g. to illustrate a scenario
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// Individual involved in exchange
    #[fhir(name="actor", min="0", max="*", summary="false", modifier="false")]
    pub actor: Option<Vec<ExampleScenarioActorBackboneElement>>,
    /// Data used in the scenario
    #[fhir(name="instance", min="0", max="*", summary="false", modifier="false")]
    pub instance: Option<Vec<ExampleScenarioInstanceBackboneElement>>,
    /// Major process within scenario
    #[fhir(name="process", min="0", max="*", summary="false", modifier="false")]
    pub process: Option<Vec<ExampleScenarioProcessBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// ID or acronym of the instance
    #[fhir(name="key", min="1", max="1", summary="false", modifier="false")]
    pub key: Option<StringDt>,
    /// Data structure for example
    #[fhir(name="structureType", min="1", max="1", summary="false", modifier="false")]
    pub structure_type: Option<Coding>,
    /// E.g. 4.0.1
    #[fhir(name="structureVersion", min="0", max="1", summary="false", modifier="false")]
    pub structure_version: Option<StringDt>,
    /// Rules instance adheres to
    #[fhir(name="structureProfile", min="0", max="1", summary="false", modifier="false")]
    pub structure_profile: Option<UriDt>,
    /// Label for instance
    #[fhir(name="title", min="1", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Human-friendly description of the instance
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Example instance data
    #[fhir(name="content", min="0", max="1", summary="false", modifier="false")]
    pub content: Option<Reference>,
    /// Snapshot of instance that changes
    #[fhir(name="version", min="0", max="*", summary="false", modifier="false")]
    pub version: Option<Vec<ExampleScenarioInstanceVersionBackboneElement>>,
    /// Resources contained in the instance
    #[fhir(name="containedInstance", min="0", max="*", summary="false", modifier="false")]
    pub contained_instance: Option<Vec<ExampleScenarioInstanceContainedInstanceBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioInstanceVersionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// ID or acronym of the version
    #[fhir(name="key", min="1", max="1", summary="false", modifier="false")]
    pub key: Option<StringDt>,
    /// Label for instance version
    #[fhir(name="title", min="1", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Details about version
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Example instance version data
    #[fhir(name="content", min="0", max="1", summary="false", modifier="false")]
    pub content: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioInstanceContainedInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Key of contained instance
    #[fhir(name="instanceReference", min="1", max="1", summary="false", modifier="false")]
    pub instance_reference: Option<StringDt>,
    /// Key of contained instance version
    #[fhir(name="versionReference", min="0", max="1", summary="false", modifier="false")]
    pub version_reference: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioActorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// ID or acronym of the actor
    #[fhir(name="key", min="1", max="1", summary="false", modifier="false")]
    pub key: Option<StringDt>,
    /// person | system
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeDt>,
    /// Label for actor when rendering
    #[fhir(name="title", min="1", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Details about actor
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioProcessBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for procss
    #[fhir(name="title", min="1", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// Human-friendly description of the process
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Status before process starts
    #[fhir(name="preConditions", min="0", max="1", summary="false", modifier="false")]
    pub pre_conditions: Option<MarkdownDt>,
    /// Status after successful completion
    #[fhir(name="postConditions", min="0", max="1", summary="false", modifier="false")]
    pub post_conditions: Option<MarkdownDt>,
    /// Event within of the process
    #[fhir(name="step", min="0", max="*", summary="false", modifier="false")]
    pub step: Option<Vec<ExampleScenarioProcessStepBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioProcessStepBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Sequential number of the step
    #[fhir(name="number", min="0", max="1", summary="false", modifier="false")]
    pub number: Option<StringDt>,
    /// Step is nested process
    #[fhir(name="process", min="0", max="1", summary="false", modifier="false")]
    pub process: Option<ExampleScenarioProcessBackboneElement>,
    /// Step is nested workflow
    #[fhir(name="workflow", min="0", max="1", summary="false", modifier="false")]
    pub workflow: Option<CanonicalDt>,
    /// Step is simple action
    #[fhir(name="operation", min="0", max="1", summary="false", modifier="false")]
    pub operation: Option<ExampleScenarioProcessStepOperationBackboneElement>,
    /// Alternate non-typical step action
    #[fhir(name="alternative", min="0", max="*", summary="false", modifier="false")]
    pub alternative: Option<Vec<ExampleScenarioProcessStepAlternativeBackboneElement>>,
    /// Pause in the flow?
    #[fhir(name="pause", min="0", max="1", summary="false", modifier="false")]
    pub pause: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioProcessStepAlternativeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for alternative
    #[fhir(name="title", min="1", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Human-readable description of option
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Alternative action(s)
    #[fhir(name="step", min="0", max="*", summary="false", modifier="false")]
    pub step: Option<Vec<ExampleScenarioProcessStepBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ExampleScenarioProcessStepOperationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Kind of action
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<Coding>,
    /// Label for step
    #[fhir(name="title", min="1", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Who starts the operation
    #[fhir(name="initiator", min="0", max="1", summary="false", modifier="false")]
    pub initiator: Option<StringDt>,
    /// Who receives the operation
    #[fhir(name="receiver", min="0", max="1", summary="false", modifier="false")]
    pub receiver: Option<StringDt>,
    /// Human-friendly description of the operation
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Initiator stays active?
    #[fhir(name="initiatorActive", min="0", max="1", summary="false", modifier="false")]
    pub initiator_active: Option<BooleanDt>,
    /// Receiver stays active?
    #[fhir(name="receiverActive", min="0", max="1", summary="false", modifier="false")]
    pub receiver_active: Option<BooleanDt>,
    /// Instance transmitted on invocation
    #[fhir(name="request", min="0", max="1", summary="false", modifier="false")]
    pub request: Option<ExampleScenarioInstanceContainedInstanceBackboneElement>,
    /// Instance transmitted on invocation response
    #[fhir(name="response", min="0", max="1", summary="false", modifier="false")]
    pub response: Option<ExampleScenarioInstanceContainedInstanceBackboneElement>,
}


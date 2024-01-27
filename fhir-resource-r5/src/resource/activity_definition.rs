use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ActivityDefinition {
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
    /// Canonical identifier for this activity definition, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Additional identifier for the activity definition
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the activity definition
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary="true", modifier="false")]
    pub version_algorithm: Option<Coding>,
    /// Name for this activity definition (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Name for this activity definition (human friendly)
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// Subordinate title of the activity definition
    #[fhir(name="subtitle", min="0", max="1", summary="false", modifier="false")]
    pub subtitle: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary="true", modifier="false")]
    pub experimental: Option<BooleanDt>,
    /// Type of individual the activity definition is intended for
    #[fhir(name="subject", min="0", max="1", summary="false", modifier="false")]
    pub subject: Option<CanonicalDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary="true", modifier="false")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary="true", modifier="false")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the activity definition
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for activity definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary="true", modifier="false")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this activity definition is defined
    #[fhir(name="purpose", min="0", max="1", summary="false", modifier="false")]
    pub purpose: Option<MarkdownDt>,
    /// Describes the clinical usage of the activity definition
    #[fhir(name="usage", min="0", max="1", summary="false", modifier="false")]
    pub usage: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary="false", modifier="false")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary="false", modifier="false")]
    pub copyright_label: Option<StringDt>,
    /// When the activity definition was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary="false", modifier="false")]
    pub approval_date: Option<DateDt>,
    /// When the activity definition was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary="false", modifier="false")]
    pub last_review_date: Option<DateDt>,
    /// When the activity definition is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary="true", modifier="false")]
    pub effective_period: Option<Period>,
    /// E.g. Education, Treatment, Assessment, etc
    #[fhir(name="topic", min="0", max="*", summary="false", modifier="false")]
    pub topic: Option<Vec<CodeableConcept>>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary="false", modifier="false")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary="false", modifier="false")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary="false", modifier="false")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary="false", modifier="false")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Additional documentation, citations, etc
    #[fhir(name="relatedArtifact", min="0", max="*", summary="false", modifier="false")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Logic used by the activity definition
    #[fhir(name="library", min="0", max="*", summary="false", modifier="false")]
    pub library: Option<Vec<CanonicalDt>>,
    /// Kind of resource
    #[fhir(name="kind", min="0", max="1", summary="true", modifier="false")]
    pub kind: Option<CodeDt>,
    /// What profile the resource needs to conform to
    #[fhir(name="profile", min="0", max="1", summary="false", modifier="false")]
    pub profile: Option<CanonicalDt>,
    /// Detail type of activity
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// proposal | plan | directive | order | original-order | reflex-order | filler-order | instance-order | option
    #[fhir(name="intent", min="0", max="1", summary="false", modifier="false")]
    pub intent: Option<CodeDt>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="false", modifier="false")]
    pub priority: Option<CodeDt>,
    /// True if the activity should not be performed
    #[fhir(name="doNotPerform", min="0", max="1", summary="true", modifier="true")]
    pub do_not_perform: Option<BooleanDt>,
    /// When activity is to occur
    #[fhir(name="timing", min="0", max="1", summary="false", modifier="false")]
    pub timing: Option<Duration>,
    /// Preconditions for service
    #[fhir(name="asNeeded", min="0", max="1", summary="true", modifier="false")]
    pub as_needed: Option<CodeableConcept>,
    /// Where it should happen
    #[fhir(name="location", min="0", max="1", summary="false", modifier="false")]
    pub location: Option<CodeableReference>,
    /// Who should participate in the action
    #[fhir(name="participant", min="0", max="*", summary="false", modifier="false")]
    pub participant: Option<Vec<ActivityDefinitionParticipantBackboneElement>>,
    /// What's administered/supplied
    #[fhir(name="product", min="0", max="1", summary="false", modifier="false")]
    pub product: Option<CodeableConcept>,
    /// How much is administered/consumed/supplied
    #[fhir(name="quantity", min="0", max="1", summary="false", modifier="false")]
    pub quantity: Option<Quantity>,
    /// Detailed dosage instructions
    #[fhir(name="dosage", min="0", max="*", summary="false", modifier="false")]
    pub dosage: Option<Vec<Dosage>>,
    /// What part of body to perform on
    #[fhir(name="bodySite", min="0", max="*", summary="false", modifier="false")]
    pub body_site: Option<Vec<CodeableConcept>>,
    /// What specimens are required to perform this action
    #[fhir(name="specimenRequirement", min="0", max="*", summary="false", modifier="false")]
    pub specimen_requirement: Option<Vec<CanonicalDt>>,
    /// What observations are required to perform this action
    #[fhir(name="observationRequirement", min="0", max="*", summary="false", modifier="false")]
    pub observation_requirement: Option<Vec<CanonicalDt>>,
    /// What observations must be produced by this action
    #[fhir(name="observationResultRequirement", min="0", max="*", summary="false", modifier="false")]
    pub observation_result_requirement: Option<Vec<CanonicalDt>>,
    /// Transform to apply the template
    #[fhir(name="transform", min="0", max="1", summary="false", modifier="false")]
    pub transform: Option<CanonicalDt>,
    /// Dynamic aspects of the definition
    #[fhir(name="dynamicValue", min="0", max="*", summary="false", modifier="false")]
    pub dynamic_value: Option<Vec<ActivityDefinitionDynamicValueBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ActivityDefinitionDynamicValueBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The path to the element to be set dynamically
    #[fhir(name="path", min="1", max="1", summary="false", modifier="false")]
    pub path: Option<StringDt>,
    /// An expression that provides the dynamic value for the customization
    #[fhir(name="expression", min="1", max="1", summary="false", modifier="false")]
    pub expression: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ActivityDefinitionParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// careteam | device | group | healthcareservice | location | organization | patient | practitioner | practitionerrole | relatedperson
    #[fhir(name="type", min="0", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeDt>,
    /// Who or what can participate
    #[fhir(name="typeCanonical", min="0", max="1", summary="false", modifier="false")]
    pub type_canonical: Option<CanonicalDt>,
    /// Who or what can participate
    #[fhir(name="typeReference", min="0", max="1", summary="false", modifier="false")]
    pub type_reference: Option<Reference>,
    /// E.g. Nurse, Surgeon, Parent, etc
    #[fhir(name="role", min="0", max="1", summary="false", modifier="false")]
    pub role: Option<CodeableConcept>,
    /// E.g. Author, Reviewer, Witness, etc
    #[fhir(name="function", min="0", max="1", summary="false", modifier="false")]
    pub function: Option<CodeableConcept>,
}


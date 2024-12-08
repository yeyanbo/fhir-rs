use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ImplementationGuide {
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
    /// Canonical identifier for this implementation guide, represented as a URI (globally unique)
    #[fhir(name="url", min="1", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the implementation guide (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the implementation guide
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this implementation guide (computer friendly)
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this implementation guide (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice="")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice="")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice="")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the implementation guide
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for implementation guide (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this implementation guide is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// NPM Package name for IG
    #[fhir(name="packageId", min="1", max="1", summary=true, modifier=false, choice="")]
    pub package_id: Option<IdDt>,
    /// SPDX license code for this IG (or not-open-source)
    #[fhir(name="license", min="0", max="1", summary=true, modifier=false, choice="")]
    pub license: Option<CodeDt>,
    /// FHIR Version(s) this Implementation Guide targets
    #[fhir(name="fhirVersion", min="1", max="*", summary=true, modifier=false, choice="")]
    pub fhir_version: Option<Vec<CodeDt>>,
    /// Another Implementation guide this depends on
    #[fhir(name="dependsOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub depends_on: Option<Vec<ImplementationGuideDependsOnBackboneElement>>,
    /// Profiles that apply globally
    #[fhir(name="global", min="0", max="*", summary=true, modifier=false, choice="")]
    pub global: Option<Vec<ImplementationGuideGlobalBackboneElement>>,
    /// Information needed to build the IG
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub definition: Option<ImplementationGuideDefinitionBackboneElement>,
    /// Information about an assembled IG
    #[fhir(name="manifest", min="0", max="1", summary=false, modifier=false, choice="")]
    pub manifest: Option<ImplementationGuideManifestBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideManifestBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location of rendered implementation guide
    #[fhir(name="rendering", min="0", max="1", summary=true, modifier=false, choice="")]
    pub rendering: Option<UrlDt>,
    /// Resource in the implementation guide
    #[fhir(name="resource", min="1", max="*", summary=true, modifier=false, choice="")]
    pub resource: Option<Vec<ImplementationGuideManifestResourceBackboneElement>>,
    /// HTML page within the parent IG
    #[fhir(name="page", min="0", max="*", summary=false, modifier=false, choice="")]
    pub page: Option<Vec<ImplementationGuideManifestPageBackboneElement>>,
    /// Image within the IG
    #[fhir(name="image", min="0", max="*", summary=false, modifier=false, choice="")]
    pub image: Option<Vec<StringDt>>,
    /// Additional linkable file in IG
    #[fhir(name="other", min="0", max="*", summary=false, modifier=false, choice="")]
    pub other: Option<Vec<StringDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideManifestResourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location of the resource
    #[fhir(name="reference", min="1", max="1", summary=true, modifier=false, choice="")]
    pub reference: Option<Reference>,
    /// Is this an example
    #[fhir(name="isExample", min="0", max="1", summary=false, modifier=false, choice="")]
    pub is_example: Option<BooleanDt>,
    /// Profile(s) this is an example of
    #[fhir(name="profile", min="0", max="*", summary=false, modifier=false, choice="")]
    pub profile: Option<Vec<CanonicalDt>>,
    /// Relative path for page in IG
    #[fhir(name="relativePath", min="0", max="1", summary=false, modifier=false, choice="")]
    pub relative_path: Option<UrlDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideManifestPageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// HTML page name
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Title of the page, for references
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// Anchor available on the page
    #[fhir(name="anchor", min="0", max="*", summary=false, modifier=false, choice="")]
    pub anchor: Option<Vec<StringDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDefinitionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Grouping used to present related resources in the IG
    #[fhir(name="grouping", min="0", max="*", summary=false, modifier=false, choice="")]
    pub grouping: Option<Vec<ImplementationGuideDefinitionGroupingBackboneElement>>,
    /// Resource in the implementation guide
    #[fhir(name="resource", min="0", max="*", summary=false, modifier=false, choice="")]
    pub resource: Option<Vec<ImplementationGuideDefinitionResourceBackboneElement>>,
    /// Page/Section in the Guide
    #[fhir(name="page", min="0", max="1", summary=false, modifier=false, choice="")]
    pub page: Option<ImplementationGuideDefinitionPageBackboneElement>,
    /// Defines how IG is built by tools
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub parameter: Option<Vec<ImplementationGuideDefinitionParameterBackboneElement>>,
    /// A template for building resources
    #[fhir(name="template", min="0", max="*", summary=false, modifier=false, choice="")]
    pub template: Option<Vec<ImplementationGuideDefinitionTemplateBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDefinitionGroupingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Descriptive name for the package
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Human readable text describing the package
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDefinitionTemplateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of template specified
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// The source location for the template
    #[fhir(name="source", min="1", max="1", summary=false, modifier=false, choice="")]
    pub source: Option<StringDt>,
    /// The scope in which the template applies
    #[fhir(name="scope", min="0", max="1", summary=false, modifier=false, choice="")]
    pub scope: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDefinitionPageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Source for page
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false, choice="")]
    pub source: Option<MarkdownDt>,
    /// Name of the page when published
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<UrlDt>,
    /// Short title shown for navigational assistance
    #[fhir(name="title", min="1", max="1", summary=false, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// html | markdown | xml | generated
    #[fhir(name="generation", min="1", max="1", summary=false, modifier=false, choice="")]
    pub generation: Option<CodeDt>,
    /// Nested Pages / Sections
    #[fhir(name="page", min="0", max="*", summary=false, modifier=false, choice="")]
    pub page: Option<Vec<ImplementationGuideDefinitionPageBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDefinitionResourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location of the resource
    #[fhir(name="reference", min="1", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<Reference>,
    /// Versions this applies to (if different to IG)
    #[fhir(name="fhirVersion", min="0", max="*", summary=false, modifier=false, choice="")]
    pub fhir_version: Option<Vec<CodeDt>>,
    /// Human readable name for the resource
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Reason why included in guide
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Is this an example
    #[fhir(name="isExample", min="0", max="1", summary=false, modifier=false, choice="")]
    pub is_example: Option<BooleanDt>,
    /// Profile(s) this is an example of
    #[fhir(name="profile", min="0", max="*", summary=false, modifier=false, choice="")]
    pub profile: Option<Vec<CanonicalDt>>,
    /// Grouping this is part of
    #[fhir(name="groupingId", min="0", max="1", summary=false, modifier=false, choice="")]
    pub grouping_id: Option<IdDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDefinitionParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that identifies parameter
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<Coding>,
    /// Value for named type
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideDependsOnBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identity of the IG that this depends on
    #[fhir(name="uri", min="1", max="1", summary=true, modifier=false, choice="")]
    pub uri: Option<CanonicalDt>,
    /// NPM Package name for IG this depends on
    #[fhir(name="packageId", min="0", max="1", summary=true, modifier=false, choice="")]
    pub package_id: Option<IdDt>,
    /// Version of the IG
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// Why dependency exists
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reason: Option<MarkdownDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImplementationGuideGlobalBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type this profile applies to
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// Profile that all resources must conform to
    #[fhir(name="profile", min="1", max="1", summary=true, modifier=false, choice="")]
    pub profile: Option<CanonicalDt>,
}


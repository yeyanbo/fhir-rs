use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CapabilityStatement {
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
    /// Canonical identifier for this capability statement, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub url: Option<UriDt>,
    /// Additional identifier for the CapabilityStatement (business identifier)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the capability statement
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this capability statement (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Name for this capability statement (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the capability statement
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for capability statement (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this capability statement is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub copyright_label: Option<StringDt>,
    /// instance | capability | requirements
    #[fhir(name="kind", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub kind: Option<CodeDt>,
    /// Canonical URL of another capability statement this implements
    #[fhir(name="instantiates", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub instantiates: Option<Vec<CanonicalDt>>,
    /// Canonical URL of another capability statement this adds to
    #[fhir(name="imports", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub imports: Option<Vec<CanonicalDt>>,
    /// Software that is covered by this capability statement
    #[fhir(name="software", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub software: Option<CapabilityStatementSoftwareBackboneElement>,
    /// If this describes a specific instance
    #[fhir(name="implementation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub implementation: Option<CapabilityStatementImplementationBackboneElement>,
    /// FHIR Version the system supports
    #[fhir(name="fhirVersion", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub fhir_version: Option<CodeDt>,
    /// formats supported (xml | json | ttl | mime type)
    #[fhir(name="format", min="1", max="*", summary=true, modifier=false, choice=false)]
    pub format: Option<Vec<CodeDt>>,
    /// Patch formats supported
    #[fhir(name="patchFormat", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub patch_format: Option<Vec<CodeDt>>,
    /// Languages supported
    #[fhir(name="acceptLanguage", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub accept_language: Option<Vec<CodeDt>>,
    /// Implementation guides supported
    #[fhir(name="implementationGuide", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub implementation_guide: Option<Vec<CanonicalDt>>,
    /// If the endpoint is a RESTful one
    #[fhir(name="rest", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub rest: Option<Vec<CapabilityStatementRestBackboneElement>>,
    /// If messaging is supported
    #[fhir(name="messaging", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub messaging: Option<Vec<CapabilityStatementMessagingBackboneElement>>,
    /// Document definition
    #[fhir(name="document", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub document: Option<Vec<CapabilityStatementDocumentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementDocumentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// producer | consumer
    #[fhir(name="mode", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub mode: Option<CodeDt>,
    /// Description of document support
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
    /// Constraint on the resources used in the document
    #[fhir(name="profile", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub profile: Option<CanonicalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementSoftwareBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A name the software is known by
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Version covered by this statement
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub version: Option<StringDt>,
    /// Date this version was released
    #[fhir(name="releaseDate", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub release_date: Option<DateTimeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// client | server
    #[fhir(name="mode", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub mode: Option<CodeDt>,
    /// General description of implementation
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
    /// Information about security of implementation
    #[fhir(name="security", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub security: Option<CapabilityStatementRestSecurityBackboneElement>,
    /// Resource served on the REST interface
    #[fhir(name="resource", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub resource: Option<Vec<CapabilityStatementRestResourceBackboneElement>>,
    /// What operations are supported?
    #[fhir(name="interaction", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub interaction: Option<Vec<CapabilityStatementRestInteractionBackboneElement>>,
    /// Search parameters for searching all resources
    #[fhir(name="searchParam", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParamBackboneElement>>,
    /// Definition of a system level operation
    #[fhir(name="operation", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub operation: Option<Vec<CapabilityStatementRestResourceOperationBackboneElement>>,
    /// Compartments served/used by system
    #[fhir(name="compartment", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub compartment: Option<Vec<CanonicalDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestInteractionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// transaction | batch | search-system | history-system
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeDt>,
    /// Anything special about operation behavior
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestResourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A resource type that is supported
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub type_: Option<CodeDt>,
    /// System-wide profile
    #[fhir(name="profile", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub profile: Option<CanonicalDt>,
    /// Use-case specific profiles
    #[fhir(name="supportedProfile", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub supported_profile: Option<Vec<CanonicalDt>>,
    /// Additional information about the use of the resource type
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
    /// What operations are supported?
    #[fhir(name="interaction", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub interaction: Option<Vec<CapabilityStatementRestResourceInteractionBackboneElement>>,
    /// no-version | versioned | versioned-update
    #[fhir(name="versioning", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub versioning: Option<CodeDt>,
    /// Whether vRead can return past versions
    #[fhir(name="readHistory", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub read_history: Option<BooleanDt>,
    /// If update can commit to a new identity
    #[fhir(name="updateCreate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub update_create: Option<BooleanDt>,
    /// If allows/uses conditional create
    #[fhir(name="conditionalCreate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub conditional_create: Option<BooleanDt>,
    /// not-supported | modified-since | not-match | full-support
    #[fhir(name="conditionalRead", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub conditional_read: Option<CodeDt>,
    /// If allows/uses conditional update
    #[fhir(name="conditionalUpdate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub conditional_update: Option<BooleanDt>,
    /// If allows/uses conditional patch
    #[fhir(name="conditionalPatch", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub conditional_patch: Option<BooleanDt>,
    /// not-supported | single | multiple - how conditional delete is supported
    #[fhir(name="conditionalDelete", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub conditional_delete: Option<CodeDt>,
    /// literal | logical | resolves | enforced | local
    #[fhir(name="referencePolicy", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub reference_policy: Option<Vec<CodeDt>>,
    /// _include values supported by the server
    #[fhir(name="searchInclude", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub search_include: Option<Vec<StringDt>>,
    /// _revinclude values supported by the server
    #[fhir(name="searchRevInclude", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub search_rev_include: Option<Vec<StringDt>>,
    /// Search parameters supported by implementation
    #[fhir(name="searchParam", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub search_param: Option<Vec<CapabilityStatementRestResourceSearchParamBackboneElement>>,
    /// Definition of a resource operation
    #[fhir(name="operation", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub operation: Option<Vec<CapabilityStatementRestResourceOperationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestResourceSearchParamBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name for parameter in search url
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Source of definition for parameter
    #[fhir(name="definition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub definition: Option<CanonicalDt>,
    /// number | date | string | token | reference | composite | quantity | uri | special
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeDt>,
    /// Server-specific usage
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestResourceOperationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name by which the operation/query is invoked
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// The defined operation/query
    #[fhir(name="definition", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub definition: Option<CanonicalDt>,
    /// Specific details about operation behavior
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestResourceInteractionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// read | vread | update | patch | delete | history-instance | history-type | create | search-type
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeDt>,
    /// Anything special about operation behavior
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementRestSecurityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Adds CORS Headers (http://enable-cors.org/)
    #[fhir(name="cors", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub cors: Option<BooleanDt>,
    /// OAuth | SMART-on-FHIR | NTLM | Basic | Kerberos | Certificates
    #[fhir(name="service", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub service: Option<Vec<CodeableConcept>>,
    /// General description of how security works
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementImplementationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Describes this specific instance
    #[fhir(name="description", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Base URL for the installation
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub url: Option<UrlDt>,
    /// Organization that manages the data
    #[fhir(name="custodian", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub custodian: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementMessagingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Where messages should be sent
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub endpoint: Option<Vec<CapabilityStatementMessagingEndpointBackboneElement>>,
    /// Reliable Message Cache Length (min)
    #[fhir(name="reliableCache", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub reliable_cache: Option<UnsignedIntDt>,
    /// Messaging interface behavior details
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub documentation: Option<MarkdownDt>,
    /// Messages supported by this system
    #[fhir(name="supportedMessage", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub supported_message: Option<Vec<CapabilityStatementMessagingSupportedMessageBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementMessagingEndpointBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// http | ftp | mllp +
    #[fhir(name="protocol", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub protocol: Option<Coding>,
    /// Network address or identifier of the end-point
    #[fhir(name="address", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub address: Option<UrlDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CapabilityStatementMessagingSupportedMessageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// sender | receiver
    #[fhir(name="mode", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub mode: Option<CodeDt>,
    /// Message supported by this system
    #[fhir(name="definition", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub definition: Option<CanonicalDt>,
}


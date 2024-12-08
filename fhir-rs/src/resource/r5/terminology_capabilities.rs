use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct TerminologyCapabilities {
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
    /// Canonical identifier for this terminology capabilities, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the terminology capabilities
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the terminology capabilities
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this terminology capabilities (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this terminology capabilities (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// For testing purposes, not real usage
    #[fhir(name="experimental", min="0", max="1", summary=true, modifier=false, choice="")]
    pub experimental: Option<BooleanDt>,
    /// Date last changed
    #[fhir(name="date", min="1", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice="")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice="")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the terminology capabilities
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for terminology capabilities (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this terminology capabilities is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=true, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// instance | capability | requirements
    #[fhir(name="kind", min="1", max="1", summary=true, modifier=false, choice="")]
    pub kind: Option<CodeDt>,
    /// Software that is covered by this terminology capability statement
    #[fhir(name="software", min="0", max="1", summary=true, modifier=false, choice="")]
    pub software: Option<TerminologyCapabilitiesSoftwareBackboneElement>,
    /// If this describes a specific instance
    #[fhir(name="implementation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub implementation: Option<TerminologyCapabilitiesImplementationBackboneElement>,
    /// Whether lockedDate is supported
    #[fhir(name="lockedDate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub locked_date: Option<BooleanDt>,
    /// A code system supported by the server
    #[fhir(name="codeSystem", min="0", max="*", summary=false, modifier=false, choice="")]
    pub code_system: Option<Vec<TerminologyCapabilitiesCodeSystemBackboneElement>>,
    /// Information about the [ValueSet/$expand](valueset-operation-expand.html) operation
    #[fhir(name="expansion", min="0", max="1", summary=false, modifier=false, choice="")]
    pub expansion: Option<TerminologyCapabilitiesExpansionBackboneElement>,
    /// in-compose | in-expansion | in-compose-or-expansion
    #[fhir(name="codeSearch", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code_search: Option<CodeDt>,
    /// Information about the [ValueSet/$validate-code](valueset-operation-validate-code.html) operation
    #[fhir(name="validateCode", min="0", max="1", summary=false, modifier=false, choice="")]
    pub validate_code: Option<TerminologyCapabilitiesValidateCodeBackboneElement>,
    /// Information about the [ConceptMap/$translate](conceptmap-operation-translate.html) operation
    #[fhir(name="translation", min="0", max="1", summary=false, modifier=false, choice="")]
    pub translation: Option<TerminologyCapabilitiesTranslationBackboneElement>,
    /// Information about the [ConceptMap/$closure](conceptmap-operation-closure.html) operation
    #[fhir(name="closure", min="0", max="1", summary=false, modifier=false, choice="")]
    pub closure: Option<TerminologyCapabilitiesClosureBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesImplementationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Describes this specific instance
    #[fhir(name="description", min="1", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// Base URL for the implementation
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UrlDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesCodeSystemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Canonical identifier for the code system, represented as a URI
    #[fhir(name="uri", min="0", max="1", summary=false, modifier=false, choice="")]
    pub uri: Option<CanonicalDt>,
    /// Version of Code System supported
    #[fhir(name="version", min="0", max="*", summary=false, modifier=false, choice="")]
    pub version: Option<Vec<TerminologyCapabilitiesCodeSystemVersionBackboneElement>>,
    /// not-present | example | fragment | complete | supplement
    #[fhir(name="content", min="1", max="1", summary=true, modifier=false, choice="")]
    pub content: Option<CodeDt>,
    /// Whether subsumption is supported
    #[fhir(name="subsumption", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subsumption: Option<BooleanDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesCodeSystemVersionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Version identifier for this version
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<StringDt>,
    /// If this is the default version for this code system
    #[fhir(name="isDefault", min="0", max="1", summary=true, modifier=false, choice="")]
    pub is_default: Option<BooleanDt>,
    /// If compositional grammar is supported
    #[fhir(name="compositional", min="0", max="1", summary=false, modifier=false, choice="")]
    pub compositional: Option<BooleanDt>,
    /// Language Displays supported
    #[fhir(name="language", min="0", max="*", summary=false, modifier=false, choice="")]
    pub language: Option<Vec<CodeDt>>,
    /// Filter Properties supported
    #[fhir(name="filter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub filter: Option<Vec<TerminologyCapabilitiesCodeSystemVersionFilterBackboneElement>>,
    /// Properties supported for $lookup
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false, choice="")]
    pub property: Option<Vec<CodeDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesCodeSystemVersionFilterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code of the property supported
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Operations supported for the property
    #[fhir(name="op", min="1", max="*", summary=false, modifier=false, choice="")]
    pub op: Option<Vec<CodeDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesSoftwareBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A name the software is known by
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Version covered by this statement
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesValidateCodeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether translations are validated
    #[fhir(name="translations", min="1", max="1", summary=false, modifier=false, choice="")]
    pub translations: Option<BooleanDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesTranslationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether the client must identify the map
    #[fhir(name="needsMap", min="1", max="1", summary=false, modifier=false, choice="")]
    pub needs_map: Option<BooleanDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesExpansionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Whether the server can return nested value sets
    #[fhir(name="hierarchical", min="0", max="1", summary=false, modifier=false, choice="")]
    pub hierarchical: Option<BooleanDt>,
    /// Whether the server supports paging on expansion
    #[fhir(name="paging", min="0", max="1", summary=false, modifier=false, choice="")]
    pub paging: Option<BooleanDt>,
    /// Allow request for incomplete expansions?
    #[fhir(name="incomplete", min="0", max="1", summary=false, modifier=false, choice="")]
    pub incomplete: Option<BooleanDt>,
    /// Supported expansion parameter
    #[fhir(name="parameter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub parameter: Option<Vec<TerminologyCapabilitiesExpansionParameterBackboneElement>>,
    /// Documentation about text searching works
    #[fhir(name="textFilter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text_filter: Option<MarkdownDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesExpansionParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of the supported expansion parameter
    #[fhir(name="name", min="1", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<CodeDt>,
    /// Description of support for parameter
    #[fhir(name="documentation", min="0", max="1", summary=false, modifier=false, choice="")]
    pub documentation: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct TerminologyCapabilitiesClosureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// If cross-system closure is supported
    #[fhir(name="translation", min="0", max="1", summary=false, modifier=false, choice="")]
    pub translation: Option<BooleanDt>,
}


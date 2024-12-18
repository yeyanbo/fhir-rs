use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct StructureDefinition {
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
    /// Canonical identifier for this structure definition, represented as a URI (globally unique)
    #[fhir(name="url", min="1", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Additional identifier for the structure definition
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the structure definition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version_algorithm: Option<Coding>,
    /// Name for this structure definition (computer friendly)
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Name for this structure definition (human friendly)
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
    /// Natural language description of the structure definition
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for structure definition (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false, choice="")]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this structure definition is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false, choice="")]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s)
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright_label: Option<StringDt>,
    /// Assist with indexing and finding
    #[fhir(name="keyword", min="0", max="*", summary=true, modifier=false, choice="")]
    pub keyword: Option<Vec<Coding>>,
    /// FHIR Version this StructureDefinition targets
    #[fhir(name="fhirVersion", min="0", max="1", summary=true, modifier=false, choice="")]
    pub fhir_version: Option<CodeDt>,
    /// External specification that the content is mapped to
    #[fhir(name="mapping", min="0", max="*", summary=false, modifier=false, choice="")]
    pub mapping: Option<Vec<StructureDefinitionMappingBackboneElement>>,
    /// primitive-type | complex-type | resource | logical
    #[fhir(name="kind", min="1", max="1", summary=true, modifier=false, choice="")]
    pub kind: Option<CodeDt>,
    /// Whether the structure is abstract
    #[fhir(name="abstract", min="1", max="1", summary=true, modifier=false, choice="")]
    pub abstract_: Option<BooleanDt>,
    /// If an extension, where it can be used in instances
    #[fhir(name="context", min="0", max="*", summary=true, modifier=false, choice="")]
    pub context: Option<Vec<StructureDefinitionContextBackboneElement>>,
    /// FHIRPath invariants - when the extension can be used
    #[fhir(name="contextInvariant", min="0", max="*", summary=true, modifier=false, choice="")]
    pub context_invariant: Option<Vec<StringDt>>,
    /// Type defined or constrained by this structure
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<UriDt>,
    /// Definition that this type is constrained/specialized from
    #[fhir(name="baseDefinition", min="0", max="1", summary=true, modifier=false, choice="")]
    pub base_definition: Option<CanonicalDt>,
    /// specialization | constraint - How relates to base definition
    #[fhir(name="derivation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub derivation: Option<CodeDt>,
    /// Snapshot view of the structure
    #[fhir(name="snapshot", min="0", max="1", summary=false, modifier=false, choice="")]
    pub snapshot: Option<StructureDefinitionSnapshotBackboneElement>,
    /// Differential view of the structure
    #[fhir(name="differential", min="0", max="1", summary=false, modifier=false, choice="")]
    pub differential: Option<StructureDefinitionDifferentialBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct StructureDefinitionSnapshotBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Definition of elements in the resource (if no StructureDefinition)
    #[fhir(name="element", min="1", max="*", summary=false, modifier=false, choice="")]
    pub element: Option<Vec<ElementDefinition>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct StructureDefinitionContextBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// fhirpath | element | extension
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// Where the extension can be used in instances
    #[fhir(name="expression", min="1", max="1", summary=true, modifier=false, choice="")]
    pub expression: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct StructureDefinitionMappingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Internal id when this mapping is used
    #[fhir(name="identity", min="1", max="1", summary=false, modifier=false, choice="")]
    pub identity: Option<IdDt>,
    /// Identifies what this mapping refers to
    #[fhir(name="uri", min="0", max="1", summary=false, modifier=false, choice="")]
    pub uri: Option<UriDt>,
    /// Names what this mapping refers to
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Versions, Issues, Scope limitations etc
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub comment: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct StructureDefinitionDifferentialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Definition of elements in the resource (if no StructureDefinition)
    #[fhir(name="element", min="1", max="*", summary=false, modifier=false, choice="")]
    pub element: Option<Vec<ElementDefinition>>,
}


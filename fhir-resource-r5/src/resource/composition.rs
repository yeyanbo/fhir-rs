use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Composition {
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
    /// Canonical identifier for this Composition, represented as a URI (globally unique)
    #[fhir(name="url", min="0", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// Version-independent identifier for the Composition
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// An explicitly assigned identifer of a variation of the content in the Composition
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Kind of composition (LOINC if possible)
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Categorization of Composition
    #[fhir(name="category", min="0", max="*", summary="true", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Who and/or what the composition is about
    #[fhir(name="subject", min="0", max="*", summary="true", modifier="false")]
    pub subject: Option<Vec<Reference>>,
    /// Context of the Composition
    #[fhir(name="encounter", min="0", max="1", summary="true", modifier="false")]
    pub encounter: Option<Reference>,
    /// Composition editing time
    #[fhir(name="date", min="1", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary="true", modifier="false")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Who and/or what authored the composition
    #[fhir(name="author", min="1", max="*", summary="true", modifier="false")]
    pub author: Option<Vec<Reference>>,
    /// Name for this Composition (computer friendly)
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Human Readable name/title
    #[fhir(name="title", min="1", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// For any additional notes
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Attests to accuracy of composition
    #[fhir(name="attester", min="0", max="*", summary="false", modifier="false")]
    pub attester: Option<Vec<CompositionAttesterBackboneElement>>,
    /// Organization which maintains the composition
    #[fhir(name="custodian", min="0", max="1", summary="true", modifier="false")]
    pub custodian: Option<Reference>,
    /// Relationships to other compositions/documents
    #[fhir(name="relatesTo", min="0", max="*", summary="false", modifier="false")]
    pub relates_to: Option<Vec<RelatedArtifact>>,
    /// The clinical service(s) being documented
    #[fhir(name="event", min="0", max="*", summary="true", modifier="false")]
    pub event: Option<Vec<CompositionEventBackboneElement>>,
    /// Composition is broken into sections
    #[fhir(name="section", min="0", max="*", summary="false", modifier="false")]
    pub section: Option<Vec<CompositionSectionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CompositionEventBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The period covered by the documentation
    #[fhir(name="period", min="0", max="1", summary="true", modifier="false")]
    pub period: Option<Period>,
    /// The event(s) being documented, as code(s), reference(s), or both
    #[fhir(name="detail", min="0", max="*", summary="true", modifier="false")]
    pub detail: Option<Vec<CodeableReference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CompositionSectionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for section (e.g. for ToC)
    #[fhir(name="title", min="0", max="1", summary="false", modifier="false")]
    pub title: Option<StringDt>,
    /// Classification of section (recommended)
    #[fhir(name="code", min="0", max="1", summary="false", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Who and/or what authored the section
    #[fhir(name="author", min="0", max="*", summary="false", modifier="false")]
    pub author: Option<Vec<Reference>>,
    /// Who/what the section is about, when it is not about the subject of composition
    #[fhir(name="focus", min="0", max="1", summary="false", modifier="false")]
    pub focus: Option<Reference>,
    /// Text summary of the section, for human interpretation
    #[fhir(name="text", min="0", max="1", summary="false", modifier="false")]
    pub text: Option<Narrative>,
    /// Order of section entries
    #[fhir(name="orderedBy", min="0", max="1", summary="false", modifier="false")]
    pub ordered_by: Option<CodeableConcept>,
    /// A reference to data that supports this section
    #[fhir(name="entry", min="0", max="*", summary="false", modifier="false")]
    pub entry: Option<Vec<Reference>>,
    /// Why the section is empty
    #[fhir(name="emptyReason", min="0", max="1", summary="false", modifier="false")]
    pub empty_reason: Option<CodeableConcept>,
    /// Nested Section
    #[fhir(name="section", min="0", max="*", summary="false", modifier="false")]
    pub section: Option<Vec<CompositionSectionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CompositionAttesterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// personal | professional | legal | official
    #[fhir(name="mode", min="1", max="1", summary="false", modifier="false")]
    pub mode: Option<CodeableConcept>,
    /// When the composition was attested
    #[fhir(name="time", min="0", max="1", summary="false", modifier="false")]
    pub time: Option<DateTimeDt>,
    /// Who attested the composition
    #[fhir(name="party", min="0", max="1", summary="false", modifier="false")]
    pub party: Option<Reference>,
}


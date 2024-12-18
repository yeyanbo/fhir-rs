use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct EvidenceReport {
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
    /// Canonical identifier for this EvidenceReport, represented as a globally unique URI
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// The context that the content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false, choice="")]
    pub use_context: Option<Vec<UsageContext>>,
    /// Unique identifier for the evidence report
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Identifiers for articles that may relate to more than one evidence report
    #[fhir(name="relatedIdentifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub related_identifier: Option<Vec<Identifier>>,
    /// Citation for this report
    #[fhir(name="citeAs", min="0", max="1", summary=false, modifier=false, choice="")]
    pub cite_as: Option<MarkdownDt>,
    /// Kind of report
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Used for footnotes and annotations
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Link, description or reference to artifact associated with the report
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// Focus of the report
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<EvidenceReportSubjectBackboneElement>,
    /// Name of the publisher/steward (organization or individual)
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false, choice="")]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false, choice="")]
    pub contact: Option<Vec<ContactDetail>>,
    /// Who authored the content
    #[fhir(name="author", min="0", max="*", summary=true, modifier=false, choice="")]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the content
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false, choice="")]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the content
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the content
    #[fhir(name="endorser", min="0", max="*", summary=true, modifier=false, choice="")]
    pub endorser: Option<Vec<ContactDetail>>,
    /// Relationships to other compositions/documents
    #[fhir(name="relatesTo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub relates_to: Option<Vec<EvidenceReportRelatesToBackboneElement>>,
    /// Composition is broken into sections
    #[fhir(name="section", min="0", max="*", summary=false, modifier=false, choice="")]
    pub section: Option<Vec<EvidenceReportSectionBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EvidenceReportSubjectBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Characteristic
    #[fhir(name="characteristic", min="0", max="*", summary=false, modifier=false, choice="")]
    pub characteristic: Option<Vec<EvidenceReportSubjectCharacteristicBackboneElement>>,
    /// Footnotes and/or explanatory notes
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EvidenceReportSubjectCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Characteristic code
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Characteristic value
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Range>,
    /// Is used to express not the characteristic
    #[fhir(name="exclude", min="0", max="1", summary=false, modifier=false, choice="")]
    pub exclude: Option<BooleanDt>,
    /// Timeframe for the characteristic
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EvidenceReportRelatesToBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// replaces | amends | appends | transforms | replacedWith | amendedWith | appendedWith | transformedWith
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeDt>,
    /// Target of the relationship
    #[fhir(name="target", min="1", max="1", summary=false, modifier=false, choice="")]
    pub target: Option<EvidenceReportRelatesToTargetBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EvidenceReportRelatesToTargetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Target of the relationship URL
    #[fhir(name="url", min="0", max="1", summary=false, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Target of the relationship Identifier
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Target of the relationship Display
    #[fhir(name="display", min="0", max="1", summary=false, modifier=false, choice="")]
    pub display: Option<MarkdownDt>,
    /// Target of the relationship Resource reference
    #[fhir(name="resource", min="0", max="1", summary=false, modifier=false, choice="")]
    pub resource: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EvidenceReportSectionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Label for section (e.g. for ToC)
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// Classification of section (recommended)
    #[fhir(name="focus", min="0", max="1", summary=false, modifier=false, choice="")]
    pub focus: Option<CodeableConcept>,
    /// Classification of section by Resource
    #[fhir(name="focusReference", min="0", max="1", summary=false, modifier=false, choice="")]
    pub focus_reference: Option<Reference>,
    /// Who and/or what authored the section
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false, choice="")]
    pub author: Option<Vec<Reference>>,
    /// Text summary of the section, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<Narrative>,
    /// working | snapshot | changes
    #[fhir(name="mode", min="0", max="1", summary=false, modifier=false, choice="")]
    pub mode: Option<CodeDt>,
    /// Order of section entries
    #[fhir(name="orderedBy", min="0", max="1", summary=false, modifier=false, choice="")]
    pub ordered_by: Option<CodeableConcept>,
    /// Extensible classifiers as content
    #[fhir(name="entryClassifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub entry_classifier: Option<Vec<CodeableConcept>>,
    /// Reference to resources as content
    #[fhir(name="entryReference", min="0", max="*", summary=false, modifier=false, choice="")]
    pub entry_reference: Option<Vec<Reference>>,
    /// Quantity as content
    #[fhir(name="entryQuantity", min="0", max="*", summary=false, modifier=false, choice="")]
    pub entry_quantity: Option<Vec<Quantity>>,
    /// Why the section is empty
    #[fhir(name="emptyReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub empty_reason: Option<CodeableConcept>,
    /// Nested Section
    #[fhir(name="section", min="0", max="*", summary=false, modifier=false, choice="")]
    pub section: Option<Vec<EvidenceReportSectionBackboneElement>>,
}


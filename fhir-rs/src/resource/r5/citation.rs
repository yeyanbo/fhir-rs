use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Citation {
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
    /// Canonical identifier for this citation record, represented as a globally unique URI
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false)]
    pub url: Option<UriDt>,
    /// Identifier for the citation record itself
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Business version of the citation record
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// How to compare versions
    #[fhir(name="versionAlgorithm", min="0", max="1", summary=true, modifier=false)]
    pub version_algorithm: Option<Coding>,
    /// Name for this citation record (computer friendly)
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name for this citation record (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
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
    /// The publisher of the citation record, not the publisher of the article or artifact being cited
    #[fhir(name="publisher", min="0", max="1", summary=true, modifier=false)]
    pub publisher: Option<StringDt>,
    /// Contact details for the publisher of the citation record
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false)]
    pub contact: Option<Vec<ContactDetail>>,
    /// Natural language description of the citation
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The context that the citation record content is intended to support
    #[fhir(name="useContext", min="0", max="*", summary=true, modifier=false)]
    pub use_context: Option<Vec<UsageContext>>,
    /// Intended jurisdiction for citation record (if applicable)
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// Why this citation is defined
    #[fhir(name="purpose", min="0", max="1", summary=false, modifier=false)]
    pub purpose: Option<MarkdownDt>,
    /// Use and/or publishing restrictions for the citation record, not for the cited artifact
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
    /// Copyright holder and year(s) for the ciation record, not for the cited artifact
    #[fhir(name="copyrightLabel", min="0", max="1", summary=false, modifier=false)]
    pub copyright_label: Option<StringDt>,
    /// When the citation record was approved by publisher
    #[fhir(name="approvalDate", min="0", max="1", summary=false, modifier=false)]
    pub approval_date: Option<DateDt>,
    /// When the citation record was last reviewed by the publisher
    #[fhir(name="lastReviewDate", min="0", max="1", summary=false, modifier=false)]
    pub last_review_date: Option<DateDt>,
    /// When the citation record is expected to be used
    #[fhir(name="effectivePeriod", min="0", max="1", summary=true, modifier=false)]
    pub effective_period: Option<Period>,
    /// Who authored the citation record
    #[fhir(name="author", min="0", max="*", summary=false, modifier=false)]
    pub author: Option<Vec<ContactDetail>>,
    /// Who edited the citation record
    #[fhir(name="editor", min="0", max="*", summary=false, modifier=false)]
    pub editor: Option<Vec<ContactDetail>>,
    /// Who reviewed the citation record
    #[fhir(name="reviewer", min="0", max="*", summary=false, modifier=false)]
    pub reviewer: Option<Vec<ContactDetail>>,
    /// Who endorsed the citation record
    #[fhir(name="endorser", min="0", max="*", summary=false, modifier=false)]
    pub endorser: Option<Vec<ContactDetail>>,
    /// A human-readable display of key concepts to represent the citation
    #[fhir(name="summary", min="0", max="*", summary=false, modifier=false)]
    pub summary: Option<Vec<CitationSummaryBackboneElement>>,
    /// The assignment to an organizing scheme
    #[fhir(name="classification", min="0", max="*", summary=false, modifier=false)]
    pub classification: Option<Vec<CitationClassificationBackboneElement>>,
    /// Used for general notes and annotations not coded elsewhere
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// The status of the citation record
    #[fhir(name="currentState", min="0", max="*", summary=false, modifier=false)]
    pub current_state: Option<Vec<CodeableConcept>>,
    /// An effective date or period for a status of the citation record
    #[fhir(name="statusDate", min="0", max="*", summary=false, modifier=false)]
    pub status_date: Option<Vec<CitationStatusDateBackboneElement>>,
    /// Artifact related to the citation record
    #[fhir(name="relatedArtifact", min="0", max="*", summary=false, modifier=false)]
    pub related_artifact: Option<Vec<RelatedArtifact>>,
    /// The article or artifact being described
    #[fhir(name="citedArtifact", min="0", max="1", summary=false, modifier=false)]
    pub cited_artifact: Option<CitationCitedArtifactBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationSummaryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Format for display of the citation summary
    #[fhir(name="style", min="0", max="1", summary=false, modifier=false)]
    pub style: Option<CodeableConcept>,
    /// The human-readable display of the citation summary
    #[fhir(name="text", min="1", max="1", summary=true, modifier=false)]
    pub text: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationStatusDateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Classification of the status
    #[fhir(name="activity", min="1", max="1", summary=false, modifier=false)]
    pub activity: Option<CodeableConcept>,
    /// Either occurred or expected
    #[fhir(name="actual", min="0", max="1", summary=false, modifier=false)]
    pub actual: Option<BooleanDt>,
    /// When the status started and/or ended
    #[fhir(name="period", min="1", max="1", summary=false, modifier=false)]
    pub period: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Unique identifier. May include DOI, PMID, PMCID, etc
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Identifier not unique to the cited artifact. May include trial registry identifiers
    #[fhir(name="relatedIdentifier", min="0", max="*", summary=true, modifier=false)]
    pub related_identifier: Option<Vec<Identifier>>,
    /// When the cited artifact was accessed
    #[fhir(name="dateAccessed", min="0", max="1", summary=true, modifier=false)]
    pub date_accessed: Option<DateTimeDt>,
    /// The defined version of the cited artifact
    #[fhir(name="version", min="0", max="1", summary=false, modifier=false)]
    pub version: Option<CitationCitedArtifactVersionBackboneElement>,
    /// The status of the cited artifact
    #[fhir(name="currentState", min="0", max="*", summary=false, modifier=false)]
    pub current_state: Option<Vec<CodeableConcept>>,
    /// An effective date or period for a status of the cited artifact
    #[fhir(name="statusDate", min="0", max="*", summary=false, modifier=false)]
    pub status_date: Option<Vec<CitationCitedArtifactStatusDateBackboneElement>>,
    /// The title details of the article or artifact
    #[fhir(name="title", min="0", max="*", summary=false, modifier=false)]
    pub title: Option<Vec<CitationCitedArtifactTitleBackboneElement>>,
    /// Summary of the article or artifact
    #[fhir(name="abstract", min="0", max="*", summary=false, modifier=false)]
    pub abstract_: Option<Vec<CitationCitedArtifactAbstractBackboneElement>>,
    /// The component of the article or artifact
    #[fhir(name="part", min="0", max="1", summary=false, modifier=false)]
    pub part: Option<CitationCitedArtifactPartBackboneElement>,
    /// The artifact related to the cited artifact
    #[fhir(name="relatesTo", min="0", max="*", summary=false, modifier=false)]
    pub relates_to: Option<Vec<CitationCitedArtifactRelatesToBackboneElement>>,
    /// If multiple, used to represent alternative forms of the article that are not separate citations
    #[fhir(name="publicationForm", min="0", max="*", summary=false, modifier=false)]
    pub publication_form: Option<Vec<CitationCitedArtifactPublicationFormBackboneElement>>,
    /// Used for any URL for the article or artifact cited
    #[fhir(name="webLocation", min="0", max="*", summary=false, modifier=false)]
    pub web_location: Option<Vec<CitationCitedArtifactWebLocationBackboneElement>>,
    /// The assignment to an organizing scheme
    #[fhir(name="classification", min="0", max="*", summary=false, modifier=false)]
    pub classification: Option<Vec<CitationCitedArtifactClassificationBackboneElement>>,
    /// Attribution of authors and other contributors
    #[fhir(name="contributorship", min="0", max="1", summary=false, modifier=false)]
    pub contributorship: Option<CitationCitedArtifactContributorshipBackboneElement>,
    /// Any additional information or content for the article or artifact
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactClassificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of classifier (e.g. publication type, keyword)
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The specific classification value
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Complex or externally created classification
    #[fhir(name="artifactAssessment", min="0", max="*", summary=false, modifier=false)]
    pub artifact_assessment: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactWebLocationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code the reason for different URLs, e.g. abstract and full-text
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// The specific URL
    #[fhir(name="url", min="0", max="1", summary=false, modifier=false)]
    pub url: Option<UriDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactPartBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of component
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The specification of the component
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false)]
    pub value: Option<StringDt>,
    /// The citation for the full article or artifact
    #[fhir(name="baseCitation", min="0", max="1", summary=false, modifier=false)]
    pub base_citation: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactTitleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of title
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Used to express the specific language
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeableConcept>,
    /// The title of the article or artifact
    #[fhir(name="text", min="1", max="1", summary=false, modifier=false)]
    pub text: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactContributorshipBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Indicates if the list includes all authors and/or contributors
    #[fhir(name="complete", min="0", max="1", summary=false, modifier=false)]
    pub complete: Option<BooleanDt>,
    /// An individual entity named as a contributor
    #[fhir(name="entry", min="0", max="*", summary=false, modifier=false)]
    pub entry: Option<Vec<CitationCitedArtifactContributorshipEntryBackboneElement>>,
    /// Used to record a display of the author/contributor list without separate data element for each list member
    #[fhir(name="summary", min="0", max="*", summary=false, modifier=false)]
    pub summary: Option<Vec<CitationCitedArtifactContributorshipSummaryBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactContributorshipEntryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The identity of the individual contributor
    #[fhir(name="contributor", min="1", max="1", summary=false, modifier=false)]
    pub contributor: Option<Reference>,
    /// For citation styles that use initials
    #[fhir(name="forenameInitials", min="0", max="1", summary=false, modifier=false)]
    pub forename_initials: Option<StringDt>,
    /// Organizational affiliation
    #[fhir(name="affiliation", min="0", max="*", summary=false, modifier=false)]
    pub affiliation: Option<Vec<Reference>>,
    /// The specific contribution
    #[fhir(name="contributionType", min="0", max="*", summary=false, modifier=false)]
    pub contribution_type: Option<Vec<CodeableConcept>>,
    /// The role of the contributor (e.g. author, editor, reviewer, funder)
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false)]
    pub role: Option<CodeableConcept>,
    /// Contributions with accounting for time or number
    #[fhir(name="contributionInstance", min="0", max="*", summary=false, modifier=false)]
    pub contribution_instance: Option<Vec<CitationCitedArtifactContributorshipEntryContributionInstanceBackboneElement>>,
    /// Whether the contributor is the corresponding contributor for the role
    #[fhir(name="correspondingContact", min="0", max="1", summary=false, modifier=false)]
    pub corresponding_contact: Option<BooleanDt>,
    /// Ranked order of contribution
    #[fhir(name="rankingOrder", min="0", max="1", summary=false, modifier=false)]
    pub ranking_order: Option<PositiveIntDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactContributorshipEntryContributionInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The specific contribution
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The time that the contribution was made
    #[fhir(name="time", min="0", max="1", summary=false, modifier=false)]
    pub time: Option<DateTimeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactContributorshipSummaryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Such as author list, contributorship statement, funding statement, acknowledgements statement, or conflicts of interest statement
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The format for the display string
    #[fhir(name="style", min="0", max="1", summary=false, modifier=false)]
    pub style: Option<CodeableConcept>,
    /// Used to code the producer or rule for creating the display string
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false)]
    pub source: Option<CodeableConcept>,
    /// The display string for the author list, contributor list, or contributorship statement
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false)]
    pub value: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactStatusDateBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Classification of the status
    #[fhir(name="activity", min="1", max="1", summary=false, modifier=false)]
    pub activity: Option<CodeableConcept>,
    /// Either occurred or expected
    #[fhir(name="actual", min="0", max="1", summary=false, modifier=false)]
    pub actual: Option<BooleanDt>,
    /// When the status started and/or ended
    #[fhir(name="period", min="1", max="1", summary=false, modifier=false)]
    pub period: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactRelatesToBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as | reprint | reprint-of
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Additional classifiers
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Short label
    #[fhir(name="label", min="0", max="1", summary=false, modifier=false)]
    pub label: Option<StringDt>,
    /// Brief description of the related artifact
    #[fhir(name="display", min="0", max="1", summary=false, modifier=false)]
    pub display: Option<StringDt>,
    /// Bibliographic citation for the artifact
    #[fhir(name="citation", min="0", max="1", summary=false, modifier=false)]
    pub citation: Option<MarkdownDt>,
    /// What document is being referenced
    #[fhir(name="document", min="0", max="1", summary=false, modifier=false)]
    pub document: Option<Attachment>,
    /// What artifact is being referenced
    #[fhir(name="resource", min="0", max="1", summary=false, modifier=false)]
    pub resource: Option<CanonicalDt>,
    /// What artifact, if not a conformance resource
    #[fhir(name="resourceReference", min="0", max="1", summary=false, modifier=false)]
    pub resource_reference: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactVersionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The version number or other version identifier
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false)]
    pub value: Option<StringDt>,
    /// Citation for the main version of the cited artifact
    #[fhir(name="baseCitation", min="0", max="1", summary=false, modifier=false)]
    pub base_citation: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactAbstractBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of abstract
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Used to express the specific language
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeableConcept>,
    /// Abstract content
    #[fhir(name="text", min="1", max="1", summary=false, modifier=false)]
    pub text: Option<MarkdownDt>,
    /// Copyright notice for the abstract
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactPublicationFormBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The collection the cited article or artifact is published in
    #[fhir(name="publishedIn", min="0", max="1", summary=false, modifier=false)]
    pub published_in: Option<CitationCitedArtifactPublicationFormPublishedInBackboneElement>,
    /// Internet or Print
    #[fhir(name="citedMedium", min="0", max="1", summary=false, modifier=false)]
    pub cited_medium: Option<CodeableConcept>,
    /// Volume number of journal or other collection in which the article is published
    #[fhir(name="volume", min="0", max="1", summary=false, modifier=false)]
    pub volume: Option<StringDt>,
    /// Issue, part or supplement of journal or other collection in which the article is published
    #[fhir(name="issue", min="0", max="1", summary=false, modifier=false)]
    pub issue: Option<StringDt>,
    /// The date the article was added to the database, or the date the article was released
    #[fhir(name="articleDate", min="0", max="1", summary=false, modifier=false)]
    pub article_date: Option<DateTimeDt>,
    /// Text representation of the date on which the issue of the cited artifact was published
    #[fhir(name="publicationDateText", min="0", max="1", summary=false, modifier=false)]
    pub publication_date_text: Option<StringDt>,
    /// Season in which the cited artifact was published
    #[fhir(name="publicationDateSeason", min="0", max="1", summary=false, modifier=false)]
    pub publication_date_season: Option<StringDt>,
    /// The date the article was last revised or updated in the database
    #[fhir(name="lastRevisionDate", min="0", max="1", summary=false, modifier=false)]
    pub last_revision_date: Option<DateTimeDt>,
    /// Language(s) in which this form of the article is published
    #[fhir(name="language", min="0", max="*", summary=false, modifier=false)]
    pub language: Option<Vec<CodeableConcept>>,
    /// Entry number or identifier for inclusion in a database
    #[fhir(name="accessionNumber", min="0", max="1", summary=false, modifier=false)]
    pub accession_number: Option<StringDt>,
    /// Used for full display of pagination
    #[fhir(name="pageString", min="0", max="1", summary=false, modifier=false)]
    pub page_string: Option<StringDt>,
    /// Used for isolated representation of first page
    #[fhir(name="firstPage", min="0", max="1", summary=false, modifier=false)]
    pub first_page: Option<StringDt>,
    /// Used for isolated representation of last page
    #[fhir(name="lastPage", min="0", max="1", summary=false, modifier=false)]
    pub last_page: Option<StringDt>,
    /// Number of pages or screens
    #[fhir(name="pageCount", min="0", max="1", summary=false, modifier=false)]
    pub page_count: Option<StringDt>,
    /// Copyright notice for the full article or artifact
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false)]
    pub copyright: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationCitedArtifactPublicationFormPublishedInBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Kind of container (e.g. Periodical, database, or book)
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Journal identifiers include ISSN, ISO Abbreviation and NLMuniqueID; Book identifiers include ISBN
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Name of the database or title of the book or journal
    #[fhir(name="title", min="0", max="1", summary=false, modifier=false)]
    pub title: Option<StringDt>,
    /// Name of or resource describing the publisher
    #[fhir(name="publisher", min="0", max="1", summary=false, modifier=false)]
    pub publisher: Option<Reference>,
    /// Geographic location of the publisher
    #[fhir(name="publisherLocation", min="0", max="1", summary=false, modifier=false)]
    pub publisher_location: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CitationClassificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of classifier (e.g. publication type, keyword)
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The specific classification value
    #[fhir(name="classifier", min="0", max="*", summary=false, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
}


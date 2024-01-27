use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct DocumentReference {
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
    /// Business identifiers for the document
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// An explicitly assigned identifer of a variation of the content in the DocumentReference
    #[fhir(name="version", min="0", max="1", summary="true", modifier="false")]
    pub version: Option<StringDt>,
    /// Procedure that caused this media to be created
    #[fhir(name="basedOn", min="0", max="*", summary="false", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// current | superseded | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// registered | partial | preliminary | final | amended | corrected | appended | cancelled | entered-in-error | deprecated | unknown
    #[fhir(name="docStatus", min="0", max="1", summary="true", modifier="false")]
    pub doc_status: Option<CodeDt>,
    /// Imaging modality used
    #[fhir(name="modality", min="0", max="*", summary="true", modifier="false")]
    pub modality: Option<Vec<CodeableConcept>>,
    /// Kind of document (LOINC if possible)
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Categorization of document
    #[fhir(name="category", min="0", max="*", summary="true", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Who/what is the subject of the document
    #[fhir(name="subject", min="0", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// Context of the document content
    #[fhir(name="context", min="0", max="*", summary="false", modifier="false")]
    pub context: Option<Vec<Reference>>,
    /// Main clinical acts documented
    #[fhir(name="event", min="0", max="*", summary="false", modifier="false")]
    pub event: Option<Vec<CodeableReference>>,
    /// Body part included
    #[fhir(name="bodySite", min="0", max="*", summary="true", modifier="false")]
    pub body_site: Option<Vec<CodeableReference>>,
    /// Kind of facility where patient was seen
    #[fhir(name="facilityType", min="0", max="1", summary="false", modifier="false")]
    pub facility_type: Option<CodeableConcept>,
    /// Additional details about where the content was created (e.g. clinical specialty)
    #[fhir(name="practiceSetting", min="0", max="1", summary="false", modifier="false")]
    pub practice_setting: Option<CodeableConcept>,
    /// Time of service that is being documented
    #[fhir(name="period", min="0", max="1", summary="true", modifier="false")]
    pub period: Option<Period>,
    /// When this document reference was created
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<InstantDt>,
    /// Who and/or what authored the document
    #[fhir(name="author", min="0", max="*", summary="true", modifier="false")]
    pub author: Option<Vec<Reference>>,
    /// Attests to accuracy of the document
    #[fhir(name="attester", min="0", max="*", summary="false", modifier="false")]
    pub attester: Option<Vec<DocumentReferenceAttesterBackboneElement>>,
    /// Organization which maintains the document
    #[fhir(name="custodian", min="0", max="1", summary="false", modifier="false")]
    pub custodian: Option<Reference>,
    /// Relationships to other documents
    #[fhir(name="relatesTo", min="0", max="*", summary="true", modifier="false")]
    pub relates_to: Option<Vec<DocumentReferenceRelatesToBackboneElement>>,
    /// Human-readable description
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Document security-tags
    #[fhir(name="securityLabel", min="0", max="*", summary="true", modifier="false")]
    pub security_label: Option<Vec<CodeableConcept>>,
    /// Document referenced
    #[fhir(name="content", min="1", max="*", summary="true", modifier="false")]
    pub content: Option<Vec<DocumentReferenceContentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DocumentReferenceRelatesToBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The relationship type with another document
    #[fhir(name="code", min="1", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Target of the relationship
    #[fhir(name="target", min="1", max="1", summary="true", modifier="false")]
    pub target: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DocumentReferenceAttesterBackboneElement {
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
    /// When the document was attested
    #[fhir(name="time", min="0", max="1", summary="false", modifier="false")]
    pub time: Option<DateTimeDt>,
    /// Who attested the document
    #[fhir(name="party", min="0", max="1", summary="false", modifier="false")]
    pub party: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DocumentReferenceContentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Where to access the document
    #[fhir(name="attachment", min="1", max="1", summary="true", modifier="false")]
    pub attachment: Option<Attachment>,
    /// Content profile rules for the document
    #[fhir(name="profile", min="0", max="*", summary="true", modifier="false")]
    pub profile: Option<Vec<DocumentReferenceContentProfileBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct DocumentReferenceContentProfileBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code|uri|canonical
    #[fhir(name="value", min="1", max="1", summary="true", modifier="false")]
    pub value: Option<CanonicalDt>,
}


use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct List {
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
    /// Business identifier
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// current | retired | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// working | snapshot | changes
    #[fhir(name="mode", min="1", max="1", summary="true", modifier="true")]
    pub mode: Option<CodeDt>,
    /// Descriptive name for the list
    #[fhir(name="title", min="0", max="1", summary="true", modifier="false")]
    pub title: Option<StringDt>,
    /// What the purpose of this list is
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// If all resources have the same subject(s)
    #[fhir(name="subject", min="0", max="*", summary="true", modifier="false")]
    pub subject: Option<Vec<Reference>>,
    /// Context in which list created
    #[fhir(name="encounter", min="0", max="1", summary="false", modifier="false")]
    pub encounter: Option<Reference>,
    /// When the list was prepared
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Who and/or what defined the list contents (aka Author)
    #[fhir(name="source", min="0", max="1", summary="true", modifier="false")]
    pub source: Option<Reference>,
    /// What order the list has
    #[fhir(name="orderedBy", min="0", max="1", summary="false", modifier="false")]
    pub ordered_by: Option<CodeableConcept>,
    /// Comments about the list
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
    /// Entries in the list
    #[fhir(name="entry", min="0", max="*", summary="false", modifier="false")]
    pub entry: Option<Vec<ListEntryBackboneElement>>,
    /// Why list is empty
    #[fhir(name="emptyReason", min="0", max="1", summary="false", modifier="false")]
    pub empty_reason: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ListEntryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Status/Workflow information about this item
    #[fhir(name="flag", min="0", max="1", summary="false", modifier="false")]
    pub flag: Option<CodeableConcept>,
    /// If this item is actually marked as deleted
    #[fhir(name="deleted", min="0", max="1", summary="false", modifier="true")]
    pub deleted: Option<BooleanDt>,
    /// When item added to list
    #[fhir(name="date", min="0", max="1", summary="false", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Actual entry
    #[fhir(name="item", min="1", max="1", summary="false", modifier="false")]
    pub item: Option<Reference>,
}


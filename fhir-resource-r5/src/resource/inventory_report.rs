use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct InventoryReport {
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
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business identifier for the report
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | requested | active | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// snapshot | difference
    #[fhir(name="countType", min="1", max="1", summary="true", modifier="true")]
    pub count_type: Option<CodeDt>,
    /// addition | subtraction
    #[fhir(name="operationType", min="0", max="1", summary="true", modifier="false")]
    pub operation_type: Option<CodeableConcept>,
    /// The reason for this count - regular count, ad-hoc count, new arrivals, etc
    #[fhir(name="operationTypeReason", min="0", max="1", summary="true", modifier="false")]
    pub operation_type_reason: Option<CodeableConcept>,
    /// When the report has been submitted
    #[fhir(name="reportedDateTime", min="1", max="1", summary="true", modifier="false")]
    pub reported_date_time: Option<DateTimeDt>,
    /// Who submits the report
    #[fhir(name="reporter", min="0", max="1", summary="false", modifier="false")]
    pub reporter: Option<Reference>,
    /// The period the report refers to
    #[fhir(name="reportingPeriod", min="0", max="1", summary="false", modifier="false")]
    pub reporting_period: Option<Period>,
    /// An inventory listing section (grouped by any of the attributes)
    #[fhir(name="inventoryListing", min="0", max="*", summary="true", modifier="false")]
    pub inventory_listing: Option<Vec<InventoryReportInventoryListingBackboneElement>>,
    /// A note associated with the InventoryReport
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryReportInventoryListingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location of the inventory items
    #[fhir(name="location", min="0", max="1", summary="false", modifier="false")]
    pub location: Option<Reference>,
    /// The status of the items that are being reported
    #[fhir(name="itemStatus", min="0", max="1", summary="true", modifier="false")]
    pub item_status: Option<CodeableConcept>,
    /// The date and time when the items were counted
    #[fhir(name="countingDateTime", min="0", max="1", summary="false", modifier="false")]
    pub counting_date_time: Option<DateTimeDt>,
    /// The item or items in this listing
    #[fhir(name="item", min="0", max="*", summary="true", modifier="false")]
    pub item: Option<Vec<InventoryReportInventoryListingItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryReportInventoryListingItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The inventory category or classification of the items being reported
    #[fhir(name="category", min="0", max="1", summary="true", modifier="false")]
    pub category: Option<CodeableConcept>,
    /// The quantity of the item or items being reported
    #[fhir(name="quantity", min="1", max="1", summary="true", modifier="false")]
    pub quantity: Option<Quantity>,
    /// The code or reference to the item type
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableReference>,
}


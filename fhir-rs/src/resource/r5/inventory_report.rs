use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct InventoryReport {
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
    /// Business identifier for the report
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | requested | active | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// snapshot | difference
    #[fhir(name="countType", min="1", max="1", summary=true, modifier=true)]
    pub count_type: Option<CodeDt>,
    /// addition | subtraction
    #[fhir(name="operationType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub operation_type: Option<CodeableConcept>,
    /// The reason for this count - regular count, ad-hoc count, new arrivals, etc
    #[fhir(name="operationTypeReason", min="0", max="1", summary=true, modifier=false, choice="")]
    pub operation_type_reason: Option<CodeableConcept>,
    /// When the report has been submitted
    #[fhir(name="reportedDateTime", min="1", max="1", summary=true, modifier=false, choice="")]
    pub reported_date_time: Option<DateTimeDt>,
    /// Who submits the report
    #[fhir(name="reporter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reporter: Option<Reference>,
    /// The period the report refers to
    #[fhir(name="reportingPeriod", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reporting_period: Option<Period>,
    /// An inventory listing section (grouped by any of the attributes)
    #[fhir(name="inventoryListing", min="0", max="*", summary=true, modifier=false, choice="")]
    pub inventory_listing: Option<Vec<InventoryReportInventoryListingBackboneElement>>,
    /// A note associated with the InventoryReport
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct InventoryReportInventoryListingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location of the inventory items
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// The status of the items that are being reported
    #[fhir(name="itemStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub item_status: Option<CodeableConcept>,
    /// The date and time when the items were counted
    #[fhir(name="countingDateTime", min="0", max="1", summary=false, modifier=false, choice="")]
    pub counting_date_time: Option<DateTimeDt>,
    /// The item or items in this listing
    #[fhir(name="item", min="0", max="*", summary=true, modifier=false, choice="")]
    pub item: Option<Vec<InventoryReportInventoryListingItemBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct InventoryReportInventoryListingItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The inventory category or classification of the items being reported
    #[fhir(name="category", min="0", max="1", summary=true, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// The quantity of the item or items being reported
    #[fhir(name="quantity", min="1", max="1", summary=true, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// The code or reference to the item type
    #[fhir(name="item", min="1", max="1", summary=true, modifier=false, choice="")]
    pub item: Option<CodeableReference>,
}


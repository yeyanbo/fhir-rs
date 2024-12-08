use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ClaimResponse {
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
    /// Business Identifier for a claim response
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// More granular claim type
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// More granular claim type
    #[fhir(name="subType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sub_type: Option<CodeableConcept>,
    /// claim | preauthorization | predetermination
    #[fhir(name="use", min="1", max="1", summary=true, modifier=false, choice="")]
    pub use_: Option<CodeDt>,
    /// The recipient of the products and services
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false, choice="")]
    pub patient: Option<Reference>,
    /// Response creation date
    #[fhir(name="created", min="1", max="1", summary=true, modifier=false, choice="")]
    pub created: Option<DateTimeDt>,
    /// Party responsible for reimbursement
    #[fhir(name="insurer", min="0", max="1", summary=true, modifier=false, choice="")]
    pub insurer: Option<Reference>,
    /// Party responsible for the claim
    #[fhir(name="requestor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub requestor: Option<Reference>,
    /// Id of resource triggering adjudication
    #[fhir(name="request", min="0", max="1", summary=true, modifier=false, choice="")]
    pub request: Option<Reference>,
    /// queued | complete | error | partial
    #[fhir(name="outcome", min="1", max="1", summary=true, modifier=false, choice="")]
    pub outcome: Option<CodeDt>,
    /// Result of the adjudication
    #[fhir(name="decision", min="0", max="1", summary=true, modifier=false, choice="")]
    pub decision: Option<CodeableConcept>,
    /// Disposition Message
    #[fhir(name="disposition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub disposition: Option<StringDt>,
    /// Preauthorization reference
    #[fhir(name="preAuthRef", min="0", max="1", summary=false, modifier=false, choice="")]
    pub pre_auth_ref: Option<StringDt>,
    /// Preauthorization reference effective period
    #[fhir(name="preAuthPeriod", min="0", max="1", summary=false, modifier=false, choice="")]
    pub pre_auth_period: Option<Period>,
    /// Event information
    #[fhir(name="event", min="0", max="*", summary=false, modifier=false, choice="")]
    pub event: Option<Vec<ClaimResponseEventBackboneElement>>,
    /// Party to be paid any benefits payable
    #[fhir(name="payeeType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payee_type: Option<CodeableConcept>,
    /// Encounters associated with the listed treatments
    #[fhir(name="encounter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub encounter: Option<Vec<Reference>>,
    /// Package billing code
    #[fhir(name="diagnosisRelatedGroup", min="0", max="1", summary=false, modifier=false, choice="")]
    pub diagnosis_related_group: Option<CodeableConcept>,
    /// Adjudication for claim line items
    #[fhir(name="item", min="0", max="*", summary=false, modifier=false, choice="")]
    pub item: Option<Vec<ClaimResponseItemBackboneElement>>,
    /// Insurer added line items
    #[fhir(name="addItem", min="0", max="*", summary=false, modifier=false, choice="")]
    pub add_item: Option<Vec<ClaimResponseAddItemBackboneElement>>,
    /// Header-level adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
    /// Adjudication totals
    #[fhir(name="total", min="0", max="*", summary=true, modifier=false, choice="")]
    pub total: Option<Vec<ClaimResponseTotalBackboneElement>>,
    /// Payment Details
    #[fhir(name="payment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payment: Option<ClaimResponsePaymentBackboneElement>,
    /// Funds reserved status
    #[fhir(name="fundsReserve", min="0", max="1", summary=false, modifier=false, choice="")]
    pub funds_reserve: Option<CodeableConcept>,
    /// Printed form identifier
    #[fhir(name="formCode", min="0", max="1", summary=false, modifier=false, choice="")]
    pub form_code: Option<CodeableConcept>,
    /// Printed reference or actual form
    #[fhir(name="form", min="0", max="1", summary=false, modifier=false, choice="")]
    pub form: Option<Attachment>,
    /// Note concerning adjudication
    #[fhir(name="processNote", min="0", max="*", summary=false, modifier=false, choice="")]
    pub process_note: Option<Vec<ClaimResponseProcessNoteBackboneElement>>,
    /// Request for additional information
    #[fhir(name="communicationRequest", min="0", max="*", summary=false, modifier=false, choice="")]
    pub communication_request: Option<Vec<Reference>>,
    /// Patient insurance information
    #[fhir(name="insurance", min="0", max="*", summary=false, modifier=false, choice="")]
    pub insurance: Option<Vec<ClaimResponseInsuranceBackboneElement>>,
    /// Processing errors
    #[fhir(name="error", min="0", max="*", summary=false, modifier=false, choice="")]
    pub error: Option<Vec<ClaimResponseErrorBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseEventBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specific event
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Occurance date or period
    #[fhir(name="when", min="1", max="1", summary=false, modifier=false, choice="")]
    pub when: Option<Period>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponsePaymentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Partial or complete payment
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Payment adjustment for non-claim issues
    #[fhir(name="adjustment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub adjustment: Option<Money>,
    /// Explanation for the adjustment
    #[fhir(name="adjustmentReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub adjustment_reason: Option<CodeableConcept>,
    /// Expected date of payment
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateDt>,
    /// Payable amount after adjustment
    #[fhir(name="amount", min="1", max="1", summary=false, modifier=false, choice="")]
    pub amount: Option<Money>,
    /// Business identifier for the payment
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseErrorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item sequence number
    #[fhir(name="itemSequence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub item_sequence: Option<PositiveIntDt>,
    /// Detail sequence number
    #[fhir(name="detailSequence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub detail_sequence: Option<PositiveIntDt>,
    /// Subdetail sequence number
    #[fhir(name="subDetailSequence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sub_detail_sequence: Option<PositiveIntDt>,
    /// Error code detailing processing issues
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// FHIRPath of element(s) related to issue
    #[fhir(name="expression", min="0", max="*", summary=true, modifier=false, choice="")]
    pub expression: Option<Vec<StringDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseAddItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item sequence number
    #[fhir(name="itemSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub item_sequence: Option<Vec<PositiveIntDt>>,
    /// Detail sequence number
    #[fhir(name="detailSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail_sequence: Option<Vec<PositiveIntDt>>,
    /// Subdetail sequence number
    #[fhir(name="subdetailSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub subdetail_sequence: Option<Vec<PositiveIntDt>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Authorized providers
    #[fhir(name="provider", min="0", max="*", summary=false, modifier=false, choice="")]
    pub provider: Option<Vec<Reference>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub revenue: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Request or Referral for Service
    #[fhir(name="request", min="0", max="*", summary=false, modifier=false, choice="")]
    pub request: Option<Vec<Reference>>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    #[fhir(name="programCode", min="0", max="*", summary=false, modifier=false, choice="")]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Date or dates of service or product delivery
    #[fhir(name="serviced", min="0", max="1", summary=false, modifier=false, choice="")]
    pub serviced: Option<Period>,
    /// Place of service or where product was supplied
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice="")]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub factor: Option<DecimalDt>,
    /// Total tax
    #[fhir(name="tax", min="0", max="1", summary=false, modifier=false, choice="")]
    pub tax: Option<Money>,
    /// Total item cost
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice="")]
    pub net: Option<Money>,
    /// Anatomical location
    #[fhir(name="bodySite", min="0", max="*", summary=false, modifier=false, choice="")]
    pub body_site: Option<Vec<ClaimResponseAddItemBodySiteBackboneElement>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Added items adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcomeBackboneElement>,
    /// Added items adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
    /// Insurer added line details
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail: Option<Vec<ClaimResponseAddItemDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseAddItemDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub revenue: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice="")]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub factor: Option<DecimalDt>,
    /// Total tax
    #[fhir(name="tax", min="0", max="1", summary=false, modifier=false, choice="")]
    pub tax: Option<Money>,
    /// Total item cost
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice="")]
    pub net: Option<Money>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Added items detail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcomeBackboneElement>,
    /// Added items detail adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
    /// Insurer added line items
    #[fhir(name="subDetail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_detail: Option<Vec<ClaimResponseAddItemDetailSubDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseAddItemDetailSubDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub revenue: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice="")]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub factor: Option<DecimalDt>,
    /// Total tax
    #[fhir(name="tax", min="0", max="1", summary=false, modifier=false, choice="")]
    pub tax: Option<Money>,
    /// Total item cost
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice="")]
    pub net: Option<Money>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Added items subdetail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcomeBackboneElement>,
    /// Added items subdetail adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseAddItemBodySiteBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location
    #[fhir(name="site", min="1", max="*", summary=false, modifier=false, choice="")]
    pub site: Option<Vec<CodeableReference>>,
    /// Sub-location
    #[fhir(name="subSite", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_site: Option<Vec<CodeableConcept>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseProcessNoteBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Note instance identifier
    #[fhir(name="number", min="0", max="1", summary=false, modifier=false, choice="")]
    pub number: Option<PositiveIntDt>,
    /// Note purpose
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Note explanatory text
    #[fhir(name="text", min="1", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
    /// Language of the text
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice="")]
    pub language: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseInsuranceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Insurance instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Coverage to be used for adjudication
    #[fhir(name="focal", min="1", max="1", summary=false, modifier=false, choice="")]
    pub focal: Option<BooleanDt>,
    /// Insurance information
    #[fhir(name="coverage", min="1", max="1", summary=false, modifier=false, choice="")]
    pub coverage: Option<Reference>,
    /// Additional provider contract number
    #[fhir(name="businessArrangement", min="0", max="1", summary=false, modifier=false, choice="")]
    pub business_arrangement: Option<StringDt>,
    /// Adjudication results
    #[fhir(name="claimResponse", min="0", max="1", summary=false, modifier=false, choice="")]
    pub claim_response: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Claim item instance identifier
    #[fhir(name="itemSequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub item_sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcomeBackboneElement>,
    /// Adjudication details
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
    /// Adjudication for claim details
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail: Option<Vec<ClaimResponseItemDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseItemDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Claim detail instance identifier
    #[fhir(name="detailSequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub detail_sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Detail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcomeBackboneElement>,
    /// Detail level adjudication details
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
    /// Adjudication for claim sub-details
    #[fhir(name="subDetail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_detail: Option<Vec<ClaimResponseItemDetailSubDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseItemDetailSubDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Claim sub-detail instance identifier
    #[fhir(name="subDetailSequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sub_detail_sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Subdetail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ClaimResponseItemReviewOutcomeBackboneElement>,
    /// Subdetail level adjudication details
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ClaimResponseItemAdjudicationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseItemAdjudicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of adjudication information
    #[fhir(name="category", min="1", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Explanation of adjudication outcome
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reason: Option<CodeableConcept>,
    /// Monetary amount
    #[fhir(name="amount", min="0", max="1", summary=false, modifier=false, choice="")]
    pub amount: Option<Money>,
    /// Non-monetary value
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseItemReviewOutcomeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Result of the adjudication
    #[fhir(name="decision", min="0", max="1", summary=false, modifier=false, choice="")]
    pub decision: Option<CodeableConcept>,
    /// Reason for result of the adjudication
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableConcept>>,
    /// Preauthorization reference
    #[fhir(name="preAuthRef", min="0", max="1", summary=false, modifier=false, choice="")]
    pub pre_auth_ref: Option<StringDt>,
    /// Preauthorization reference effective period
    #[fhir(name="preAuthPeriod", min="0", max="1", summary=false, modifier=false, choice="")]
    pub pre_auth_period: Option<Period>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ClaimResponseTotalBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of adjudication information
    #[fhir(name="category", min="1", max="1", summary=true, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Financial total for the category
    #[fhir(name="amount", min="1", max="1", summary=true, modifier=false, choice="")]
    pub amount: Option<Money>,
}


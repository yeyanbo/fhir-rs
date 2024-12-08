use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ExplanationOfBenefit {
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
    /// Business Identifier for the resource
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Category or discipline
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
    /// Relevant time frame for the claim
    #[fhir(name="billablePeriod", min="0", max="1", summary=true, modifier=false, choice="")]
    pub billable_period: Option<Period>,
    /// Response creation date
    #[fhir(name="created", min="1", max="1", summary=true, modifier=false, choice="")]
    pub created: Option<DateTimeDt>,
    /// Author of the claim
    #[fhir(name="enterer", min="0", max="1", summary=false, modifier=false, choice="")]
    pub enterer: Option<Reference>,
    /// Party responsible for reimbursement
    #[fhir(name="insurer", min="0", max="1", summary=true, modifier=false, choice="")]
    pub insurer: Option<Reference>,
    /// Party responsible for the claim
    #[fhir(name="provider", min="0", max="1", summary=true, modifier=false, choice="")]
    pub provider: Option<Reference>,
    /// Desired processing urgency
    #[fhir(name="priority", min="0", max="1", summary=false, modifier=false, choice="")]
    pub priority: Option<CodeableConcept>,
    /// For whom to reserve funds
    #[fhir(name="fundsReserveRequested", min="0", max="1", summary=false, modifier=false, choice="")]
    pub funds_reserve_requested: Option<CodeableConcept>,
    /// Funds reserved status
    #[fhir(name="fundsReserve", min="0", max="1", summary=false, modifier=false, choice="")]
    pub funds_reserve: Option<CodeableConcept>,
    /// Prior or corollary claims
    #[fhir(name="related", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related: Option<Vec<ExplanationOfBenefitRelatedBackboneElement>>,
    /// Prescription authorizing services or products
    #[fhir(name="prescription", min="0", max="1", summary=false, modifier=false, choice="")]
    pub prescription: Option<Reference>,
    /// Original prescription if superceded by fulfiller
    #[fhir(name="originalPrescription", min="0", max="1", summary=false, modifier=false, choice="")]
    pub original_prescription: Option<Reference>,
    /// Event information
    #[fhir(name="event", min="0", max="*", summary=false, modifier=false, choice="")]
    pub event: Option<Vec<ExplanationOfBenefitEventBackboneElement>>,
    /// Recipient of benefits payable
    #[fhir(name="payee", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payee: Option<ExplanationOfBenefitPayeeBackboneElement>,
    /// Treatment Referral
    #[fhir(name="referral", min="0", max="1", summary=false, modifier=false, choice="")]
    pub referral: Option<Reference>,
    /// Encounters associated with the listed treatments
    #[fhir(name="encounter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub encounter: Option<Vec<Reference>>,
    /// Servicing Facility
    #[fhir(name="facility", min="0", max="1", summary=false, modifier=false, choice="")]
    pub facility: Option<Reference>,
    /// Claim reference
    #[fhir(name="claim", min="0", max="1", summary=false, modifier=false, choice="")]
    pub claim: Option<Reference>,
    /// Claim response reference
    #[fhir(name="claimResponse", min="0", max="1", summary=false, modifier=false, choice="")]
    pub claim_response: Option<Reference>,
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
    #[fhir(name="preAuthRef", min="0", max="*", summary=false, modifier=false, choice="")]
    pub pre_auth_ref: Option<Vec<StringDt>>,
    /// Preauthorization in-effect period
    #[fhir(name="preAuthRefPeriod", min="0", max="*", summary=false, modifier=false, choice="")]
    pub pre_auth_ref_period: Option<Vec<Period>>,
    /// Package billing code
    #[fhir(name="diagnosisRelatedGroup", min="0", max="1", summary=false, modifier=false, choice="")]
    pub diagnosis_related_group: Option<CodeableConcept>,
    /// Care Team members
    #[fhir(name="careTeam", min="0", max="*", summary=false, modifier=false, choice="")]
    pub care_team: Option<Vec<ExplanationOfBenefitCareTeamBackboneElement>>,
    /// Supporting information
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_info: Option<Vec<ExplanationOfBenefitSupportingInfoBackboneElement>>,
    /// Pertinent diagnosis information
    #[fhir(name="diagnosis", min="0", max="*", summary=false, modifier=false, choice="")]
    pub diagnosis: Option<Vec<ExplanationOfBenefitDiagnosisBackboneElement>>,
    /// Clinical procedures performed
    #[fhir(name="procedure", min="0", max="*", summary=false, modifier=false, choice="")]
    pub procedure: Option<Vec<ExplanationOfBenefitProcedureBackboneElement>>,
    /// Precedence (primary, secondary, etc.)
    #[fhir(name="precedence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub precedence: Option<PositiveIntDt>,
    /// Patient insurance information
    #[fhir(name="insurance", min="0", max="*", summary=true, modifier=false, choice="")]
    pub insurance: Option<Vec<ExplanationOfBenefitInsuranceBackboneElement>>,
    /// Details of the event
    #[fhir(name="accident", min="0", max="1", summary=false, modifier=false, choice="")]
    pub accident: Option<ExplanationOfBenefitAccidentBackboneElement>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
    /// Product or service provided
    #[fhir(name="item", min="0", max="*", summary=false, modifier=false, choice="")]
    pub item: Option<Vec<ExplanationOfBenefitItemBackboneElement>>,
    /// Insurer added line items
    #[fhir(name="addItem", min="0", max="*", summary=false, modifier=false, choice="")]
    pub add_item: Option<Vec<ExplanationOfBenefitAddItemBackboneElement>>,
    /// Header-level adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
    /// Adjudication totals
    #[fhir(name="total", min="0", max="*", summary=true, modifier=false, choice="")]
    pub total: Option<Vec<ExplanationOfBenefitTotalBackboneElement>>,
    /// Payment Details
    #[fhir(name="payment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payment: Option<ExplanationOfBenefitPaymentBackboneElement>,
    /// Printed form identifier
    #[fhir(name="formCode", min="0", max="1", summary=false, modifier=false, choice="")]
    pub form_code: Option<CodeableConcept>,
    /// Printed reference or actual form
    #[fhir(name="form", min="0", max="1", summary=false, modifier=false, choice="")]
    pub form: Option<Attachment>,
    /// Note concerning adjudication
    #[fhir(name="processNote", min="0", max="*", summary=false, modifier=false, choice="")]
    pub process_note: Option<Vec<ExplanationOfBenefitProcessNoteBackboneElement>>,
    /// When the benefits are applicable
    #[fhir(name="benefitPeriod", min="0", max="1", summary=false, modifier=false, choice="")]
    pub benefit_period: Option<Period>,
    /// Balance by Benefit Category
    #[fhir(name="benefitBalance", min="0", max="*", summary=false, modifier=false, choice="")]
    pub benefit_balance: Option<Vec<ExplanationOfBenefitBenefitBalanceBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitInsuranceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Coverage to be used for adjudication
    #[fhir(name="focal", min="1", max="1", summary=true, modifier=false, choice="")]
    pub focal: Option<BooleanDt>,
    /// Insurance information
    #[fhir(name="coverage", min="1", max="1", summary=true, modifier=false, choice="")]
    pub coverage: Option<Reference>,
    /// Prior authorization reference number
    #[fhir(name="preAuthRef", min="0", max="*", summary=false, modifier=false, choice="")]
    pub pre_auth_ref: Option<Vec<StringDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitAccidentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// When the incident occurred
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateDt>,
    /// The nature of the accident
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Where the event occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Applicable care team members
    #[fhir(name="careTeamSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub care_team_sequence: Option<Vec<PositiveIntDt>>,
    /// Applicable diagnoses
    #[fhir(name="diagnosisSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub diagnosis_sequence: Option<Vec<PositiveIntDt>>,
    /// Applicable procedures
    #[fhir(name="procedureSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub procedure_sequence: Option<Vec<PositiveIntDt>>,
    /// Applicable exception and supporting information
    #[fhir(name="informationSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub information_sequence: Option<Vec<PositiveIntDt>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Request or Referral for Service
    #[fhir(name="request", min="0", max="*", summary=false, modifier=false, choice="")]
    pub request: Option<Vec<Reference>>,
    /// Product or service billing modifiers
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
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
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
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice="")]
    pub udi: Option<Vec<Reference>>,
    /// Anatomical location
    #[fhir(name="bodySite", min="0", max="*", summary=false, modifier=false, choice="")]
    pub body_site: Option<Vec<ExplanationOfBenefitItemBodySiteBackboneElement>>,
    /// Encounters associated with the listed treatments
    #[fhir(name="encounter", min="0", max="*", summary=false, modifier=false, choice="")]
    pub encounter: Option<Vec<Reference>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcomeBackboneElement>,
    /// Adjudication details
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
    /// Additional items
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail: Option<Vec<ExplanationOfBenefitItemDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitItemAdjudicationBackboneElement {
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
    /// Non-monitary value
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitItemDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Product or service provided
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    #[fhir(name="programCode", min="0", max="*", summary=false, modifier=false, choice="")]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
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
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice="")]
    pub udi: Option<Vec<Reference>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Detail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcomeBackboneElement>,
    /// Detail level adjudication details
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
    /// Additional items
    #[fhir(name="subDetail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_detail: Option<Vec<ExplanationOfBenefitItemDetailSubDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitItemDetailSubDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Product or service provided
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice="")]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    #[fhir(name="programCode", min="0", max="*", summary=false, modifier=false, choice="")]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
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
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice="")]
    pub udi: Option<Vec<Reference>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Subdetail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcomeBackboneElement>,
    /// Subdetail level adjudication details
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitItemReviewOutcomeBackboneElement {
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
pub struct ExplanationOfBenefitItemBodySiteBackboneElement {
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
pub struct ExplanationOfBenefitSupportingInfoBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Information instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Classification of the supplied information
    #[fhir(name="category", min="1", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Type of information
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// When it occurred
    #[fhir(name="timing", min="0", max="1", summary=false, modifier=false, choice="")]
    pub timing: Option<Period>,
    /// Data to be provided
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Identifier>,
    /// Explanation for the information
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reason: Option<Coding>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitAddItemBackboneElement {
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
    #[fhir(name="subDetailSequence", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_detail_sequence: Option<Vec<PositiveIntDt>>,
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
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
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
    pub body_site: Option<Vec<ExplanationOfBenefitAddItemBodySiteBackboneElement>>,
    /// Applicable note numbers
    #[fhir(name="noteNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note_number: Option<Vec<PositiveIntDt>>,
    /// Additem level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcomeBackboneElement>,
    /// Added items adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
    /// Insurer added line items
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub detail: Option<Vec<ExplanationOfBenefitAddItemDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitAddItemDetailBackboneElement {
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
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
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
    /// Additem detail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcomeBackboneElement>,
    /// Added items adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
    /// Insurer added line items
    #[fhir(name="subDetail", min="0", max="*", summary=false, modifier=false, choice="")]
    pub sub_detail: Option<Vec<ExplanationOfBenefitAddItemDetailSubDetailBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitAddItemDetailSubDetailBackboneElement {
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
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice="")]
    pub patient_paid: Option<Money>,
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
    /// Additem subdetail level adjudication results
    #[fhir(name="reviewOutcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub review_outcome: Option<ExplanationOfBenefitItemReviewOutcomeBackboneElement>,
    /// Added items adjudication
    #[fhir(name="adjudication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub adjudication: Option<Vec<ExplanationOfBenefitItemAdjudicationBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitAddItemBodySiteBackboneElement {
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
pub struct ExplanationOfBenefitPaymentBackboneElement {
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
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Payment adjustment for non-claim issues
    #[fhir(name="adjustment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub adjustment: Option<Money>,
    /// Explanation for the variance
    #[fhir(name="adjustmentReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub adjustment_reason: Option<CodeableConcept>,
    /// Expected date of payment
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateDt>,
    /// Payable amount after adjustment
    #[fhir(name="amount", min="0", max="1", summary=false, modifier=false, choice="")]
    pub amount: Option<Money>,
    /// Business identifier for the payment
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitEventBackboneElement {
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
pub struct ExplanationOfBenefitPayeeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Category of recipient
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Recipient reference
    #[fhir(name="party", min="0", max="1", summary=false, modifier=false, choice="")]
    pub party: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitDiagnosisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Diagnosis instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Nature of illness or problem
    #[fhir(name="diagnosis", min="1", max="1", summary=false, modifier=false, choice="")]
    pub diagnosis: Option<Reference>,
    /// Timing or nature of the diagnosis
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Present on admission
    #[fhir(name="onAdmission", min="0", max="1", summary=false, modifier=false, choice="")]
    pub on_admission: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitTotalBackboneElement {
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

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitProcessNoteBackboneElement {
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
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
    /// Language of the text
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice="")]
    pub language: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitCareTeamBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Order of care team
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Practitioner or organization
    #[fhir(name="provider", min="1", max="1", summary=false, modifier=false, choice="")]
    pub provider: Option<Reference>,
    /// Indicator of the lead practitioner
    #[fhir(name="responsible", min="0", max="1", summary=false, modifier=false, choice="")]
    pub responsible: Option<BooleanDt>,
    /// Function within the team
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
    /// Practitioner or provider specialization
    #[fhir(name="specialty", min="0", max="1", summary=false, modifier=false, choice="")]
    pub specialty: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitProcedureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Procedure instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Category of Procedure
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// When the procedure was performed
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// Specific clinical procedure
    #[fhir(name="procedure", min="1", max="1", summary=false, modifier=false, choice="")]
    pub procedure: Option<Reference>,
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice="")]
    pub udi: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitRelatedBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to the related claim
    #[fhir(name="claim", min="0", max="1", summary=false, modifier=false, choice="")]
    pub claim: Option<Reference>,
    /// How the reference claim is related
    #[fhir(name="relationship", min="0", max="1", summary=false, modifier=false, choice="")]
    pub relationship: Option<CodeableConcept>,
    /// File or case reference
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<Identifier>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitBenefitBalanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Benefit classification
    #[fhir(name="category", min="1", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Excluded from the plan
    #[fhir(name="excluded", min="0", max="1", summary=false, modifier=false, choice="")]
    pub excluded: Option<BooleanDt>,
    /// Short name for the benefit
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Description of the benefit or services covered
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// In or out of network
    #[fhir(name="network", min="0", max="1", summary=false, modifier=false, choice="")]
    pub network: Option<CodeableConcept>,
    /// Individual or family
    #[fhir(name="unit", min="0", max="1", summary=false, modifier=false, choice="")]
    pub unit: Option<CodeableConcept>,
    /// Annual or lifetime
    #[fhir(name="term", min="0", max="1", summary=false, modifier=false, choice="")]
    pub term: Option<CodeableConcept>,
    /// Benefit Summary
    #[fhir(name="financial", min="0", max="*", summary=false, modifier=false, choice="")]
    pub financial: Option<Vec<ExplanationOfBenefitBenefitBalanceFinancialBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ExplanationOfBenefitBenefitBalanceFinancialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Benefit classification
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Benefits allowed
    #[fhir(name="allowed", min="0", max="1", summary=false, modifier=false, choice="")]
    pub allowed: Option<Money>,
    /// Benefits used
    #[fhir(name="used", min="0", max="1", summary=false, modifier=false, choice="")]
    pub used: Option<Money>,
}


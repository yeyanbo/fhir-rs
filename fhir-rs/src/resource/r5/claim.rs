use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Claim {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business Identifier for claim
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub trace_number: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Category or discipline
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// More granular claim type
    #[fhir(name="subType", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub sub_type: Option<CodeableConcept>,
    /// claim | preauthorization | predetermination
    #[fhir(name="use", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub use_: Option<CodeDt>,
    /// The recipient of the products and services
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub patient: Option<Reference>,
    /// Relevant time frame for the claim
    #[fhir(name="billablePeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub billable_period: Option<Period>,
    /// Resource creation date
    #[fhir(name="created", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub created: Option<DateTimeDt>,
    /// Author of the claim
    #[fhir(name="enterer", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub enterer: Option<Reference>,
    /// Target
    #[fhir(name="insurer", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub insurer: Option<Reference>,
    /// Party responsible for the claim
    #[fhir(name="provider", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub provider: Option<Reference>,
    /// Desired processing urgency
    #[fhir(name="priority", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub priority: Option<CodeableConcept>,
    /// For whom to reserve funds
    #[fhir(name="fundsReserve", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub funds_reserve: Option<CodeableConcept>,
    /// Prior or corollary claims
    #[fhir(name="related", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub related: Option<Vec<ClaimRelatedBackboneElement>>,
    /// Prescription authorizing services and products
    #[fhir(name="prescription", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub prescription: Option<Reference>,
    /// Original prescription if superseded by fulfiller
    #[fhir(name="originalPrescription", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub original_prescription: Option<Reference>,
    /// Recipient of benefits payable
    #[fhir(name="payee", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub payee: Option<ClaimPayeeBackboneElement>,
    /// Treatment referral
    #[fhir(name="referral", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub referral: Option<Reference>,
    /// Encounters associated with the listed treatments
    #[fhir(name="encounter", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub encounter: Option<Vec<Reference>>,
    /// Servicing facility
    #[fhir(name="facility", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub facility: Option<Reference>,
    /// Package billing code
    #[fhir(name="diagnosisRelatedGroup", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub diagnosis_related_group: Option<CodeableConcept>,
    /// Event information
    #[fhir(name="event", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub event: Option<Vec<ClaimEventBackboneElement>>,
    /// Members of the care team
    #[fhir(name="careTeam", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub care_team: Option<Vec<ClaimCareTeamBackboneElement>>,
    /// Supporting information
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supporting_info: Option<Vec<ClaimSupportingInfoBackboneElement>>,
    /// Pertinent diagnosis information
    #[fhir(name="diagnosis", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub diagnosis: Option<Vec<ClaimDiagnosisBackboneElement>>,
    /// Clinical procedures performed
    #[fhir(name="procedure", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub procedure: Option<Vec<ClaimProcedureBackboneElement>>,
    /// Patient insurance information
    #[fhir(name="insurance", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub insurance: Option<Vec<ClaimInsuranceBackboneElement>>,
    /// Details of the event
    #[fhir(name="accident", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub accident: Option<ClaimAccidentBackboneElement>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub patient_paid: Option<Money>,
    /// Product or service provided
    #[fhir(name="item", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub item: Option<Vec<ClaimItemBackboneElement>>,
    /// Total claim cost
    #[fhir(name="total", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub total: Option<Money>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimCareTeamBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Order of care team
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Practitioner or organization
    #[fhir(name="provider", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub provider: Option<Reference>,
    /// Indicator of the lead practitioner
    #[fhir(name="responsible", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub responsible: Option<BooleanDt>,
    /// Function within the team
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub role: Option<CodeableConcept>,
    /// Practitioner or provider specialization
    #[fhir(name="specialty", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub specialty: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub trace_number: Option<Vec<Identifier>>,
    /// Applicable careTeam members
    #[fhir(name="careTeamSequence", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub care_team_sequence: Option<Vec<PositiveIntDt>>,
    /// Applicable diagnoses
    #[fhir(name="diagnosisSequence", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub diagnosis_sequence: Option<Vec<PositiveIntDt>>,
    /// Applicable procedures
    #[fhir(name="procedureSequence", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub procedure_sequence: Option<Vec<PositiveIntDt>>,
    /// Applicable exception and supporting information
    #[fhir(name="informationSequence", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub information_sequence: Option<Vec<PositiveIntDt>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Request or Referral for Service
    #[fhir(name="request", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub request: Option<Vec<Reference>>,
    /// Product or service billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    #[fhir(name="programCode", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Date or dates of service or product delivery
    #[fhir(name="serviced", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub serviced: Option<Period>,
    /// Place of service or where product was supplied
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub location: Option<Reference>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub patient_paid: Option<Money>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub factor: Option<DecimalDt>,
    /// Total tax
    #[fhir(name="tax", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub tax: Option<Money>,
    /// Total item cost
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub net: Option<Money>,
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub udi: Option<Vec<Reference>>,
    /// Anatomical location
    #[fhir(name="bodySite", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub body_site: Option<Vec<ClaimItemBodySiteBackboneElement>>,
    /// Encounters associated with the listed treatments
    #[fhir(name="encounter", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub encounter: Option<Vec<Reference>>,
    /// Product or service provided
    #[fhir(name="detail", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub detail: Option<Vec<ClaimItemDetailBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimItemDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    #[fhir(name="programCode", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub patient_paid: Option<Money>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub factor: Option<DecimalDt>,
    /// Total tax
    #[fhir(name="tax", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub tax: Option<Money>,
    /// Total item cost
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub net: Option<Money>,
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub udi: Option<Vec<Reference>>,
    /// Product or service provided
    #[fhir(name="subDetail", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub sub_detail: Option<Vec<ClaimItemDetailSubDetailBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimItemDetailSubDetailBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Number for tracking
    #[fhir(name="traceNumber", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub trace_number: Option<Vec<Identifier>>,
    /// Revenue or cost center code
    #[fhir(name="revenue", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub revenue: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service: Option<CodeableConcept>,
    /// End of a range of codes
    #[fhir(name="productOrServiceEnd", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service_end: Option<CodeableConcept>,
    /// Service/Product billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Program the product or service is provided under
    #[fhir(name="programCode", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub program_code: Option<Vec<CodeableConcept>>,
    /// Paid by the patient
    #[fhir(name="patientPaid", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub patient_paid: Option<Money>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub unit_price: Option<Money>,
    /// Price scaling factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub factor: Option<DecimalDt>,
    /// Total tax
    #[fhir(name="tax", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub tax: Option<Money>,
    /// Total item cost
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub net: Option<Money>,
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub udi: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimItemBodySiteBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Location
    #[fhir(name="site", min="1", max="*", summary=false, modifier=false, choice=false)]
    pub site: Option<Vec<CodeableReference>>,
    /// Sub-location
    #[fhir(name="subSite", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub sub_site: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimPayeeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Category of recipient
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Recipient reference
    #[fhir(name="party", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub party: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimEventBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specific event
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Occurance date or period
    #[fhir(name="when", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub when: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimProcedureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Procedure instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Category of Procedure
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// When the procedure was performed
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub date: Option<DateTimeDt>,
    /// Specific clinical procedure
    #[fhir(name="procedure", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub procedure: Option<Reference>,
    /// Unique device identifier
    #[fhir(name="udi", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub udi: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimAccidentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// When the incident occurred
    #[fhir(name="date", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub date: Option<DateDt>,
    /// The nature of the accident
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Where the event occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub location: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimInsuranceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Insurance instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Coverage to be used for adjudication
    #[fhir(name="focal", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub focal: Option<BooleanDt>,
    /// Pre-assigned Claim number
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Identifier>,
    /// Insurance information
    #[fhir(name="coverage", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub coverage: Option<Reference>,
    /// Additional provider contract number
    #[fhir(name="businessArrangement", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub business_arrangement: Option<StringDt>,
    /// Prior authorization reference number
    #[fhir(name="preAuthRef", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub pre_auth_ref: Option<Vec<StringDt>>,
    /// Adjudication results
    #[fhir(name="claimResponse", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub claim_response: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimDiagnosisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Diagnosis instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Nature of illness or problem
    #[fhir(name="diagnosis", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub diagnosis: Option<Reference>,
    /// Timing or nature of the diagnosis
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Present on admission
    #[fhir(name="onAdmission", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub on_admission: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimSupportingInfoBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Information instance identifier
    #[fhir(name="sequence", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub sequence: Option<PositiveIntDt>,
    /// Classification of the supplied information
    #[fhir(name="category", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub category: Option<CodeableConcept>,
    /// Type of information
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// When it occurred
    #[fhir(name="timing", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub timing: Option<Period>,
    /// Data to be provided
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub value: Option<Identifier>,
    /// Explanation for the information
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub reason: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClaimRelatedBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to the related claim
    #[fhir(name="claim", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub claim: Option<Reference>,
    /// How the reference claim is related
    #[fhir(name="relationship", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub relationship: Option<CodeableConcept>,
    /// File or case reference
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub reference: Option<Identifier>,
}


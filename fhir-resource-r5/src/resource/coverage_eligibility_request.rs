use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CoverageEligibilityRequest {
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
    /// Business Identifier for coverage eligiblity request
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Desired processing priority
    #[fhir(name="priority", min="0", max="1", summary="false", modifier="false")]
    pub priority: Option<CodeableConcept>,
    /// auth-requirements | benefits | discovery | validation
    #[fhir(name="purpose", min="1", max="*", summary="true", modifier="false")]
    pub purpose: Option<Vec<CodeDt>>,
    /// Intended recipient of products and services
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
    /// Event information
    #[fhir(name="event", min="0", max="*", summary="false", modifier="false")]
    pub event: Option<Vec<CoverageEligibilityRequestEventBackboneElement>>,
    /// Estimated date or dates of service
    #[fhir(name="serviced", min="0", max="1", summary="false", modifier="false")]
    pub serviced: Option<Period>,
    /// Creation date
    #[fhir(name="created", min="1", max="1", summary="true", modifier="false")]
    pub created: Option<DateTimeDt>,
    /// Author
    #[fhir(name="enterer", min="0", max="1", summary="false", modifier="false")]
    pub enterer: Option<Reference>,
    /// Party responsible for the request
    #[fhir(name="provider", min="0", max="1", summary="false", modifier="false")]
    pub provider: Option<Reference>,
    /// Coverage issuer
    #[fhir(name="insurer", min="1", max="1", summary="true", modifier="false")]
    pub insurer: Option<Reference>,
    /// Servicing facility
    #[fhir(name="facility", min="0", max="1", summary="false", modifier="false")]
    pub facility: Option<Reference>,
    /// Supporting information
    #[fhir(name="supportingInfo", min="0", max="*", summary="false", modifier="false")]
    pub supporting_info: Option<Vec<CoverageEligibilityRequestSupportingInfoBackboneElement>>,
    /// Patient insurance information
    #[fhir(name="insurance", min="0", max="*", summary="false", modifier="false")]
    pub insurance: Option<Vec<CoverageEligibilityRequestInsuranceBackboneElement>>,
    /// Item to be evaluated for eligibiity
    #[fhir(name="item", min="0", max="*", summary="false", modifier="false")]
    pub item: Option<Vec<CoverageEligibilityRequestItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityRequestEventBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specific event
    #[fhir(name="type", min="1", max="1", summary="false", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Occurance date or period
    #[fhir(name="when", min="1", max="1", summary="false", modifier="false")]
    pub when: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityRequestItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Applicable exception or supporting information
    #[fhir(name="supportingInfoSequence", min="0", max="*", summary="false", modifier="false")]
    pub supporting_info_sequence: Option<Vec<PositiveIntDt>>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary="false", modifier="false")]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary="false", modifier="false")]
    pub product_or_service: Option<CodeableConcept>,
    /// Product or service billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary="false", modifier="false")]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Perfoming practitioner
    #[fhir(name="provider", min="0", max="1", summary="false", modifier="false")]
    pub provider: Option<Reference>,
    /// Count of products or services
    #[fhir(name="quantity", min="0", max="1", summary="false", modifier="false")]
    pub quantity: Option<Quantity>,
    /// Fee, charge or cost per item
    #[fhir(name="unitPrice", min="0", max="1", summary="false", modifier="false")]
    pub unit_price: Option<Money>,
    /// Servicing facility
    #[fhir(name="facility", min="0", max="1", summary="false", modifier="false")]
    pub facility: Option<Reference>,
    /// Applicable diagnosis
    #[fhir(name="diagnosis", min="0", max="*", summary="false", modifier="false")]
    pub diagnosis: Option<Vec<CoverageEligibilityRequestItemDiagnosisBackboneElement>>,
    /// Product or service details
    #[fhir(name="detail", min="0", max="*", summary="false", modifier="false")]
    pub detail: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityRequestItemDiagnosisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Nature of illness or problem
    #[fhir(name="diagnosis", min="0", max="1", summary="false", modifier="false")]
    pub diagnosis: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityRequestInsuranceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Applicable coverage
    #[fhir(name="focal", min="0", max="1", summary="false", modifier="false")]
    pub focal: Option<BooleanDt>,
    /// Insurance information
    #[fhir(name="coverage", min="1", max="1", summary="false", modifier="false")]
    pub coverage: Option<Reference>,
    /// Additional provider contract number
    #[fhir(name="businessArrangement", min="0", max="1", summary="false", modifier="false")]
    pub business_arrangement: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityRequestSupportingInfoBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Information instance identifier
    #[fhir(name="sequence", min="1", max="1", summary="false", modifier="false")]
    pub sequence: Option<PositiveIntDt>,
    /// Data to be provided
    #[fhir(name="information", min="1", max="1", summary="false", modifier="false")]
    pub information: Option<Reference>,
    /// Applies to all items
    #[fhir(name="appliesToAll", min="0", max="1", summary="false", modifier="false")]
    pub applies_to_all: Option<BooleanDt>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CoverageEligibilityResponse {
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
    /// Business Identifier for coverage eligiblity request
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// auth-requirements | benefits | discovery | validation
    #[fhir(name="purpose", min="1", max="*", summary=true, modifier=false, choice=false)]
    pub purpose: Option<Vec<CodeDt>>,
    /// Intended recipient of products and services
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub patient: Option<Reference>,
    /// Event information
    #[fhir(name="event", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub event: Option<Vec<CoverageEligibilityResponseEventBackboneElement>>,
    /// Estimated date or dates of service
    #[fhir(name="serviced", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub serviced: Option<Period>,
    /// Response creation date
    #[fhir(name="created", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub created: Option<DateTimeDt>,
    /// Party responsible for the request
    #[fhir(name="requestor", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub requestor: Option<Reference>,
    /// Eligibility request reference
    #[fhir(name="request", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub request: Option<Reference>,
    /// queued | complete | error | partial
    #[fhir(name="outcome", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub outcome: Option<CodeDt>,
    /// Disposition Message
    #[fhir(name="disposition", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub disposition: Option<StringDt>,
    /// Coverage issuer
    #[fhir(name="insurer", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub insurer: Option<Reference>,
    /// Patient insurance information
    #[fhir(name="insurance", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub insurance: Option<Vec<CoverageEligibilityResponseInsuranceBackboneElement>>,
    /// Preauthorization reference
    #[fhir(name="preAuthRef", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub pre_auth_ref: Option<StringDt>,
    /// Printed form identifier
    #[fhir(name="form", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub form: Option<CodeableConcept>,
    /// Processing errors
    #[fhir(name="error", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub error: Option<Vec<CoverageEligibilityResponseErrorBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityResponseEventBackboneElement {
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
pub struct CoverageEligibilityResponseInsuranceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Insurance information
    #[fhir(name="coverage", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub coverage: Option<Reference>,
    /// Coverage inforce indicator
    #[fhir(name="inforce", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub inforce: Option<BooleanDt>,
    /// When the benefits are applicable
    #[fhir(name="benefitPeriod", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub benefit_period: Option<Period>,
    /// Benefits and authorization details
    #[fhir(name="item", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub item: Option<Vec<CoverageEligibilityResponseInsuranceItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityResponseInsuranceItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub category: Option<CodeableConcept>,
    /// Billing, service, product, or drug code
    #[fhir(name="productOrService", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_or_service: Option<CodeableConcept>,
    /// Product or service billing modifiers
    #[fhir(name="modifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub modifier: Option<Vec<CodeableConcept>>,
    /// Performing practitioner
    #[fhir(name="provider", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub provider: Option<Reference>,
    /// Excluded from the plan
    #[fhir(name="excluded", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub excluded: Option<BooleanDt>,
    /// Short name for the benefit
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub name: Option<StringDt>,
    /// Description of the benefit or services covered
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<StringDt>,
    /// In or out of network
    #[fhir(name="network", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub network: Option<CodeableConcept>,
    /// Individual or family
    #[fhir(name="unit", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub unit: Option<CodeableConcept>,
    /// Annual or lifetime
    #[fhir(name="term", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub term: Option<CodeableConcept>,
    /// Benefit Summary
    #[fhir(name="benefit", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub benefit: Option<Vec<CoverageEligibilityResponseInsuranceItemBenefitBackboneElement>>,
    /// Authorization required flag
    #[fhir(name="authorizationRequired", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub authorization_required: Option<BooleanDt>,
    /// Type of required supporting materials
    #[fhir(name="authorizationSupporting", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub authorization_supporting: Option<Vec<CodeableConcept>>,
    /// Preauthorization requirements endpoint
    #[fhir(name="authorizationUrl", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub authorization_url: Option<UriDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityResponseInsuranceItemBenefitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Benefit classification
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Benefits allowed
    #[fhir(name="allowed", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub allowed: Option<Money>,
    /// Benefits used
    #[fhir(name="used", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub used: Option<Money>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageEligibilityResponseErrorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Error code detailing processing issues
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// FHIRPath of element(s) related to issue
    #[fhir(name="expression", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub expression: Option<Vec<StringDt>>,
}


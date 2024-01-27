use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Coverage {
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
    /// Business identifier(s) for this coverage
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// insurance | self-pay | other
    #[fhir(name="kind", min="1", max="1", summary="true", modifier="false")]
    pub kind: Option<CodeDt>,
    /// Self-pay parties and responsibility
    #[fhir(name="paymentBy", min="0", max="*", summary="false", modifier="false")]
    pub payment_by: Option<Vec<CoveragePaymentByBackboneElement>>,
    /// Coverage category such as medical or accident
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Owner of the policy
    #[fhir(name="policyHolder", min="0", max="1", summary="true", modifier="false")]
    pub policy_holder: Option<Reference>,
    /// Subscriber to the policy
    #[fhir(name="subscriber", min="0", max="1", summary="true", modifier="false")]
    pub subscriber: Option<Reference>,
    /// ID assigned to the subscriber
    #[fhir(name="subscriberId", min="0", max="*", summary="true", modifier="false")]
    pub subscriber_id: Option<Vec<Identifier>>,
    /// Plan beneficiary
    #[fhir(name="beneficiary", min="1", max="1", summary="true", modifier="false")]
    pub beneficiary: Option<Reference>,
    /// Dependent number
    #[fhir(name="dependent", min="0", max="1", summary="true", modifier="false")]
    pub dependent: Option<StringDt>,
    /// Beneficiary relationship to the subscriber
    #[fhir(name="relationship", min="0", max="1", summary="false", modifier="false")]
    pub relationship: Option<CodeableConcept>,
    /// Coverage start and end dates
    #[fhir(name="period", min="0", max="1", summary="true", modifier="false")]
    pub period: Option<Period>,
    /// Issuer of the policy
    #[fhir(name="insurer", min="0", max="1", summary="true", modifier="false")]
    pub insurer: Option<Reference>,
    /// Additional coverage classifications
    #[fhir(name="class", min="0", max="*", summary="false", modifier="false")]
    pub class: Option<Vec<CoverageClassBackboneElement>>,
    /// Relative order of the coverage
    #[fhir(name="order", min="0", max="1", summary="true", modifier="false")]
    pub order: Option<PositiveIntDt>,
    /// Insurer network
    #[fhir(name="network", min="0", max="1", summary="true", modifier="false")]
    pub network: Option<StringDt>,
    /// Patient payments for services/products
    #[fhir(name="costToBeneficiary", min="0", max="*", summary="false", modifier="false")]
    pub cost_to_beneficiary: Option<Vec<CoverageCostToBeneficiaryBackboneElement>>,
    /// Reimbursement to insurer
    #[fhir(name="subrogation", min="0", max="1", summary="false", modifier="false")]
    pub subrogation: Option<BooleanDt>,
    /// Contract details
    #[fhir(name="contract", min="0", max="*", summary="false", modifier="false")]
    pub contract: Option<Vec<Reference>>,
    /// Insurance plan details
    #[fhir(name="insurancePlan", min="0", max="1", summary="false", modifier="false")]
    pub insurance_plan: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoveragePaymentByBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Parties performing self-payment
    #[fhir(name="party", min="1", max="1", summary="true", modifier="false")]
    pub party: Option<Reference>,
    /// Party's responsibility
    #[fhir(name="responsibility", min="0", max="1", summary="true", modifier="false")]
    pub responsibility: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageCostToBeneficiaryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Cost category
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Benefit classification
    #[fhir(name="category", min="0", max="1", summary="false", modifier="false")]
    pub category: Option<CodeableConcept>,
    /// In or out of network
    #[fhir(name="network", min="0", max="1", summary="false", modifier="false")]
    pub network: Option<CodeableConcept>,
    /// Individual or family
    #[fhir(name="unit", min="0", max="1", summary="false", modifier="false")]
    pub unit: Option<CodeableConcept>,
    /// Annual or lifetime
    #[fhir(name="term", min="0", max="1", summary="false", modifier="false")]
    pub term: Option<CodeableConcept>,
    /// The amount or percentage due from the beneficiary
    #[fhir(name="value", min="0", max="1", summary="true", modifier="false")]
    pub value: Option<Money>,
    /// Exceptions for patient payments
    #[fhir(name="exception", min="0", max="*", summary="false", modifier="false")]
    pub exception: Option<Vec<CoverageCostToBeneficiaryExceptionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageCostToBeneficiaryExceptionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Exception category
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The effective period of the exception
    #[fhir(name="period", min="0", max="1", summary="true", modifier="false")]
    pub period: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CoverageClassBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of class such as 'group' or 'plan'
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Value associated with the type
    #[fhir(name="value", min="1", max="1", summary="true", modifier="false")]
    pub value: Option<Identifier>,
    /// Human readable description of the type and value
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
}


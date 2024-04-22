use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct InsurancePlan {
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
    /// Business Identifier for Product
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Kind of product
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Official name
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Alternate names
    #[fhir(name="alias", min="0", max="*", summary=false, modifier=false, choice="")]
    pub alias: Option<Vec<StringDt>>,
    /// When the product is available
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Product issuer
    #[fhir(name="ownedBy", min="0", max="1", summary=true, modifier=false, choice="")]
    pub owned_by: Option<Reference>,
    /// Product administrator
    #[fhir(name="administeredBy", min="0", max="1", summary=true, modifier=false, choice="")]
    pub administered_by: Option<Reference>,
    /// Where product applies
    #[fhir(name="coverageArea", min="0", max="*", summary=true, modifier=false, choice="")]
    pub coverage_area: Option<Vec<Reference>>,
    /// Official contact details relevant to the health insurance plan/product
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// Technical endpoint
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false, choice="")]
    pub endpoint: Option<Vec<Reference>>,
    /// What networks are Included
    #[fhir(name="network", min="0", max="*", summary=false, modifier=false, choice="")]
    pub network: Option<Vec<Reference>>,
    /// Coverage details
    #[fhir(name="coverage", min="0", max="*", summary=false, modifier=false, choice="")]
    pub coverage: Option<Vec<InsurancePlanCoverageBackboneElement>>,
    /// Plan details
    #[fhir(name="plan", min="0", max="*", summary=false, modifier=false, choice="")]
    pub plan: Option<Vec<InsurancePlanPlanBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanPlanBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business Identifier for Product
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Type of plan
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Where product applies
    #[fhir(name="coverageArea", min="0", max="*", summary=true, modifier=false, choice="")]
    pub coverage_area: Option<Vec<Reference>>,
    /// What networks provide coverage
    #[fhir(name="network", min="0", max="*", summary=false, modifier=false, choice="")]
    pub network: Option<Vec<Reference>>,
    /// Overall costs
    #[fhir(name="generalCost", min="0", max="*", summary=false, modifier=false, choice="")]
    pub general_cost: Option<Vec<InsurancePlanPlanGeneralCostBackboneElement>>,
    /// Specific costs
    #[fhir(name="specificCost", min="0", max="*", summary=false, modifier=false, choice="")]
    pub specific_cost: Option<Vec<InsurancePlanPlanSpecificCostBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanPlanGeneralCostBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of cost
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Number of enrollees
    #[fhir(name="groupSize", min="0", max="1", summary=false, modifier=false, choice="")]
    pub group_size: Option<PositiveIntDt>,
    /// Cost value
    #[fhir(name="cost", min="0", max="1", summary=false, modifier=false, choice="")]
    pub cost: Option<Money>,
    /// Additional cost information
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub comment: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanPlanSpecificCostBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// General category of benefit
    #[fhir(name="category", min="1", max="1", summary=false, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Benefits list
    #[fhir(name="benefit", min="0", max="*", summary=false, modifier=false, choice="")]
    pub benefit: Option<Vec<InsurancePlanPlanSpecificCostBenefitBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanPlanSpecificCostBenefitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of specific benefit
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// List of the costs
    #[fhir(name="cost", min="0", max="*", summary=false, modifier=false, choice="")]
    pub cost: Option<Vec<InsurancePlanPlanSpecificCostBenefitCostBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanPlanSpecificCostBenefitCostBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of cost
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// in-network | out-of-network | other
    #[fhir(name="applicability", min="0", max="1", summary=false, modifier=false, choice="")]
    pub applicability: Option<CodeableConcept>,
    /// Additional information about the cost
    #[fhir(name="qualifiers", min="0", max="*", summary=false, modifier=false, choice="")]
    pub qualifiers: Option<Vec<CodeableConcept>>,
    /// The actual cost value
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanCoverageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of coverage
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// What networks provide coverage
    #[fhir(name="network", min="0", max="*", summary=false, modifier=false, choice="")]
    pub network: Option<Vec<Reference>>,
    /// List of benefits
    #[fhir(name="benefit", min="1", max="*", summary=false, modifier=false, choice="")]
    pub benefit: Option<Vec<InsurancePlanCoverageBenefitBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanCoverageBenefitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of benefit
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Referral requirements
    #[fhir(name="requirement", min="0", max="1", summary=false, modifier=false, choice="")]
    pub requirement: Option<StringDt>,
    /// Benefit limits
    #[fhir(name="limit", min="0", max="*", summary=false, modifier=false, choice="")]
    pub limit: Option<Vec<InsurancePlanCoverageBenefitLimitBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InsurancePlanCoverageBenefitLimitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Maximum value allowed
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Quantity>,
    /// Benefit limit details
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
}


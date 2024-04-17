use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Account {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Account number
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | entered-in-error | on-hold | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Tracks the lifecycle of the account through the billing process
    #[fhir(name="billingStatus", min="0", max="1", summary=true, modifier=false)]
    pub billing_status: Option<CodeableConcept>,
    /// E.g. patient, expense, depreciation
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Human-readable label
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// The entity that caused the expenses
    #[fhir(name="subject", min="0", max="*", summary=true, modifier=false)]
    pub subject: Option<Vec<Reference>>,
    /// Transaction window
    #[fhir(name="servicePeriod", min="0", max="1", summary=true, modifier=false)]
    pub service_period: Option<Period>,
    /// The party(s) that are responsible for covering the payment of this account, and what order should they be applied to the account
    #[fhir(name="coverage", min="0", max="*", summary=true, modifier=false)]
    pub coverage: Option<Vec<AccountCoverageBackboneElement>>,
    /// Entity managing the Account
    #[fhir(name="owner", min="0", max="1", summary=true, modifier=false)]
    pub owner: Option<Reference>,
    /// Explanation of purpose/use
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The parties ultimately responsible for balancing the Account
    #[fhir(name="guarantor", min="0", max="*", summary=false, modifier=false)]
    pub guarantor: Option<Vec<AccountGuarantorBackboneElement>>,
    /// The list of diagnoses relevant to this account
    #[fhir(name="diagnosis", min="0", max="*", summary=true, modifier=false)]
    pub diagnosis: Option<Vec<AccountDiagnosisBackboneElement>>,
    /// The list of procedures relevant to this account
    #[fhir(name="procedure", min="0", max="*", summary=true, modifier=false)]
    pub procedure: Option<Vec<AccountProcedureBackboneElement>>,
    /// Other associated accounts related to this account
    #[fhir(name="relatedAccount", min="0", max="*", summary=false, modifier=false)]
    pub related_account: Option<Vec<AccountRelatedAccountBackboneElement>>,
    /// The base or default currency
    #[fhir(name="currency", min="0", max="1", summary=false, modifier=false)]
    pub currency: Option<CodeableConcept>,
    /// Calculated account balance(s)
    #[fhir(name="balance", min="0", max="*", summary=false, modifier=false)]
    pub balance: Option<Vec<AccountBalanceBackboneElement>>,
    /// Time the balance amount was calculated
    #[fhir(name="calculatedAt", min="0", max="1", summary=false, modifier=false)]
    pub calculated_at: Option<InstantDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AccountBalanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Who is expected to pay this part of the balance
    #[fhir(name="aggregate", min="0", max="1", summary=false, modifier=false)]
    pub aggregate: Option<CodeableConcept>,
    /// current | 30 | 60 | 90 | 120
    #[fhir(name="term", min="0", max="1", summary=false, modifier=false)]
    pub term: Option<CodeableConcept>,
    /// Estimated balance
    #[fhir(name="estimate", min="0", max="1", summary=false, modifier=false)]
    pub estimate: Option<BooleanDt>,
    /// Calculated amount
    #[fhir(name="amount", min="1", max="1", summary=false, modifier=false)]
    pub amount: Option<Money>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AccountProcedureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Ranking of the procedure (for each type)
    #[fhir(name="sequence", min="0", max="1", summary=false, modifier=false)]
    pub sequence: Option<PositiveIntDt>,
    /// The procedure relevant to the account
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableReference>,
    /// Date of the procedure (when coded procedure)
    #[fhir(name="dateOfService", min="0", max="1", summary=false, modifier=false)]
    pub date_of_service: Option<DateTimeDt>,
    /// How this procedure value should be used in charging the account
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Package Code specific for billing
    #[fhir(name="packageCode", min="0", max="*", summary=false, modifier=false)]
    pub package_code: Option<Vec<CodeableConcept>>,
    /// Any devices that were associated with the procedure
    #[fhir(name="device", min="0", max="*", summary=true, modifier=false)]
    pub device: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AccountRelatedAccountBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Relationship of the associated Account
    #[fhir(name="relationship", min="0", max="1", summary=false, modifier=false)]
    pub relationship: Option<CodeableConcept>,
    /// Reference to an associated Account
    #[fhir(name="account", min="1", max="1", summary=false, modifier=false)]
    pub account: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AccountGuarantorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Responsible entity
    #[fhir(name="party", min="1", max="1", summary=false, modifier=false)]
    pub party: Option<Reference>,
    /// Credit or other hold applied
    #[fhir(name="onHold", min="0", max="1", summary=false, modifier=false)]
    pub on_hold: Option<BooleanDt>,
    /// Guarantee account during
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false)]
    pub period: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AccountCoverageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The party(s), such as insurances, that may contribute to the payment of this account
    #[fhir(name="coverage", min="1", max="1", summary=true, modifier=false)]
    pub coverage: Option<Reference>,
    /// The priority of the coverage in the context of this account
    #[fhir(name="priority", min="0", max="1", summary=true, modifier=false)]
    pub priority: Option<PositiveIntDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AccountDiagnosisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Ranking of the diagnosis (for each type)
    #[fhir(name="sequence", min="0", max="1", summary=false, modifier=false)]
    pub sequence: Option<PositiveIntDt>,
    /// The diagnosis relevant to the account
    #[fhir(name="condition", min="1", max="1", summary=true, modifier=false)]
    pub condition: Option<CodeableReference>,
    /// Date of the diagnosis (when coded diagnosis)
    #[fhir(name="dateOfDiagnosis", min="0", max="1", summary=false, modifier=false)]
    pub date_of_diagnosis: Option<DateTimeDt>,
    /// Type that this diagnosis has relevant to the account (e.g. admission, billing, discharge …)
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Diagnosis present on Admission
    #[fhir(name="onAdmission", min="0", max="1", summary=false, modifier=false)]
    pub on_admission: Option<BooleanDt>,
    /// Package Code specific for billing
    #[fhir(name="packageCode", min="0", max="*", summary=false, modifier=false)]
    pub package_code: Option<Vec<CodeableConcept>>,
}
use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct PaymentReconciliation {
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
    /// Business Identifier for a payment reconciliation
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Category of payment
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Workflow originating payment
    #[fhir(name="kind", min="0", max="1", summary=false, modifier=false, choice="")]
    pub kind: Option<CodeableConcept>,
    /// Period covered
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Creation date
    #[fhir(name="created", min="1", max="1", summary=true, modifier=false, choice="")]
    pub created: Option<DateTimeDt>,
    /// Who entered the payment
    #[fhir(name="enterer", min="0", max="1", summary=false, modifier=false, choice="")]
    pub enterer: Option<Reference>,
    /// Nature of the source
    #[fhir(name="issuerType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub issuer_type: Option<CodeableConcept>,
    /// Party generating payment
    #[fhir(name="paymentIssuer", min="0", max="1", summary=true, modifier=false, choice="")]
    pub payment_issuer: Option<Reference>,
    /// Reference to requesting resource
    #[fhir(name="request", min="0", max="1", summary=false, modifier=false, choice="")]
    pub request: Option<Reference>,
    /// Responsible practitioner
    #[fhir(name="requestor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub requestor: Option<Reference>,
    /// queued | complete | error | partial
    #[fhir(name="outcome", min="0", max="1", summary=false, modifier=false, choice="")]
    pub outcome: Option<CodeDt>,
    /// Disposition message
    #[fhir(name="disposition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub disposition: Option<StringDt>,
    /// When payment issued
    #[fhir(name="date", min="1", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateDt>,
    /// Where payment collected
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice="")]
    pub location: Option<Reference>,
    /// Payment instrument
    #[fhir(name="method", min="0", max="1", summary=false, modifier=false, choice="")]
    pub method: Option<CodeableConcept>,
    /// Type of card
    #[fhir(name="cardBrand", min="0", max="1", summary=false, modifier=false, choice="")]
    pub card_brand: Option<StringDt>,
    /// Digits for verification
    #[fhir(name="accountNumber", min="0", max="1", summary=false, modifier=false, choice="")]
    pub account_number: Option<StringDt>,
    /// Expiration year-month
    #[fhir(name="expirationDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub expiration_date: Option<DateDt>,
    /// Processor name
    #[fhir(name="processor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub processor: Option<StringDt>,
    /// Check number or payment reference
    #[fhir(name="referenceNumber", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reference_number: Option<StringDt>,
    /// Authorization number
    #[fhir(name="authorization", min="0", max="1", summary=false, modifier=false, choice="")]
    pub authorization: Option<StringDt>,
    /// Amount offered by the issuer
    #[fhir(name="tenderedAmount", min="0", max="1", summary=false, modifier=false, choice="")]
    pub tendered_amount: Option<Money>,
    /// Amount returned by the receiver
    #[fhir(name="returnedAmount", min="0", max="1", summary=false, modifier=false, choice="")]
    pub returned_amount: Option<Money>,
    /// Total amount of Payment
    #[fhir(name="amount", min="1", max="1", summary=true, modifier=false, choice="")]
    pub amount: Option<Money>,
    /// Business identifier for the payment
    #[fhir(name="paymentIdentifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payment_identifier: Option<Identifier>,
    /// Settlement particulars
    #[fhir(name="allocation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub allocation: Option<Vec<PaymentReconciliationAllocationBackboneElement>>,
    /// Printed form identifier
    #[fhir(name="formCode", min="0", max="1", summary=false, modifier=false, choice="")]
    pub form_code: Option<CodeableConcept>,
    /// Note concerning processing
    #[fhir(name="processNote", min="0", max="*", summary=false, modifier=false, choice="")]
    pub process_note: Option<Vec<PaymentReconciliationProcessNoteBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct PaymentReconciliationAllocationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business identifier of the payment detail
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Business identifier of the prior payment detail
    #[fhir(name="predecessor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub predecessor: Option<Identifier>,
    /// Subject of the payment
    #[fhir(name="target", min="0", max="1", summary=false, modifier=false, choice="")]
    pub target: Option<Reference>,
    /// Sub-element of the subject
    #[fhir(name="targetItem", min="0", max="1", summary=false, modifier=false, choice="")]
    pub target_item: Option<PositiveIntDt>,
    /// Applied-to encounter
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Applied-to account
    #[fhir(name="account", min="0", max="1", summary=false, modifier=false, choice="")]
    pub account: Option<Reference>,
    /// Category of payment
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Submitter of the request
    #[fhir(name="submitter", min="0", max="1", summary=false, modifier=false, choice="")]
    pub submitter: Option<Reference>,
    /// Response committing to a payment
    #[fhir(name="response", min="0", max="1", summary=false, modifier=false, choice="")]
    pub response: Option<Reference>,
    /// Date of commitment to pay
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateDt>,
    /// Contact for the response
    #[fhir(name="responsible", min="0", max="1", summary=false, modifier=false, choice="")]
    pub responsible: Option<Reference>,
    /// Recipient of the payment
    #[fhir(name="payee", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payee: Option<Reference>,
    /// Amount allocated to this payable
    #[fhir(name="amount", min="0", max="1", summary=false, modifier=false, choice="")]
    pub amount: Option<Money>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct PaymentReconciliationProcessNoteBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// display | print | printoper
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// Note explanatory text
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
}


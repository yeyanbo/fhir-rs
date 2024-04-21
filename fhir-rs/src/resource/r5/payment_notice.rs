use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct PaymentNotice {
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
    /// Business Identifier for the payment notice
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Request reference
    #[fhir(name="request", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub request: Option<Reference>,
    /// Response reference
    #[fhir(name="response", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub response: Option<Reference>,
    /// Creation date
    #[fhir(name="created", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub created: Option<DateTimeDt>,
    /// Responsible practitioner
    #[fhir(name="reporter", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub reporter: Option<Reference>,
    /// Payment reference
    #[fhir(name="payment", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub payment: Option<Reference>,
    /// Payment or clearing date
    #[fhir(name="paymentDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub payment_date: Option<DateDt>,
    /// Party being paid
    #[fhir(name="payee", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub payee: Option<Reference>,
    /// Party being notified
    #[fhir(name="recipient", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub recipient: Option<Reference>,
    /// Monetary amount of the payment
    #[fhir(name="amount", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub amount: Option<Money>,
    /// Issued or cleared Status of the payment
    #[fhir(name="paymentStatus", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub payment_status: Option<CodeableConcept>,
}


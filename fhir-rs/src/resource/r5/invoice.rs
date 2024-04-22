use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Invoice {
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
    /// Business Identifier for item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | issued | balanced | cancelled | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for cancellation of this Invoice
    #[fhir(name="cancelledReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub cancelled_reason: Option<StringDt>,
    /// Type of Invoice
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Recipient(s) of goods and services
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Recipient of this invoice
    #[fhir(name="recipient", min="0", max="1", summary=true, modifier=false, choice="")]
    pub recipient: Option<Reference>,
    /// DEPRICATED
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice="")]
    pub date: Option<DateTimeDt>,
    /// When posted
    #[fhir(name="creation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub creation: Option<DateTimeDt>,
    /// Billing date or period
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Participant in creation of this Invoice
    #[fhir(name="participant", min="0", max="*", summary=false, modifier=false, choice="")]
    pub participant: Option<Vec<InvoiceParticipantBackboneElement>>,
    /// Issuing Organization of Invoice
    #[fhir(name="issuer", min="0", max="1", summary=false, modifier=false, choice="")]
    pub issuer: Option<Reference>,
    /// Account that is being balanced
    #[fhir(name="account", min="0", max="1", summary=false, modifier=false, choice="")]
    pub account: Option<Reference>,
    /// Line items of this Invoice
    #[fhir(name="lineItem", min="0", max="*", summary=false, modifier=false, choice="")]
    pub line_item: Option<Vec<InvoiceLineItemBackboneElement>>,
    /// Components of Invoice total
    #[fhir(name="totalPriceComponent", min="0", max="*", summary=false, modifier=false, choice="")]
    pub total_price_component: Option<Vec<MonetaryComponent>>,
    /// Net total of this Invoice
    #[fhir(name="totalNet", min="0", max="1", summary=true, modifier=false, choice="")]
    pub total_net: Option<Money>,
    /// Gross total of this Invoice
    #[fhir(name="totalGross", min="0", max="1", summary=true, modifier=false, choice="")]
    pub total_gross: Option<Money>,
    /// Payment details
    #[fhir(name="paymentTerms", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payment_terms: Option<MarkdownDt>,
    /// Comments made about the invoice
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InvoiceLineItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Sequence number of line item
    #[fhir(name="sequence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sequence: Option<PositiveIntDt>,
    /// Service data or period
    #[fhir(name="serviced", min="0", max="1", summary=false, modifier=false, choice="")]
    pub serviced: Option<Period>,
    /// Reference to ChargeItem containing details of this line item or an inline billing code
    #[fhir(name="chargeItem", min="1", max="1", summary=false, modifier=false, choice="")]
    pub charge_item: Option<CodeableConcept>,
    /// Components of total line item price
    #[fhir(name="priceComponent", min="0", max="*", summary=false, modifier=false, choice="")]
    pub price_component: Option<Vec<MonetaryComponent>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InvoiceParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of involvement in creation of this Invoice
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
    /// Individual who was involved
    #[fhir(name="actor", min="1", max="1", summary=false, modifier=false, choice="")]
    pub actor: Option<Reference>,
}


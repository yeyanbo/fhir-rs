use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Contract {
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
    /// Contract number
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Basal definition
    #[fhir(name="url", min="0", max="1", summary=false, modifier=false, choice="")]
    pub url: Option<UriDt>,
    /// Business edition
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false, choice="")]
    pub version: Option<StringDt>,
    /// amended | appended | cancelled | disputed | entered-in-error | executable +
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Negotiation status
    #[fhir(name="legalState", min="0", max="1", summary=false, modifier=false, choice="")]
    pub legal_state: Option<CodeableConcept>,
    /// Source Contract Definition
    #[fhir(name="instantiatesCanonical", min="0", max="1", summary=false, modifier=false, choice="")]
    pub instantiates_canonical: Option<Reference>,
    /// External Contract Definition
    #[fhir(name="instantiatesUri", min="0", max="1", summary=false, modifier=false, choice="")]
    pub instantiates_uri: Option<UriDt>,
    /// Content derived from the basal information
    #[fhir(name="contentDerivative", min="0", max="1", summary=false, modifier=false, choice="")]
    pub content_derivative: Option<CodeableConcept>,
    /// When this Contract was issued
    #[fhir(name="issued", min="0", max="1", summary=true, modifier=false, choice="")]
    pub issued: Option<DateTimeDt>,
    /// Effective time
    #[fhir(name="applies", min="0", max="1", summary=true, modifier=false, choice="")]
    pub applies: Option<Period>,
    /// Contract cessation cause
    #[fhir(name="expirationType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub expiration_type: Option<CodeableConcept>,
    /// Contract Target Entity
    #[fhir(name="subject", min="0", max="*", summary=true, modifier=false, choice="")]
    pub subject: Option<Vec<Reference>>,
    /// Authority under which this Contract has standing
    #[fhir(name="authority", min="0", max="*", summary=false, modifier=false, choice="")]
    pub authority: Option<Vec<Reference>>,
    /// A sphere of control governed by an authoritative jurisdiction, organization, or person
    #[fhir(name="domain", min="0", max="*", summary=false, modifier=false, choice="")]
    pub domain: Option<Vec<Reference>>,
    /// Specific Location
    #[fhir(name="site", min="0", max="*", summary=false, modifier=false, choice="")]
    pub site: Option<Vec<Reference>>,
    /// Computer friendly designation
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// Human Friendly name
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// Subordinate Friendly name
    #[fhir(name="subtitle", min="0", max="1", summary=false, modifier=false, choice="")]
    pub subtitle: Option<StringDt>,
    /// Acronym or short name
    #[fhir(name="alias", min="0", max="*", summary=false, modifier=false, choice="")]
    pub alias: Option<Vec<StringDt>>,
    /// Source of Contract
    #[fhir(name="author", min="0", max="1", summary=false, modifier=false, choice="")]
    pub author: Option<Reference>,
    /// Range of Legal Concerns
    #[fhir(name="scope", min="0", max="1", summary=false, modifier=false, choice="")]
    pub scope: Option<CodeableConcept>,
    /// Focus of contract interest
    #[fhir(name="topic", min="0", max="1", summary=false, modifier=false, choice="")]
    pub topic: Option<Reference>,
    /// Legal instrument category
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Subtype within the context of type
    #[fhir(name="subType", min="0", max="*", summary=true, modifier=false, choice="")]
    pub sub_type: Option<Vec<CodeableConcept>>,
    /// Contract precursor content
    #[fhir(name="contentDefinition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub content_definition: Option<ContractContentDefinitionBackboneElement>,
    /// Contract Term List
    #[fhir(name="term", min="0", max="*", summary=false, modifier=false, choice="")]
    pub term: Option<Vec<ContractTermBackboneElement>>,
    /// Extra Information
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Key event in Contract History
    #[fhir(name="relevantHistory", min="0", max="*", summary=false, modifier=false, choice="")]
    pub relevant_history: Option<Vec<Reference>>,
    /// Contract Signatory
    #[fhir(name="signer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub signer: Option<Vec<ContractSignerBackboneElement>>,
    /// Contract Friendly Language
    #[fhir(name="friendly", min="0", max="*", summary=false, modifier=false, choice="")]
    pub friendly: Option<Vec<ContractFriendlyBackboneElement>>,
    /// Contract Legal Language
    #[fhir(name="legal", min="0", max="*", summary=false, modifier=false, choice="")]
    pub legal: Option<Vec<ContractLegalBackboneElement>>,
    /// Computable Contract Language
    #[fhir(name="rule", min="0", max="*", summary=false, modifier=false, choice="")]
    pub rule: Option<Vec<ContractRuleBackboneElement>>,
    /// Binding Contract
    #[fhir(name="legallyBinding", min="0", max="1", summary=false, modifier=false, choice="")]
    pub legally_binding: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Contract Term Number
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Contract Term Issue Date Time
    #[fhir(name="issued", min="0", max="1", summary=true, modifier=false, choice="")]
    pub issued: Option<DateTimeDt>,
    /// Contract Term Effective Time
    #[fhir(name="applies", min="0", max="1", summary=true, modifier=false, choice="")]
    pub applies: Option<Period>,
    /// Term Concern
    #[fhir(name="topic", min="0", max="1", summary=false, modifier=false, choice="")]
    pub topic: Option<Reference>,
    /// Contract Term Type or Form
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Contract Term Type specific classification
    #[fhir(name="subType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sub_type: Option<CodeableConcept>,
    /// Term Statement
    #[fhir(name="text", min="0", max="1", summary=true, modifier=false, choice="")]
    pub text: Option<StringDt>,
    /// Protection for the Term
    #[fhir(name="securityLabel", min="0", max="*", summary=false, modifier=false, choice="")]
    pub security_label: Option<Vec<ContractTermSecurityLabelBackboneElement>>,
    /// Context of the Contract term
    #[fhir(name="offer", min="1", max="1", summary=false, modifier=false, choice="")]
    pub offer: Option<ContractTermOfferBackboneElement>,
    /// Contract Term Asset List
    #[fhir(name="asset", min="0", max="*", summary=false, modifier=false, choice="")]
    pub asset: Option<Vec<ContractTermAssetBackboneElement>>,
    /// Entity being ascribed responsibility
    #[fhir(name="action", min="0", max="*", summary=false, modifier=false, choice="")]
    pub action: Option<Vec<ContractTermActionBackboneElement>>,
    /// Nested Contract Term Group
    #[fhir(name="group", min="0", max="*", summary=false, modifier=false, choice="")]
    pub group: Option<Vec<ContractTermBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermSecurityLabelBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Link to Security Labels
    #[fhir(name="number", min="0", max="*", summary=false, modifier=false, choice="")]
    pub number: Option<Vec<UnsignedIntDt>>,
    /// Confidentiality Protection
    #[fhir(name="classification", min="1", max="1", summary=false, modifier=false, choice="")]
    pub classification: Option<Coding>,
    /// Applicable Policy
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<Coding>>,
    /// Handling Instructions
    #[fhir(name="control", min="0", max="*", summary=false, modifier=false, choice="")]
    pub control: Option<Vec<Coding>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermActionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// True if the term prohibits the  action
    #[fhir(name="doNotPerform", min="0", max="1", summary=false, modifier=true)]
    pub do_not_perform: Option<BooleanDt>,
    /// Type or form of the action
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Entity of the action
    #[fhir(name="subject", min="0", max="*", summary=false, modifier=false, choice="")]
    pub subject: Option<Vec<ContractTermActionSubjectBackboneElement>>,
    /// Purpose for the Contract Term Action
    #[fhir(name="intent", min="1", max="1", summary=false, modifier=false, choice="")]
    pub intent: Option<CodeableConcept>,
    /// Pointer to specific item
    #[fhir(name="linkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub link_id: Option<Vec<StringDt>>,
    /// State of the action
    #[fhir(name="status", min="1", max="1", summary=false, modifier=false, choice="")]
    pub status: Option<CodeableConcept>,
    /// Episode associated with action
    #[fhir(name="context", min="0", max="1", summary=false, modifier=false, choice="")]
    pub context: Option<Reference>,
    /// Pointer to specific item
    #[fhir(name="contextLinkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub context_link_id: Option<Vec<StringDt>>,
    /// When action happens
    #[fhir(name="occurrence", min="0", max="1", summary=false, modifier=false, choice="")]
    pub occurrence: Option<Timing>,
    /// Who asked for action
    #[fhir(name="requester", min="0", max="*", summary=false, modifier=false, choice="")]
    pub requester: Option<Vec<Reference>>,
    /// Pointer to specific item
    #[fhir(name="requesterLinkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub requester_link_id: Option<Vec<StringDt>>,
    /// Kind of service performer
    #[fhir(name="performerType", min="0", max="*", summary=false, modifier=false, choice="")]
    pub performer_type: Option<Vec<CodeableConcept>>,
    /// Competency of the performer
    #[fhir(name="performerRole", min="0", max="1", summary=false, modifier=false, choice="")]
    pub performer_role: Option<CodeableConcept>,
    /// Actor that wil execute (or not) the action
    #[fhir(name="performer", min="0", max="1", summary=false, modifier=false, choice="")]
    pub performer: Option<Reference>,
    /// Pointer to specific item
    #[fhir(name="performerLinkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub performer_link_id: Option<Vec<StringDt>>,
    /// Why is action (not) needed?
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Pointer to specific item
    #[fhir(name="reasonLinkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason_link_id: Option<Vec<StringDt>>,
    /// Comments about the action
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Action restriction numbers
    #[fhir(name="securityLabelNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub security_label_number: Option<Vec<UnsignedIntDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermActionSubjectBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Entity of the action
    #[fhir(name="reference", min="1", max="*", summary=false, modifier=false, choice="")]
    pub reference: Option<Vec<Reference>>,
    /// Role type of the agent
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermAssetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Range of asset
    #[fhir(name="scope", min="0", max="1", summary=false, modifier=false, choice="")]
    pub scope: Option<CodeableConcept>,
    /// Asset category
    #[fhir(name="type", min="0", max="*", summary=false, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Associated entities
    #[fhir(name="typeReference", min="0", max="*", summary=false, modifier=false, choice="")]
    pub type_reference: Option<Vec<Reference>>,
    /// Asset sub-category
    #[fhir(name="subtype", min="0", max="*", summary=false, modifier=false, choice="")]
    pub subtype: Option<Vec<CodeableConcept>>,
    /// Kinship of the asset
    #[fhir(name="relationship", min="0", max="1", summary=false, modifier=false, choice="")]
    pub relationship: Option<Coding>,
    /// Circumstance of the asset
    #[fhir(name="context", min="0", max="*", summary=false, modifier=false, choice="")]
    pub context: Option<Vec<ContractTermAssetContextBackboneElement>>,
    /// Quality desctiption of asset
    #[fhir(name="condition", min="0", max="1", summary=false, modifier=false, choice="")]
    pub condition: Option<StringDt>,
    /// Asset availability types
    #[fhir(name="periodType", min="0", max="*", summary=false, modifier=false, choice="")]
    pub period_type: Option<Vec<CodeableConcept>>,
    /// Time period of the asset
    #[fhir(name="period", min="0", max="*", summary=false, modifier=false, choice="")]
    pub period: Option<Vec<Period>>,
    /// Time period
    #[fhir(name="usePeriod", min="0", max="*", summary=false, modifier=false, choice="")]
    pub use_period: Option<Vec<Period>>,
    /// Asset clause or question text
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
    /// Pointer to asset text
    #[fhir(name="linkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub link_id: Option<Vec<StringDt>>,
    /// Response to assets
    #[fhir(name="answer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub answer: Option<Vec<ContractTermOfferAnswerBackboneElement>>,
    /// Asset restriction numbers
    #[fhir(name="securityLabelNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub security_label_number: Option<Vec<UnsignedIntDt>>,
    /// Contract Valued Item List
    #[fhir(name="valuedItem", min="0", max="*", summary=false, modifier=false, choice="")]
    pub valued_item: Option<Vec<ContractTermAssetValuedItemBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermAssetContextBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Creator,custodian or owner
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<Reference>,
    /// Codeable asset context
    #[fhir(name="code", min="0", max="*", summary=false, modifier=false, choice="")]
    pub code: Option<Vec<CodeableConcept>>,
    /// Context description
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermAssetValuedItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Contract Valued Item Type
    #[fhir(name="entity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub entity: Option<Reference>,
    /// Contract Valued Item Number
    #[fhir(name="identifier", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Contract Valued Item Effective Tiem
    #[fhir(name="effectiveTime", min="0", max="1", summary=false, modifier=false, choice="")]
    pub effective_time: Option<DateTimeDt>,
    /// Count of Contract Valued Items
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// Contract Valued Item fee, charge, or cost
    #[fhir(name="unitPrice", min="0", max="1", summary=false, modifier=false, choice="")]
    pub unit_price: Option<Money>,
    /// Contract Valued Item Price Scaling Factor
    #[fhir(name="factor", min="0", max="1", summary=false, modifier=false, choice="")]
    pub factor: Option<DecimalDt>,
    /// Contract Valued Item Difficulty Scaling Factor
    #[fhir(name="points", min="0", max="1", summary=false, modifier=false, choice="")]
    pub points: Option<DecimalDt>,
    /// Total Contract Valued Item Value
    #[fhir(name="net", min="0", max="1", summary=false, modifier=false, choice="")]
    pub net: Option<Money>,
    /// Terms of valuation
    #[fhir(name="payment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payment: Option<StringDt>,
    /// When payment is due
    #[fhir(name="paymentDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub payment_date: Option<DateTimeDt>,
    /// Who will make payment
    #[fhir(name="responsible", min="0", max="1", summary=false, modifier=false, choice="")]
    pub responsible: Option<Reference>,
    /// Who will receive payment
    #[fhir(name="recipient", min="0", max="1", summary=false, modifier=false, choice="")]
    pub recipient: Option<Reference>,
    /// Pointer to specific item
    #[fhir(name="linkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub link_id: Option<Vec<StringDt>>,
    /// Security Labels that define affected terms
    #[fhir(name="securityLabelNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub security_label_number: Option<Vec<UnsignedIntDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermOfferBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Offer business ID
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Offer Recipient
    #[fhir(name="party", min="0", max="*", summary=false, modifier=false, choice="")]
    pub party: Option<Vec<ContractTermOfferPartyBackboneElement>>,
    /// Negotiable offer asset
    #[fhir(name="topic", min="0", max="1", summary=true, modifier=false, choice="")]
    pub topic: Option<Reference>,
    /// Contract Offer Type or Form
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Accepting party choice
    #[fhir(name="decision", min="0", max="1", summary=false, modifier=false, choice="")]
    pub decision: Option<CodeableConcept>,
    /// How decision is conveyed
    #[fhir(name="decisionMode", min="0", max="*", summary=false, modifier=false, choice="")]
    pub decision_mode: Option<Vec<CodeableConcept>>,
    /// Response to offer text
    #[fhir(name="answer", min="0", max="*", summary=false, modifier=false, choice="")]
    pub answer: Option<Vec<ContractTermOfferAnswerBackboneElement>>,
    /// Human readable offer text
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<StringDt>,
    /// Pointer to text
    #[fhir(name="linkId", min="0", max="*", summary=false, modifier=false, choice="")]
    pub link_id: Option<Vec<StringDt>>,
    /// Offer restriction numbers
    #[fhir(name="securityLabelNumber", min="0", max="*", summary=false, modifier=false, choice="")]
    pub security_label_number: Option<Vec<UnsignedIntDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermOfferAnswerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The actual answer response
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractTermOfferPartyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Referenced entity
    #[fhir(name="reference", min="1", max="*", summary=false, modifier=false, choice="")]
    pub reference: Option<Vec<Reference>>,
    /// Participant engagement type
    #[fhir(name="role", min="1", max="1", summary=false, modifier=false, choice="")]
    pub role: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractSignerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Contract Signatory Role
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<Coding>,
    /// Contract Signatory Party
    #[fhir(name="party", min="1", max="1", summary=false, modifier=false, choice="")]
    pub party: Option<Reference>,
    /// Contract Documentation Signature
    #[fhir(name="signature", min="1", max="*", summary=false, modifier=false, choice="")]
    pub signature: Option<Vec<Signature>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractRuleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Computable Contract Rules
    #[fhir(name="content", min="1", max="1", summary=false, modifier=false, choice="")]
    pub content: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractContentDefinitionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Content structure and use
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Detailed Content Type Definition
    #[fhir(name="subType", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sub_type: Option<CodeableConcept>,
    /// Publisher Entity
    #[fhir(name="publisher", min="0", max="1", summary=false, modifier=false, choice="")]
    pub publisher: Option<Reference>,
    /// When published
    #[fhir(name="publicationDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub publication_date: Option<DateTimeDt>,
    /// amended | appended | cancelled | disputed | entered-in-error | executable +
    #[fhir(name="publicationStatus", min="1", max="1", summary=false, modifier=false, choice="")]
    pub publication_status: Option<CodeDt>,
    /// Publication Ownership
    #[fhir(name="copyright", min="0", max="1", summary=false, modifier=false, choice="")]
    pub copyright: Option<MarkdownDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractFriendlyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Easily comprehended representation of this Contract
    #[fhir(name="content", min="1", max="1", summary=false, modifier=false, choice="")]
    pub content: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ContractLegalBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Contract Legal Text
    #[fhir(name="content", min="1", max="1", summary=false, modifier=false, choice="")]
    pub content: Option<Reference>,
}


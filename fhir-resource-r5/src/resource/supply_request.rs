use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct SupplyRequest {
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
    /// Business Identifier for SupplyRequest
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | suspended +
    #[fhir(name="status", min="0", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// What other request is fulfilled by this supply request
    #[fhir(name="basedOn", min="0", max="*", summary="true", modifier="false")]
    pub based_on: Option<Vec<Reference>>,
    /// The kind of supply (central, non-stock, etc.)
    #[fhir(name="category", min="0", max="1", summary="true", modifier="false")]
    pub category: Option<CodeableConcept>,
    /// routine | urgent | asap | stat
    #[fhir(name="priority", min="0", max="1", summary="true", modifier="false")]
    pub priority: Option<CodeDt>,
    /// The patient for who the supply request is for
    #[fhir(name="deliverFor", min="0", max="1", summary="false", modifier="false")]
    pub deliver_for: Option<Reference>,
    /// Medication, Substance, or Device requested to be supplied
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableReference>,
    /// The requested amount of the item indicated
    #[fhir(name="quantity", min="1", max="1", summary="true", modifier="false")]
    pub quantity: Option<Quantity>,
    /// Ordered item details
    #[fhir(name="parameter", min="0", max="*", summary="false", modifier="false")]
    pub parameter: Option<Vec<SupplyRequestParameterBackboneElement>>,
    /// When the request should be fulfilled
    #[fhir(name="occurrence", min="0", max="1", summary="true", modifier="false")]
    pub occurrence: Option<Timing>,
    /// When the request was made
    #[fhir(name="authoredOn", min="0", max="1", summary="true", modifier="false")]
    pub authored_on: Option<DateTimeDt>,
    /// Individual making the request
    #[fhir(name="requester", min="0", max="1", summary="true", modifier="false")]
    pub requester: Option<Reference>,
    /// Who is intended to fulfill the request
    #[fhir(name="supplier", min="0", max="*", summary="true", modifier="false")]
    pub supplier: Option<Vec<Reference>>,
    /// The reason why the supply item was requested
    #[fhir(name="reason", min="0", max="*", summary="false", modifier="false")]
    pub reason: Option<Vec<CodeableReference>>,
    /// The origin of the supply
    #[fhir(name="deliverFrom", min="0", max="1", summary="false", modifier="false")]
    pub deliver_from: Option<Reference>,
    /// The destination of the supply
    #[fhir(name="deliverTo", min="0", max="1", summary="false", modifier="false")]
    pub deliver_to: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SupplyRequestParameterBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Item detail
    #[fhir(name="code", min="0", max="1", summary="false", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Value of detail
    #[fhir(name="value", min="0", max="1", summary="false", modifier="false")]
    pub value: Option<BooleanDt>,
}


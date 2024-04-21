use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SupplyDelivery {
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
    /// External identifier
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Fulfills plan, proposal or order
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub based_on: Option<Vec<Reference>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub part_of: Option<Vec<Reference>>,
    /// in-progress | completed | abandoned | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Patient for whom the item is supplied
    #[fhir(name="patient", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub patient: Option<Reference>,
    /// Category of supply event
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// The item that is delivered or supplied
    #[fhir(name="suppliedItem", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supplied_item: Option<Vec<SupplyDeliverySuppliedItemBackboneElement>>,
    /// When event occurred
    #[fhir(name="occurrence", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub occurrence: Option<Timing>,
    /// The item supplier
    #[fhir(name="supplier", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub supplier: Option<Reference>,
    /// Where the delivery was sent
    #[fhir(name="destination", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub destination: Option<Reference>,
    /// Who received the delivery
    #[fhir(name="receiver", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub receiver: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SupplyDeliverySuppliedItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Amount supplied
    #[fhir(name="quantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub quantity: Option<Quantity>,
    /// Medication, Substance, Device or Biologically Derived Product supplied
    #[fhir(name="item", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub item: Option<Reference>,
}


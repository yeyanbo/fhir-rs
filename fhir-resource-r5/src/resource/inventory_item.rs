use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct InventoryItem {
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
    /// Business identifier for the inventory item
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="false")]
    pub status: Option<CodeDt>,
    /// Category or class of the item
    #[fhir(name="category", min="0", max="*", summary="true", modifier="false")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Code designating the specific type of item
    #[fhir(name="code", min="0", max="*", summary="true", modifier="false")]
    pub code: Option<Vec<CodeableConcept>>,
    /// The item name(s) - the brand name, or common name, functional name, generic name or others
    #[fhir(name="name", min="0", max="*", summary="true", modifier="false")]
    pub name: Option<Vec<InventoryItemNameBackboneElement>>,
    /// Organization(s) responsible for the product
    #[fhir(name="responsibleOrganization", min="0", max="*", summary="false", modifier="false")]
    pub responsible_organization: Option<Vec<InventoryItemResponsibleOrganizationBackboneElement>>,
    /// Descriptive characteristics of the item
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<InventoryItemDescriptionBackboneElement>,
    /// The usage status like recalled, in use, discarded
    #[fhir(name="inventoryStatus", min="0", max="*", summary="true", modifier="false")]
    pub inventory_status: Option<Vec<CodeableConcept>>,
    /// The base unit of measure - the unit in which the product is used or counted
    #[fhir(name="baseUnit", min="0", max="1", summary="true", modifier="false")]
    pub base_unit: Option<CodeableConcept>,
    /// Net content or amount present in the item
    #[fhir(name="netContent", min="0", max="1", summary="true", modifier="false")]
    pub net_content: Option<Quantity>,
    /// Association with other items or products
    #[fhir(name="association", min="0", max="*", summary="false", modifier="false")]
    pub association: Option<Vec<InventoryItemAssociationBackboneElement>>,
    /// Characteristic of the item
    #[fhir(name="characteristic", min="0", max="*", summary="false", modifier="false")]
    pub characteristic: Option<Vec<InventoryItemCharacteristicBackboneElement>>,
    /// Instances or occurrences of the product
    #[fhir(name="instance", min="0", max="1", summary="false", modifier="false")]
    pub instance: Option<InventoryItemInstanceBackboneElement>,
    /// Link to a product resource used in clinical workflows
    #[fhir(name="productReference", min="0", max="1", summary="false", modifier="false")]
    pub product_reference: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryItemNameBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of name e.g. 'brand-name', 'functional-name', 'common-name'
    #[fhir(name="nameType", min="1", max="1", summary="true", modifier="false")]
    pub name_type: Option<Coding>,
    /// The language used to express the item name
    #[fhir(name="language", min="1", max="1", summary="true", modifier="false")]
    pub language: Option<CodeDt>,
    /// The name or designation of the item
    #[fhir(name="name", min="1", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryItemAssociationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of association between the device and the other item
    #[fhir(name="associationType", min="1", max="1", summary="true", modifier="false")]
    pub association_type: Option<CodeableConcept>,
    /// The related item or product
    #[fhir(name="relatedItem", min="1", max="1", summary="true", modifier="false")]
    pub related_item: Option<Reference>,
    /// The quantity of the product in this product
    #[fhir(name="quantity", min="1", max="1", summary="true", modifier="false")]
    pub quantity: Option<Ratio>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryItemResponsibleOrganizationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The role of the organization e.g. manufacturer, distributor, or other
    #[fhir(name="role", min="1", max="1", summary="false", modifier="false")]
    pub role: Option<CodeableConcept>,
    /// An organization that is associated with the item
    #[fhir(name="organization", min="1", max="1", summary="false", modifier="false")]
    pub organization: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryItemDescriptionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The language that is used in the item description
    #[fhir(name="language", min="0", max="1", summary="false", modifier="false")]
    pub language: Option<CodeDt>,
    /// Textual description of the item
    #[fhir(name="description", min="0", max="1", summary="false", modifier="false")]
    pub description: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryItemCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The characteristic that is being defined
    #[fhir(name="characteristicType", min="1", max="1", summary="false", modifier="false")]
    pub characteristic_type: Option<CodeableConcept>,
    /// The value of the attribute
    #[fhir(name="value", min="1", max="1", summary="false", modifier="false")]
    pub value: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct InventoryItemInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The identifier for the physical instance, typically a serial number
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// The lot or batch number of the item
    #[fhir(name="lotNumber", min="0", max="1", summary="false", modifier="false")]
    pub lot_number: Option<StringDt>,
    /// The expiry date or date and time for the product
    #[fhir(name="expiry", min="0", max="1", summary="false", modifier="false")]
    pub expiry: Option<DateTimeDt>,
    /// The subject that the item is associated with
    #[fhir(name="subject", min="0", max="1", summary="false", modifier="false")]
    pub subject: Option<Reference>,
    /// The location that the item is associated with
    #[fhir(name="location", min="0", max="1", summary="false", modifier="false")]
    pub location: Option<Reference>,
}


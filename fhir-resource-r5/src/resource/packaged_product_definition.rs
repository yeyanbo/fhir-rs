use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct PackagedProductDefinition {
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
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A unique identifier for this package as whole - not for the content of the package
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// A name for this package. Typically as listed in a drug formulary, catalogue, inventory etc
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// A high level category e.g. medicinal product, raw material, shipping container etc
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The product that this is a pack for
    #[fhir(name="packageFor", min="0", max="*", summary="true", modifier="false")]
    pub package_for: Option<Vec<Reference>>,
    /// The status within the lifecycle of this item. High level - not intended to duplicate details elsewhere e.g. legal status, or authorization/marketing status
    #[fhir(name="status", min="0", max="1", summary="true", modifier="true")]
    pub status: Option<CodeableConcept>,
    /// The date at which the given status became applicable
    #[fhir(name="statusDate", min="0", max="1", summary="true", modifier="false")]
    pub status_date: Option<DateTimeDt>,
    /// A total of the complete count of contained items of a particular type/form, independent of sub-packaging or organization. This can be considered as the pack size. See also packaging.containedItem.amount (especially the long definition)
    #[fhir(name="containedItemQuantity", min="0", max="*", summary="true", modifier="false")]
    pub contained_item_quantity: Option<Vec<Quantity>>,
    /// Textual description. Note that this is not the name of the package or product
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The legal status of supply of the packaged item as classified by the regulator
    #[fhir(name="legalStatusOfSupply", min="0", max="*", summary="true", modifier="false")]
    pub legal_status_of_supply: Option<Vec<PackagedProductDefinitionLegalStatusOfSupplyBackboneElement>>,
    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    #[fhir(name="marketingStatus", min="0", max="*", summary="true", modifier="false")]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// Identifies if the drug product is supplied with another item such as a diluent or adjuvant
    #[fhir(name="copackagedIndicator", min="0", max="1", summary="true", modifier="false")]
    pub copackaged_indicator: Option<BooleanDt>,
    /// Manufacturer of this package type (multiple means these are all possible manufacturers)
    #[fhir(name="manufacturer", min="0", max="*", summary="true", modifier="false")]
    pub manufacturer: Option<Vec<Reference>>,
    /// Additional information or supporting documentation about the packaged product
    #[fhir(name="attachedDocument", min="0", max="*", summary="true", modifier="false")]
    pub attached_document: Option<Vec<Reference>>,
    /// A packaging item, as a container for medically related items, possibly with other packaging items within, or a packaging component, such as bottle cap
    #[fhir(name="packaging", min="0", max="1", summary="true", modifier="false")]
    pub packaging: Option<PackagedProductDefinitionPackagingBackboneElement>,
    /// Allows the key features to be recorded, such as "hospital pack", "nurse prescribable"
    #[fhir(name="characteristic", min="0", max="*", summary="true", modifier="false")]
    pub characteristic: Option<Vec<PackagedProductDefinitionPackagingPropertyBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PackagedProductDefinitionLegalStatusOfSupplyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The actual status of supply. In what situation this package type may be supplied for use
    #[fhir(name="code", min="0", max="1", summary="true", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// The place where the legal status of supply applies
    #[fhir(name="jurisdiction", min="0", max="1", summary="true", modifier="false")]
    pub jurisdiction: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PackagedProductDefinitionPackagingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// An identifier that is specific to this particular part of the packaging. Including possibly a Data Carrier Identifier
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// The physical type of the container of the items
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// Is this a part of the packaging (e.g. a cap or bottle stopper), rather than the packaging itself (e.g. a bottle or vial)
    #[fhir(name="componentPart", min="0", max="1", summary="true", modifier="false")]
    pub component_part: Option<BooleanDt>,
    /// The quantity of this level of packaging in the package that contains it (with the outermost level being 1)
    #[fhir(name="quantity", min="0", max="1", summary="true", modifier="false")]
    pub quantity: Option<IntegerDt>,
    /// Material type of the package item
    #[fhir(name="material", min="0", max="*", summary="true", modifier="false")]
    pub material: Option<Vec<CodeableConcept>>,
    /// A possible alternate material for this part of the packaging, that is allowed to be used instead of the usual material
    #[fhir(name="alternateMaterial", min="0", max="*", summary="true", modifier="false")]
    pub alternate_material: Option<Vec<CodeableConcept>>,
    /// Shelf Life and storage information
    #[fhir(name="shelfLifeStorage", min="0", max="*", summary="true", modifier="false")]
    pub shelf_life_storage: Option<Vec<ProductShelfLife>>,
    /// Manufacturer of this packaging item (multiple means these are all potential manufacturers)
    #[fhir(name="manufacturer", min="0", max="*", summary="true", modifier="false")]
    pub manufacturer: Option<Vec<Reference>>,
    /// General characteristics of this item
    #[fhir(name="property", min="0", max="*", summary="true", modifier="false")]
    pub property: Option<Vec<PackagedProductDefinitionPackagingPropertyBackboneElement>>,
    /// The item(s) within the packaging
    #[fhir(name="containedItem", min="0", max="*", summary="true", modifier="false")]
    pub contained_item: Option<Vec<PackagedProductDefinitionPackagingContainedItemBackboneElement>>,
    /// Allows containers (and parts of containers) within containers, still as a part of single packaged product
    #[fhir(name="packaging", min="0", max="*", summary="true", modifier="false")]
    pub packaging: Option<Vec<PackagedProductDefinitionPackagingBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PackagedProductDefinitionPackagingContainedItemBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The actual item(s) of medication, as manufactured, or a device, or other medically related item (food, biologicals, raw materials, medical fluids, gases etc.), as contained in the package
    #[fhir(name="item", min="1", max="1", summary="true", modifier="false")]
    pub item: Option<CodeableReference>,
    /// The number of this type of item within this packaging or for continuous items such as liquids it is the quantity (for example 25ml). See also PackagedProductDefinition.containedItemQuantity (especially the long definition)
    #[fhir(name="amount", min="0", max="1", summary="true", modifier="false")]
    pub amount: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PackagedProductDefinitionPackagingPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A code expressing the type of characteristic
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// A value for the characteristic
    #[fhir(name="value", min="0", max="1", summary="true", modifier="false")]
    pub value: Option<Attachment>,
}


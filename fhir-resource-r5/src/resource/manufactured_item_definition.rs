use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct ManufacturedItemDefinition {
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
    /// Unique identifier
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// A descriptive name applied to this item
    #[fhir(name="name", min="0", max="1", summary="true", modifier="false")]
    pub name: Option<StringDt>,
    /// Dose form as manufactured (before any necessary transformation)
    #[fhir(name="manufacturedDoseForm", min="1", max="1", summary="true", modifier="false")]
    pub manufactured_dose_form: Option<CodeableConcept>,
    /// The “real-world” units in which the quantity of the item is described
    #[fhir(name="unitOfPresentation", min="0", max="1", summary="true", modifier="false")]
    pub unit_of_presentation: Option<CodeableConcept>,
    /// Manufacturer of the item, one of several possible
    #[fhir(name="manufacturer", min="0", max="*", summary="true", modifier="false")]
    pub manufacturer: Option<Vec<Reference>>,
    /// Allows specifying that an item is on the market for sale, or that it is not available, and the dates and locations associated
    #[fhir(name="marketingStatus", min="0", max="*", summary="true", modifier="false")]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// The ingredients of this manufactured item. Only needed if these are not specified by incoming references from the Ingredient resource
    #[fhir(name="ingredient", min="0", max="*", summary="true", modifier="false")]
    pub ingredient: Option<Vec<CodeableConcept>>,
    /// General characteristics of this item
    #[fhir(name="property", min="0", max="*", summary="true", modifier="false")]
    pub property: Option<Vec<ManufacturedItemDefinitionPropertyBackboneElement>>,
    /// Physical parts of the manufactured item, that it is intrisically made from. This is distinct from the ingredients that are part of its chemical makeup
    #[fhir(name="component", min="0", max="*", summary="true", modifier="false")]
    pub component: Option<Vec<ManufacturedItemDefinitionComponentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ManufacturedItemDefinitionComponentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Defining type of the component e.g. shell, layer, ink
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The function of this component within the item e.g. delivers active ingredient, masks taste
    #[fhir(name="function", min="0", max="*", summary="true", modifier="false")]
    pub function: Option<Vec<CodeableConcept>>,
    /// The measurable amount of total quantity of all substances in the component, expressable in different ways (e.g. by mass or volume)
    #[fhir(name="amount", min="0", max="*", summary="true", modifier="false")]
    pub amount: Option<Vec<Quantity>>,
    /// A reference to a constituent of the manufactured item as a whole, linked here so that its component location within the item can be indicated. This not where the item's ingredient are primarily stated (for which see Ingredient.for or ManufacturedItemDefinition.ingredient)
    #[fhir(name="constituent", min="0", max="*", summary="true", modifier="false")]
    pub constituent: Option<Vec<ManufacturedItemDefinitionComponentConstituentBackboneElement>>,
    /// General characteristics of this component
    #[fhir(name="property", min="0", max="*", summary="true", modifier="false")]
    pub property: Option<Vec<ManufacturedItemDefinitionPropertyBackboneElement>>,
    /// A component that this component contains or is made from
    #[fhir(name="component", min="0", max="*", summary="true", modifier="false")]
    pub component: Option<Vec<ManufacturedItemDefinitionComponentBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ManufacturedItemDefinitionComponentConstituentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The measurable amount of the substance, expressable in different ways (e.g. by mass or volume)
    #[fhir(name="amount", min="0", max="*", summary="true", modifier="false")]
    pub amount: Option<Vec<Quantity>>,
    /// The physical location of the constituent/ingredient within the component
    #[fhir(name="location", min="0", max="*", summary="true", modifier="false")]
    pub location: Option<Vec<CodeableConcept>>,
    /// The function of this constituent within the component e.g. binder
    #[fhir(name="function", min="0", max="*", summary="true", modifier="false")]
    pub function: Option<Vec<CodeableConcept>>,
    /// The ingredient that is the constituent of the given component
    #[fhir(name="hasIngredient", min="0", max="*", summary="true", modifier="false")]
    pub has_ingredient: Option<Vec<CodeableReference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ManufacturedItemDefinitionPropertyBackboneElement {
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
    pub value: Option<Reference>,
}


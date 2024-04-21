use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct AdministrableProductDefinition {
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
    /// An identifier for the administrable product
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | retired | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// References a product from which one or more of the constituent parts of that product can be prepared and used as described by this administrable product
    #[fhir(name="formOf", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub form_of: Option<Vec<Reference>>,
    /// The dose form of the final product after necessary reconstitution or processing
    #[fhir(name="administrableDoseForm", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub administrable_dose_form: Option<CodeableConcept>,
    /// The presentation type in which this item is given to a patient. e.g. for a spray - 'puff'
    #[fhir(name="unitOfPresentation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub unit_of_presentation: Option<CodeableConcept>,
    /// Indicates the specific manufactured items that are part of the 'formOf' product that are used in the preparation of this specific administrable form
    #[fhir(name="producedFrom", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub produced_from: Option<Vec<Reference>>,
    /// The ingredients of this administrable medicinal product. This is only needed if the ingredients are not specified either using ManufacturedItemDefiniton, or using by incoming references from the Ingredient resource
    #[fhir(name="ingredient", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub ingredient: Option<Vec<CodeableConcept>>,
    /// A device that is integral to the medicinal product, in effect being considered as an "ingredient" of the medicinal product
    #[fhir(name="device", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub device: Option<Reference>,
    /// A general description of the product, when in its final form, suitable for administration e.g. effervescent blue liquid, to be swallowed
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub description: Option<MarkdownDt>,
    /// Characteristics e.g. a product's onset of action
    #[fhir(name="property", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub property: Option<Vec<AdministrableProductDefinitionPropertyBackboneElement>>,
    /// The path by which the product is taken into or makes contact with the body
    #[fhir(name="routeOfAdministration", min="1", max="*", summary=true, modifier=false, choice=false)]
    pub route_of_administration: Option<Vec<AdministrableProductDefinitionRouteOfAdministrationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdministrableProductDefinitionPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A code expressing the type of characteristic
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// A value for the characteristic
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub value: Option<Reference>,
    /// The status of characteristic e.g. assigned or pending
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub status: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdministrableProductDefinitionRouteOfAdministrationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Coded expression for the route
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// The first dose (dose quantity) administered can be specified for the product
    #[fhir(name="firstDose", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub first_dose: Option<Quantity>,
    /// The maximum single dose that can be administered
    #[fhir(name="maxSingleDose", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub max_single_dose: Option<Quantity>,
    /// The maximum dose quantity to be administered in any one 24-h period
    #[fhir(name="maxDosePerDay", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub max_dose_per_day: Option<Quantity>,
    /// The maximum dose per treatment period that can be administered
    #[fhir(name="maxDosePerTreatmentPeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub max_dose_per_treatment_period: Option<Ratio>,
    /// The maximum treatment period during which the product can be administered
    #[fhir(name="maxTreatmentPeriod", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub max_treatment_period: Option<Duration>,
    /// A species for which this route applies
    #[fhir(name="targetSpecies", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub target_species: Option<Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Coded expression for the species
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub code: Option<CodeableConcept>,
    /// A species specific time during which consumption of animal product is not appropriate
    #[fhir(name="withdrawalPeriod", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub withdrawal_period: Option<Vec<AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriodBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct AdministrableProductDefinitionRouteOfAdministrationTargetSpeciesWithdrawalPeriodBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of tissue for which the withdrawal period applies, e.g. meat, milk
    #[fhir(name="tissue", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub tissue: Option<CodeableConcept>,
    /// A value for the time
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub value: Option<Quantity>,
    /// Extra information about the withdrawal period
    #[fhir(name="supportingInformation", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub supporting_information: Option<StringDt>,
}


use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct SubstanceSourceMaterial {
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
    /// General high level classification of the source material specific to the origin of the material
    #[fhir(name="sourceMaterialClass", min="0", max="1", summary="true", modifier="false")]
    pub source_material_class: Option<CodeableConcept>,
    /// The type of the source material shall be specified based on a controlled vocabulary. For vaccines, this subclause refers to the class of infectious agent
    #[fhir(name="sourceMaterialType", min="0", max="1", summary="true", modifier="false")]
    pub source_material_type: Option<CodeableConcept>,
    /// The state of the source material when extracted
    #[fhir(name="sourceMaterialState", min="0", max="1", summary="true", modifier="false")]
    pub source_material_state: Option<CodeableConcept>,
    /// The unique identifier associated with the source material parent organism shall be specified
    #[fhir(name="organismId", min="0", max="1", summary="true", modifier="false")]
    pub organism_id: Option<Identifier>,
    /// The organism accepted Scientific name shall be provided based on the organism taxonomy
    #[fhir(name="organismName", min="0", max="1", summary="true", modifier="false")]
    pub organism_name: Option<StringDt>,
    /// The parent of the herbal drug Ginkgo biloba, Leaf is the substance ID of the substance (fresh) of Ginkgo biloba L. or Ginkgo biloba L. (Whole plant)
    #[fhir(name="parentSubstanceId", min="0", max="*", summary="true", modifier="false")]
    pub parent_substance_id: Option<Vec<Identifier>>,
    /// The parent substance of the Herbal Drug, or Herbal preparation
    #[fhir(name="parentSubstanceName", min="0", max="*", summary="true", modifier="false")]
    pub parent_substance_name: Option<Vec<StringDt>>,
    /// The country where the plant material is harvested or the countries where the plasma is sourced from as laid down in accordance with the Plasma Master File. For “Plasma-derived substances” the attribute country of origin provides information about the countries used for the manufacturing of the Cryopoor plama or Crioprecipitate
    #[fhir(name="countryOfOrigin", min="0", max="*", summary="true", modifier="false")]
    pub country_of_origin: Option<Vec<CodeableConcept>>,
    /// The place/region where the plant is harvested or the places/regions where the animal source material has its habitat
    #[fhir(name="geographicalLocation", min="0", max="*", summary="true", modifier="false")]
    pub geographical_location: Option<Vec<StringDt>>,
    /// Stage of life for animals, plants, insects and microorganisms. This information shall be provided only when the substance is significantly different in these stages (e.g. foetal bovine serum)
    #[fhir(name="developmentStage", min="0", max="1", summary="true", modifier="false")]
    pub development_stage: Option<CodeableConcept>,
    /// Many complex materials are fractions of parts of plants, animals, or minerals. Fraction elements are often necessary to define both Substances and Specified Group 1 Substances. For substances derived from Plants, fraction information will be captured at the Substance information level ( . Oils, Juices and Exudates). Additional information for Extracts, such as extraction solvent composition, will be captured at the Specified Substance Group 1 information level. For plasma-derived products fraction information will be captured at the Substance and the Specified Substance Group 1 levels
    #[fhir(name="fractionDescription", min="0", max="*", summary="true", modifier="false")]
    pub fraction_description: Option<Vec<SubstanceSourceMaterialFractionDescriptionBackboneElement>>,
    /// This subclause describes the organism which the substance is derived from. For vaccines, the parent organism shall be specified based on these subclause elements. As an example, full taxonomy will be described for the Substance Name: ., Leaf
    #[fhir(name="organism", min="0", max="1", summary="true", modifier="false")]
    pub organism: Option<SubstanceSourceMaterialOrganismBackboneElement>,
    /// To do
    #[fhir(name="partDescription", min="0", max="*", summary="true", modifier="false")]
    pub part_description: Option<Vec<SubstanceSourceMaterialPartDescriptionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceSourceMaterialPartDescriptionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Entity of anatomical origin of source material within an organism
    #[fhir(name="part", min="0", max="1", summary="true", modifier="false")]
    pub part: Option<CodeableConcept>,
    /// The detailed anatomic location when the part can be extracted from different anatomical locations of the organism. Multiple alternative locations may apply
    #[fhir(name="partLocation", min="0", max="1", summary="true", modifier="false")]
    pub part_location: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceSourceMaterialFractionDescriptionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// This element is capturing information about the fraction of a plant part, or human plasma for fractionation
    #[fhir(name="fraction", min="0", max="1", summary="true", modifier="false")]
    pub fraction: Option<StringDt>,
    /// The specific type of the material constituting the component. For Herbal preparations the particulars of the extracts (liquid/dry) is described in Specified Substance Group 1
    #[fhir(name="materialType", min="0", max="1", summary="true", modifier="false")]
    pub material_type: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceSourceMaterialOrganismBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The family of an organism shall be specified
    #[fhir(name="family", min="0", max="1", summary="true", modifier="false")]
    pub family: Option<CodeableConcept>,
    /// The genus of an organism shall be specified; refers to the Latin epithet of the genus element of the plant/animal scientific name; it is present in names for genera, species and infraspecies
    #[fhir(name="genus", min="0", max="1", summary="true", modifier="false")]
    pub genus: Option<CodeableConcept>,
    /// The species of an organism shall be specified; refers to the Latin epithet of the species of the plant/animal; it is present in names for species and infraspecies
    #[fhir(name="species", min="0", max="1", summary="true", modifier="false")]
    pub species: Option<CodeableConcept>,
    /// The Intraspecific type of an organism shall be specified
    #[fhir(name="intraspecificType", min="0", max="1", summary="true", modifier="false")]
    pub intraspecific_type: Option<CodeableConcept>,
    /// The intraspecific description of an organism shall be specified based on a controlled vocabulary. For Influenza Vaccine, the intraspecific description shall contain the syntax of the antigen in line with the WHO convention
    #[fhir(name="intraspecificDescription", min="0", max="1", summary="true", modifier="false")]
    pub intraspecific_description: Option<StringDt>,
    /// 4.9.13.6.1 Author type (Conditional)
    #[fhir(name="author", min="0", max="*", summary="true", modifier="false")]
    pub author: Option<Vec<SubstanceSourceMaterialOrganismAuthorBackboneElement>>,
    /// 4.9.13.8.1 Hybrid species maternal organism ID (Optional)
    #[fhir(name="hybrid", min="0", max="1", summary="true", modifier="false")]
    pub hybrid: Option<SubstanceSourceMaterialOrganismHybridBackboneElement>,
    /// 4.9.13.7.1 Kingdom (Conditional)
    #[fhir(name="organismGeneral", min="0", max="1", summary="true", modifier="false")]
    pub organism_general: Option<SubstanceSourceMaterialOrganismOrganismGeneralBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceSourceMaterialOrganismHybridBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The identifier of the maternal species constituting the hybrid organism shall be specified based on a controlled vocabulary. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal
    #[fhir(name="maternalOrganismId", min="0", max="1", summary="true", modifier="false")]
    pub maternal_organism_id: Option<StringDt>,
    /// The name of the maternal species constituting the hybrid organism shall be specified. For plants, the parents aren’t always known, and it is unlikely that it will be known which is maternal and which is paternal
    #[fhir(name="maternalOrganismName", min="0", max="1", summary="true", modifier="false")]
    pub maternal_organism_name: Option<StringDt>,
    /// The identifier of the paternal species constituting the hybrid organism shall be specified based on a controlled vocabulary
    #[fhir(name="paternalOrganismId", min="0", max="1", summary="true", modifier="false")]
    pub paternal_organism_id: Option<StringDt>,
    /// The name of the paternal species constituting the hybrid organism shall be specified
    #[fhir(name="paternalOrganismName", min="0", max="1", summary="true", modifier="false")]
    pub paternal_organism_name: Option<StringDt>,
    /// The hybrid type of an organism shall be specified
    #[fhir(name="hybridType", min="0", max="1", summary="true", modifier="false")]
    pub hybrid_type: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceSourceMaterialOrganismAuthorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of author of an organism species shall be specified. The parenthetical author of an organism species refers to the first author who published the plant/animal name (of any rank). The primary author of an organism species refers to the first author(s), who validly published the plant/animal name
    #[fhir(name="authorType", min="0", max="1", summary="true", modifier="false")]
    pub author_type: Option<CodeableConcept>,
    /// The author of an organism species shall be specified. The author year of an organism shall also be specified when applicable; refers to the year in which the first author(s) published the infraspecific plant/animal name (of any rank)
    #[fhir(name="authorDescription", min="0", max="1", summary="true", modifier="false")]
    pub author_description: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceSourceMaterialOrganismOrganismGeneralBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kingdom of an organism shall be specified
    #[fhir(name="kingdom", min="0", max="1", summary="true", modifier="false")]
    pub kingdom: Option<CodeableConcept>,
    /// The phylum of an organism shall be specified
    #[fhir(name="phylum", min="0", max="1", summary="true", modifier="false")]
    pub phylum: Option<CodeableConcept>,
    /// The class of an organism shall be specified
    #[fhir(name="class", min="0", max="1", summary="true", modifier="false")]
    pub class: Option<CodeableConcept>,
    /// The order of an organism shall be specified,
    #[fhir(name="order", min="0", max="1", summary="true", modifier="false")]
    pub order: Option<CodeableConcept>,
}


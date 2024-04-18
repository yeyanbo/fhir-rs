use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SubstanceDefinition {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifier by which this substance is known
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// A business level version identifier of the substance
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// Status of substance within the catalogue e.g. active, retired
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false)]
    pub status: Option<CodeableConcept>,
    /// A categorization, high level e.g. polymer or nucleic acid, or food, chemical, biological, or lower e.g. polymer linear or branch chain, or type of impurity
    #[fhir(name="classification", min="0", max="*", summary=true, modifier=false)]
    pub classification: Option<Vec<CodeableConcept>>,
    /// If the substance applies to human or veterinary use
    #[fhir(name="domain", min="0", max="1", summary=true, modifier=false)]
    pub domain: Option<CodeableConcept>,
    /// The quality standard, established benchmark, to which substance complies (e.g. USP/NF, BP)
    #[fhir(name="grade", min="0", max="*", summary=true, modifier=false)]
    pub grade: Option<Vec<CodeableConcept>>,
    /// Textual description of the substance
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Supporting literature
    #[fhir(name="informationSource", min="0", max="*", summary=true, modifier=false)]
    pub information_source: Option<Vec<Reference>>,
    /// Textual comment about the substance's catalogue or registry record
    #[fhir(name="note", min="0", max="*", summary=true, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// The entity that creates, makes, produces or fabricates the substance
    #[fhir(name="manufacturer", min="0", max="*", summary=true, modifier=false)]
    pub manufacturer: Option<Vec<Reference>>,
    /// An entity that is the source for the substance. It may be different from the manufacturer
    #[fhir(name="supplier", min="0", max="*", summary=true, modifier=false)]
    pub supplier: Option<Vec<Reference>>,
    /// Moiety, for structural modifications
    #[fhir(name="moiety", min="0", max="*", summary=true, modifier=false)]
    pub moiety: Option<Vec<SubstanceDefinitionMoietyBackboneElement>>,
    /// General specifications for this substance
    #[fhir(name="characterization", min="0", max="*", summary=true, modifier=false)]
    pub characterization: Option<Vec<SubstanceDefinitionCharacterizationBackboneElement>>,
    /// General specifications for this substance
    #[fhir(name="property", min="0", max="*", summary=true, modifier=false)]
    pub property: Option<Vec<SubstanceDefinitionPropertyBackboneElement>>,
    /// General information detailing this substance
    #[fhir(name="referenceInformation", min="0", max="1", summary=true, modifier=false)]
    pub reference_information: Option<Reference>,
    /// The average mass of a molecule of a compound
    #[fhir(name="molecularWeight", min="0", max="*", summary=true, modifier=false)]
    pub molecular_weight: Option<Vec<SubstanceDefinitionMolecularWeightBackboneElement>>,
    /// Structural information
    #[fhir(name="structure", min="0", max="1", summary=true, modifier=false)]
    pub structure: Option<SubstanceDefinitionStructureBackboneElement>,
    /// Codes associated with the substance
    #[fhir(name="code", min="0", max="*", summary=true, modifier=false)]
    pub code: Option<Vec<SubstanceDefinitionCodeBackboneElement>>,
    /// Names applicable to this substance
    #[fhir(name="name", min="0", max="*", summary=true, modifier=false)]
    pub name: Option<Vec<SubstanceDefinitionNameBackboneElement>>,
    /// A link between this substance and another
    #[fhir(name="relationship", min="0", max="*", summary=true, modifier=false)]
    pub relationship: Option<Vec<SubstanceDefinitionRelationshipBackboneElement>>,
    /// Data items specific to nucleic acids
    #[fhir(name="nucleicAcid", min="0", max="1", summary=true, modifier=false)]
    pub nucleic_acid: Option<Reference>,
    /// Data items specific to polymers
    #[fhir(name="polymer", min="0", max="1", summary=true, modifier=false)]
    pub polymer: Option<Reference>,
    /// Data items specific to proteins
    #[fhir(name="protein", min="0", max="1", summary=true, modifier=false)]
    pub protein: Option<Reference>,
    /// Material or taxonomic/anatomical source
    #[fhir(name="sourceMaterial", min="0", max="1", summary=true, modifier=false)]
    pub source_material: Option<SubstanceDefinitionSourceMaterialBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionNameBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The actual name
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Name type e.g. 'systematic',  'scientific, 'brand'
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The status of the name e.g. 'current', 'proposed'
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false)]
    pub status: Option<CodeableConcept>,
    /// If this is the preferred name for this substance
    #[fhir(name="preferred", min="0", max="1", summary=true, modifier=false)]
    pub preferred: Option<BooleanDt>,
    /// Human language that the name is written in
    #[fhir(name="language", min="0", max="*", summary=true, modifier=false)]
    pub language: Option<Vec<CodeableConcept>>,
    /// The use context of this name e.g. as an active ingredient or as a food colour additive
    #[fhir(name="domain", min="0", max="*", summary=true, modifier=false)]
    pub domain: Option<Vec<CodeableConcept>>,
    /// The jurisdiction where this name applies
    #[fhir(name="jurisdiction", min="0", max="*", summary=true, modifier=false)]
    pub jurisdiction: Option<Vec<CodeableConcept>>,
    /// A synonym of this particular name, by which the substance is also known
    #[fhir(name="synonym", min="0", max="*", summary=true, modifier=false)]
    pub synonym: Option<Vec<SubstanceDefinitionNameBackboneElement>>,
    /// A translation for this name into another human language
    #[fhir(name="translation", min="0", max="*", summary=true, modifier=false)]
    pub translation: Option<Vec<SubstanceDefinitionNameBackboneElement>>,
    /// Details of the official nature of this name
    #[fhir(name="official", min="0", max="*", summary=true, modifier=false)]
    pub official: Option<Vec<SubstanceDefinitionNameOfficialBackboneElement>>,
    /// Supporting literature
    #[fhir(name="source", min="0", max="*", summary=true, modifier=false)]
    pub source: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionNameOfficialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Which authority uses this official name
    #[fhir(name="authority", min="0", max="1", summary=true, modifier=false)]
    pub authority: Option<CodeableConcept>,
    /// The status of the official name, for example 'draft', 'active'
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false)]
    pub status: Option<CodeableConcept>,
    /// Date of official name change
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false)]
    pub date: Option<DateTimeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A code expressing the type of property
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// A value for the property
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<Attachment>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionCharacterizationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The method used to find the characterization e.g. HPLC
    #[fhir(name="technique", min="0", max="1", summary=true, modifier=false)]
    pub technique: Option<CodeableConcept>,
    /// Describes the nature of the chemical entity and explains, for instance, whether this is a base or a salt form
    #[fhir(name="form", min="0", max="1", summary=true, modifier=false)]
    pub form: Option<CodeableConcept>,
    /// The description or justification in support of the interpretation of the data file
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The data produced by the analytical instrument or a pictorial representation of that data. Examples: a JCAMP, JDX, or ADX file, or a chromatogram or spectrum analysis
    #[fhir(name="file", min="0", max="*", summary=true, modifier=false)]
    pub file: Option<Vec<Attachment>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionStructureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Stereochemistry type
    #[fhir(name="stereochemistry", min="0", max="1", summary=true, modifier=false)]
    pub stereochemistry: Option<CodeableConcept>,
    /// Optical activity type
    #[fhir(name="opticalActivity", min="0", max="1", summary=true, modifier=false)]
    pub optical_activity: Option<CodeableConcept>,
    /// An expression which states the number and type of atoms present in a molecule of a substance
    #[fhir(name="molecularFormula", min="0", max="1", summary=true, modifier=false)]
    pub molecular_formula: Option<StringDt>,
    /// Specified per moiety according to the Hill system
    #[fhir(name="molecularFormulaByMoiety", min="0", max="1", summary=true, modifier=false)]
    pub molecular_formula_by_moiety: Option<StringDt>,
    /// The molecular weight or weight range
    #[fhir(name="molecularWeight", min="0", max="1", summary=true, modifier=false)]
    pub molecular_weight: Option<SubstanceDefinitionMolecularWeightBackboneElement>,
    /// The method used to find the structure e.g. X-ray, NMR
    #[fhir(name="technique", min="0", max="*", summary=true, modifier=false)]
    pub technique: Option<Vec<CodeableConcept>>,
    /// Source of information for the structure
    #[fhir(name="sourceDocument", min="0", max="*", summary=true, modifier=false)]
    pub source_document: Option<Vec<Reference>>,
    /// A depiction of the structure of the substance
    #[fhir(name="representation", min="0", max="*", summary=true, modifier=false)]
    pub representation: Option<Vec<SubstanceDefinitionStructureRepresentationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionStructureRepresentationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of structural representation (e.g. full, partial)
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The structural representation as a text string in a standard format
    #[fhir(name="representation", min="0", max="1", summary=true, modifier=false)]
    pub representation: Option<StringDt>,
    /// The format of the representation e.g. InChI, SMILES, MOLFILE (note: not the physical file format)
    #[fhir(name="format", min="0", max="1", summary=true, modifier=false)]
    pub format: Option<CodeableConcept>,
    /// An attachment with the structural representation e.g. a structure graphic or AnIML file
    #[fhir(name="document", min="0", max="1", summary=true, modifier=false)]
    pub document: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionCodeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The specific code
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// Status of the code assignment, for example 'provisional', 'approved'
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false)]
    pub status: Option<CodeableConcept>,
    /// The date at which the code status was changed
    #[fhir(name="statusDate", min="0", max="1", summary=true, modifier=false)]
    pub status_date: Option<DateTimeDt>,
    /// Any comment can be provided in this field
    #[fhir(name="note", min="0", max="*", summary=true, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Supporting literature
    #[fhir(name="source", min="0", max="*", summary=true, modifier=false)]
    pub source: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionMolecularWeightBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The method by which the weight was determined
    #[fhir(name="method", min="0", max="1", summary=true, modifier=false)]
    pub method: Option<CodeableConcept>,
    /// Type of molecular weight e.g. exact, average, weight average
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Used to capture quantitative values for a variety of elements
    #[fhir(name="amount", min="1", max="1", summary=true, modifier=false)]
    pub amount: Option<Quantity>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionSourceMaterialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Classification of the origin of the raw material. e.g. cat hair is an Animal source type
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The genus of an organism e.g. the Latin epithet of the plant/animal scientific name
    #[fhir(name="genus", min="0", max="1", summary=true, modifier=false)]
    pub genus: Option<CodeableConcept>,
    /// The species of an organism e.g. the Latin epithet of the species of the plant/animal
    #[fhir(name="species", min="0", max="1", summary=true, modifier=false)]
    pub species: Option<CodeableConcept>,
    /// An anatomical origin of the source material within an organism
    #[fhir(name="part", min="0", max="1", summary=true, modifier=false)]
    pub part: Option<CodeableConcept>,
    /// The country or countries where the material is harvested
    #[fhir(name="countryOfOrigin", min="0", max="*", summary=true, modifier=false)]
    pub country_of_origin: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionRelationshipBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A pointer to another substance, as a resource or a representational code
    #[fhir(name="substanceDefinition", min="0", max="1", summary=true, modifier=false)]
    pub substance_definition: Option<CodeableConcept>,
    /// For example "salt to parent", "active moiety"
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// For example where an enzyme strongly bonds with a particular substance, this is a defining relationship for that enzyme, out of several possible relationships
    #[fhir(name="isDefining", min="0", max="1", summary=true, modifier=false)]
    pub is_defining: Option<BooleanDt>,
    /// A numeric factor for the relationship, e.g. that a substance salt has some percentage of active substance in relation to some other
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false)]
    pub amount: Option<StringDt>,
    /// For use when the numeric has an uncertain range
    #[fhir(name="ratioHighLimitAmount", min="0", max="1", summary=true, modifier=false)]
    pub ratio_high_limit_amount: Option<Ratio>,
    /// An operator for the amount, for example "average", "approximately", "less than"
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=false)]
    pub comparator: Option<CodeableConcept>,
    /// Supporting literature
    #[fhir(name="source", min="0", max="*", summary=true, modifier=false)]
    pub source: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceDefinitionMoietyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Role that the moiety is playing
    #[fhir(name="role", min="0", max="1", summary=true, modifier=false)]
    pub role: Option<CodeableConcept>,
    /// Identifier by which this moiety substance is known
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false)]
    pub identifier: Option<Identifier>,
    /// Textual name for this moiety substance
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Stereochemistry type
    #[fhir(name="stereochemistry", min="0", max="1", summary=true, modifier=false)]
    pub stereochemistry: Option<CodeableConcept>,
    /// Optical activity type
    #[fhir(name="opticalActivity", min="0", max="1", summary=true, modifier=false)]
    pub optical_activity: Option<CodeableConcept>,
    /// Molecular formula for this moiety (e.g. with the Hill system)
    #[fhir(name="molecularFormula", min="0", max="1", summary=true, modifier=false)]
    pub molecular_formula: Option<StringDt>,
    /// Quantitative value for this moiety
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false)]
    pub amount: Option<StringDt>,
    /// The measurement type of the quantitative value
    #[fhir(name="measurementType", min="0", max="1", summary=true, modifier=false)]
    pub measurement_type: Option<CodeableConcept>,
}


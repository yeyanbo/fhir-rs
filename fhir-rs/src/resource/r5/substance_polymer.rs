use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SubstancePolymer {
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
    /// A business idenfier for this polymer, but typically this is handled by a SubstanceDefinition identifier
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false, choice="")]
    pub identifier: Option<Identifier>,
    /// Overall type of the polymer
    #[fhir(name="class", min="0", max="1", summary=true, modifier=false, choice="")]
    pub class: Option<CodeableConcept>,
    /// Polymer geometry, e.g. linear, branched, cross-linked, network or dendritic
    #[fhir(name="geometry", min="0", max="1", summary=true, modifier=false, choice="")]
    pub geometry: Option<CodeableConcept>,
    /// Descrtibes the copolymer sequence type (polymer connectivity)
    #[fhir(name="copolymerConnectivity", min="0", max="*", summary=true, modifier=false, choice="")]
    pub copolymer_connectivity: Option<Vec<CodeableConcept>>,
    /// Todo - this is intended to connect to a repeating full modification structure, also used by Protein and Nucleic Acid . String is just a placeholder
    #[fhir(name="modification", min="0", max="1", summary=true, modifier=false, choice="")]
    pub modification: Option<StringDt>,
    /// Todo
    #[fhir(name="monomerSet", min="0", max="*", summary=true, modifier=false, choice="")]
    pub monomer_set: Option<Vec<SubstancePolymerMonomerSetBackboneElement>>,
    /// Specifies and quantifies the repeated units and their configuration
    #[fhir(name="repeat", min="0", max="*", summary=true, modifier=false, choice="")]
    pub repeat: Option<Vec<SubstancePolymerRepeatBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstancePolymerRepeatBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A representation of an (average) molecular formula from a polymer
    #[fhir(name="averageMolecularFormula", min="0", max="1", summary=true, modifier=false, choice="")]
    pub average_molecular_formula: Option<StringDt>,
    /// How the quantitative amount of Structural Repeat Units is captured (e.g. Exact, Numeric, Average)
    #[fhir(name="repeatUnitAmountType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub repeat_unit_amount_type: Option<CodeableConcept>,
    /// An SRU - Structural Repeat Unit
    #[fhir(name="repeatUnit", min="0", max="*", summary=true, modifier=false, choice="")]
    pub repeat_unit: Option<Vec<SubstancePolymerRepeatRepeatUnitBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstancePolymerRepeatRepeatUnitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Structural repeat units are essential elements for defining polymers
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false, choice="")]
    pub unit: Option<StringDt>,
    /// The orientation of the polymerisation, e.g. head-tail, head-head, random
    #[fhir(name="orientation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub orientation: Option<CodeableConcept>,
    /// Number of repeats of this unit
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false, choice="")]
    pub amount: Option<IntegerDt>,
    /// Applies to homopolymer and block co-polymers where the degree of polymerisation within a block can be described
    #[fhir(name="degreeOfPolymerisation", min="0", max="*", summary=true, modifier=false, choice="")]
    pub degree_of_polymerisation: Option<Vec<SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisationBackboneElement>>,
    /// A graphical structure for this SRU
    #[fhir(name="structuralRepresentation", min="0", max="*", summary=true, modifier=false, choice="")]
    pub structural_representation: Option<Vec<SubstancePolymerRepeatRepeatUnitStructuralRepresentationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstancePolymerRepeatRepeatUnitDegreeOfPolymerisationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of the degree of polymerisation shall be described, e.g. SRU/Polymer Ratio
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// An average amount of polymerisation
    #[fhir(name="average", min="0", max="1", summary=true, modifier=false, choice="")]
    pub average: Option<IntegerDt>,
    /// A low expected limit of the amount
    #[fhir(name="low", min="0", max="1", summary=true, modifier=false, choice="")]
    pub low: Option<IntegerDt>,
    /// A high expected limit of the amount
    #[fhir(name="high", min="0", max="1", summary=true, modifier=false, choice="")]
    pub high: Option<IntegerDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstancePolymerRepeatRepeatUnitStructuralRepresentationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of structure (e.g. Full, Partial, Representative)
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// The structural representation as text string in a standard format e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    #[fhir(name="representation", min="0", max="1", summary=true, modifier=false, choice="")]
    pub representation: Option<StringDt>,
    /// The format of the representation e.g. InChI, SMILES, MOLFILE, CDX, SDF, PDB, mmCIF
    #[fhir(name="format", min="0", max="1", summary=true, modifier=false, choice="")]
    pub format: Option<CodeableConcept>,
    /// An attached file with the structural representation
    #[fhir(name="attachment", min="0", max="1", summary=true, modifier=false, choice="")]
    pub attachment: Option<Attachment>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstancePolymerMonomerSetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Captures the type of ratio to the entire polymer, e.g. Monomer/Polymer ratio, SRU/Polymer Ratio
    #[fhir(name="ratioType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub ratio_type: Option<CodeableConcept>,
    /// The starting materials - monomer(s) used in the synthesis of the polymer
    #[fhir(name="startingMaterial", min="0", max="*", summary=true, modifier=false, choice="")]
    pub starting_material: Option<Vec<SubstancePolymerMonomerSetStartingMaterialBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstancePolymerMonomerSetStartingMaterialBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of substance for this starting material
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Substance high level category, e.g. chemical substance
    #[fhir(name="category", min="0", max="1", summary=true, modifier=false, choice="")]
    pub category: Option<CodeableConcept>,
    /// Used to specify whether the attribute described is a defining element for the unique identification of the polymer
    #[fhir(name="isDefining", min="0", max="1", summary=true, modifier=false, choice="")]
    pub is_defining: Option<BooleanDt>,
    /// A percentage
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false, choice="")]
    pub amount: Option<Quantity>,
}


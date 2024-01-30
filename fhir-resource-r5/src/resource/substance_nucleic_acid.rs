use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SubstanceNucleicAcid {
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
    /// The type of the sequence shall be specified based on a controlled vocabulary
    #[fhir(name="sequenceType", min="0", max="1", summary=true, modifier=false)]
    pub sequence_type: Option<CodeableConcept>,
    /// The number of linear sequences of nucleotides linked through phosphodiester bonds shall be described. Subunits would be strands of nucleic acids that are tightly associated typically through Watson-Crick base pairing. NOTE: If not specified in the reference source, the assumption is that there is 1 subunit
    #[fhir(name="numberOfSubunits", min="0", max="1", summary=true, modifier=false)]
    pub number_of_subunits: Option<IntegerDt>,
    /// The area of hybridisation shall be described if applicable for double stranded RNA or DNA. The number associated with the subunit followed by the number associated to the residue shall be specified in increasing order. The underscore “” shall be used as separator as follows: “Subunitnumber Residue”
    #[fhir(name="areaOfHybridisation", min="0", max="1", summary=true, modifier=false)]
    pub area_of_hybridisation: Option<StringDt>,
    /// (TBC)
    #[fhir(name="oligoNucleotideType", min="0", max="1", summary=true, modifier=false)]
    pub oligo_nucleotide_type: Option<CodeableConcept>,
    /// Subunits are listed in order of decreasing length; sequences of the same length will be ordered by molecular weight; subunits that have identical sequences will be repeated multiple times
    #[fhir(name="subunit", min="0", max="*", summary=true, modifier=false)]
    pub subunit: Option<Vec<SubstanceNucleicAcidSubunitBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceNucleicAcidSubunitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Index of linear sequences of nucleic acids in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts
    #[fhir(name="subunit", min="0", max="1", summary=true, modifier=false)]
    pub subunit: Option<IntegerDt>,
    /// Actual nucleotide sequence notation from 5' to 3' end using standard single letter codes. In addition to the base sequence, sugar and type of phosphate or non-phosphate linkage should also be captured
    #[fhir(name="sequence", min="0", max="1", summary=true, modifier=false)]
    pub sequence: Option<StringDt>,
    /// The length of the sequence shall be captured
    #[fhir(name="length", min="0", max="1", summary=true, modifier=false)]
    pub length: Option<IntegerDt>,
    /// (TBC)
    #[fhir(name="sequenceAttachment", min="0", max="1", summary=true, modifier=false)]
    pub sequence_attachment: Option<Attachment>,
    /// The nucleotide present at the 5’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the first position in the sequence. A separate representation would be redundant
    #[fhir(name="fivePrime", min="0", max="1", summary=true, modifier=false)]
    pub five_prime: Option<CodeableConcept>,
    /// The nucleotide present at the 3’ terminal shall be specified based on a controlled vocabulary. Since the sequence is represented from the 5' to the 3' end, the 5’ prime nucleotide is the letter at the last position in the sequence. A separate representation would be redundant
    #[fhir(name="threePrime", min="0", max="1", summary=true, modifier=false)]
    pub three_prime: Option<CodeableConcept>,
    /// The linkages between sugar residues will also be captured
    #[fhir(name="linkage", min="0", max="*", summary=true, modifier=false)]
    pub linkage: Option<Vec<SubstanceNucleicAcidSubunitLinkageBackboneElement>>,
    /// 5.3.6.8.1 Sugar ID (Mandatory)
    #[fhir(name="sugar", min="0", max="*", summary=true, modifier=false)]
    pub sugar: Option<Vec<SubstanceNucleicAcidSubunitSugarBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceNucleicAcidSubunitLinkageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The entity that links the sugar residues together should also be captured for nearly all naturally occurring nucleic acid the linkage is a phosphate group. For many synthetic oligonucleotides phosphorothioate linkages are often seen. Linkage connectivity is assumed to be 3’-5’. If the linkage is either 3’-3’ or 5’-5’ this should be specified
    #[fhir(name="connectivity", min="0", max="1", summary=true, modifier=false)]
    pub connectivity: Option<StringDt>,
    /// Each linkage will be registered as a fragment and have an ID
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false)]
    pub identifier: Option<Identifier>,
    /// Each linkage will be registered as a fragment and have at least one name. A single name shall be assigned to each linkage
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Residues shall be captured as described in 5.3.6.8.3
    #[fhir(name="residueSite", min="0", max="1", summary=true, modifier=false)]
    pub residue_site: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceNucleicAcidSubunitSugarBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The Substance ID of the sugar or sugar-like component that make up the nucleotide
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false)]
    pub identifier: Option<Identifier>,
    /// The name of the sugar or sugar-like component that make up the nucleotide
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// The residues that contain a given sugar will be captured. The order of given residues will be captured in the 5‘-3‘direction consistent with the base sequences listed above
    #[fhir(name="residueSite", min="0", max="1", summary=true, modifier=false)]
    pub residue_site: Option<StringDt>,
}


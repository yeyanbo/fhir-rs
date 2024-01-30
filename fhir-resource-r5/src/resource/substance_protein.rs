use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SubstanceProtein {
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
    /// The SubstanceProtein descriptive elements will only be used when a complete or partial amino acid sequence is available or derivable from a nucleic acid sequence
    #[fhir(name="sequenceType", min="0", max="1", summary=true, modifier=false)]
    pub sequence_type: Option<CodeableConcept>,
    /// Number of linear sequences of amino acids linked through peptide bonds. The number of subunits constituting the SubstanceProtein shall be described. It is possible that the number of subunits can be variable
    #[fhir(name="numberOfSubunits", min="0", max="1", summary=true, modifier=false)]
    pub number_of_subunits: Option<IntegerDt>,
    /// The disulphide bond between two cysteine residues either on the same subunit or on two different subunits shall be described. The position of the disulfide bonds in the SubstanceProtein shall be listed in increasing order of subunit number and position within subunit followed by the abbreviation of the amino acids involved. The disulfide linkage positions shall actually contain the amino acid Cysteine at the respective positions
    #[fhir(name="disulfideLinkage", min="0", max="*", summary=true, modifier=false)]
    pub disulfide_linkage: Option<Vec<StringDt>>,
    /// This subclause refers to the description of each subunit constituting the SubstanceProtein. A subunit is a linear sequence of amino acids linked through peptide bonds. The Subunit information shall be provided when the finished SubstanceProtein is a complex of multiple sequences; subunits are not used to delineate domains within a single sequence. Subunits are listed in order of decreasing length; sequences of the same length will be ordered by decreasing molecular weight; subunits that have identical sequences will be repeated multiple times
    #[fhir(name="subunit", min="0", max="*", summary=true, modifier=false)]
    pub subunit: Option<Vec<SubstanceProteinSubunitBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct SubstanceProteinSubunitBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Index of primary sequences of amino acids linked through peptide bonds in order of decreasing length. Sequences of the same length will be ordered by molecular weight. Subunits that have identical sequences will be repeated and have sequential subscripts
    #[fhir(name="subunit", min="0", max="1", summary=true, modifier=false)]
    pub subunit: Option<IntegerDt>,
    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowercase for D-amino acids. Transcribed SubstanceProteins will always be described using the translated sequence; for synthetic peptide containing amino acids that are not represented with a single letter code an X should be used within the sequence. The modified amino acids will be distinguished by their position in the sequence
    #[fhir(name="sequence", min="0", max="1", summary=true, modifier=false)]
    pub sequence: Option<StringDt>,
    /// Length of linear sequences of amino acids contained in the subunit
    #[fhir(name="length", min="0", max="1", summary=true, modifier=false)]
    pub length: Option<IntegerDt>,
    /// The sequence information shall be provided enumerating the amino acids from N- to C-terminal end using standard single-letter amino acid codes. Uppercase shall be used for L-amino acids and lowercase for D-amino acids. Transcribed SubstanceProteins will always be described using the translated sequence; for synthetic peptide containing amino acids that are not represented with a single letter code an X should be used within the sequence. The modified amino acids will be distinguished by their position in the sequence
    #[fhir(name="sequenceAttachment", min="0", max="1", summary=true, modifier=false)]
    pub sequence_attachment: Option<Attachment>,
    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    #[fhir(name="nTerminalModificationId", min="0", max="1", summary=true, modifier=false)]
    pub n_terminal_modification_id: Option<Identifier>,
    /// The name of the fragment modified at the N-terminal of the SubstanceProtein shall be specified
    #[fhir(name="nTerminalModification", min="0", max="1", summary=true, modifier=false)]
    pub n_terminal_modification: Option<StringDt>,
    /// Unique identifier for molecular fragment modification based on the ISO 11238 Substance ID
    #[fhir(name="cTerminalModificationId", min="0", max="1", summary=true, modifier=false)]
    pub c_terminal_modification_id: Option<Identifier>,
    /// The modification at the C-terminal shall be specified
    #[fhir(name="cTerminalModification", min="0", max="1", summary=true, modifier=false)]
    pub c_terminal_modification: Option<StringDt>,
}


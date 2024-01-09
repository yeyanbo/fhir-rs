use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct MolecularSequence {
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
    /// Unique ID for this particular sequence
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// aa | dna | rna
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeDt>,
    /// Subject this sequence is associated too
    #[fhir(name="subject", min="0", max="1", summary="true", modifier="false")]
    pub subject: Option<Reference>,
    /// What the molecular sequence is about, when it is not about the subject of record
    #[fhir(name="focus", min="0", max="*", summary="true", modifier="false")]
    pub focus: Option<Vec<Reference>>,
    /// Specimen used for sequencing
    #[fhir(name="specimen", min="0", max="1", summary="true", modifier="false")]
    pub specimen: Option<Reference>,
    /// The method for sequencing
    #[fhir(name="device", min="0", max="1", summary="true", modifier="false")]
    pub device: Option<Reference>,
    /// Who should be responsible for test result
    #[fhir(name="performer", min="0", max="1", summary="true", modifier="false")]
    pub performer: Option<Reference>,
    /// Sequence that was observed
    #[fhir(name="literal", min="0", max="1", summary="true", modifier="false")]
    pub literal: Option<StringDt>,
    /// Embedded file or a link (URL) which contains content to represent the sequence
    #[fhir(name="formatted", min="0", max="*", summary="true", modifier="false")]
    pub formatted: Option<Vec<Attachment>>,
    /// A sequence defined relative to another sequence
    #[fhir(name="relative", min="0", max="*", summary="true", modifier="false")]
    pub relative: Option<Vec<MolecularSequenceRelativeBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MolecularSequenceRelativeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Ways of identifying nucleotides or amino acids within a sequence
    #[fhir(name="coordinateSystem", min="1", max="1", summary="true", modifier="false")]
    pub coordinate_system: Option<CodeableConcept>,
    /// Indicates the order in which the sequence should be considered when putting multiple 'relative' elements together
    #[fhir(name="ordinalPosition", min="0", max="1", summary="false", modifier="false")]
    pub ordinal_position: Option<IntegerDt>,
    /// Indicates the nucleotide range in the composed sequence when multiple 'relative' elements are used together
    #[fhir(name="sequenceRange", min="0", max="1", summary="false", modifier="false")]
    pub sequence_range: Option<Range>,
    /// A sequence used as starting sequence
    #[fhir(name="startingSequence", min="0", max="1", summary="true", modifier="false")]
    pub starting_sequence: Option<MolecularSequenceRelativeStartingSequenceBackboneElement>,
    /// Changes in sequence from the starting sequence
    #[fhir(name="edit", min="0", max="*", summary="true", modifier="false")]
    pub edit: Option<Vec<MolecularSequenceRelativeEditBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MolecularSequenceRelativeEditBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Start position of the edit on the starting sequence
    #[fhir(name="start", min="0", max="1", summary="true", modifier="false")]
    pub start: Option<IntegerDt>,
    /// End position of the edit on the starting sequence
    #[fhir(name="end", min="0", max="1", summary="true", modifier="false")]
    pub end: Option<IntegerDt>,
    /// Allele that was observed
    #[fhir(name="replacementSequence", min="0", max="1", summary="true", modifier="false")]
    pub replacement_sequence: Option<StringDt>,
    /// Allele in the starting sequence
    #[fhir(name="replacedSequence", min="0", max="1", summary="true", modifier="false")]
    pub replaced_sequence: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MolecularSequenceRelativeStartingSequenceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The genome assembly used for starting sequence, e.g. GRCh38
    #[fhir(name="genomeAssembly", min="0", max="1", summary="true", modifier="false")]
    pub genome_assembly: Option<CodeableConcept>,
    /// Chromosome Identifier
    #[fhir(name="chromosome", min="0", max="1", summary="true", modifier="false")]
    pub chromosome: Option<CodeableConcept>,
    /// The reference sequence that represents the starting sequence
    #[fhir(name="sequence", min="0", max="1", summary="true", modifier="false")]
    pub sequence: Option<Reference>,
    /// Start position of the window on the starting sequence
    #[fhir(name="windowStart", min="0", max="1", summary="true", modifier="false")]
    pub window_start: Option<IntegerDt>,
    /// End position of the window on the starting sequence
    #[fhir(name="windowEnd", min="0", max="1", summary="true", modifier="false")]
    pub window_end: Option<IntegerDt>,
    /// sense | antisense
    #[fhir(name="orientation", min="0", max="1", summary="true", modifier="false")]
    pub orientation: Option<CodeDt>,
    /// watson | crick
    #[fhir(name="strand", min="0", max="1", summary="true", modifier="false")]
    pub strand: Option<CodeDt>,
}


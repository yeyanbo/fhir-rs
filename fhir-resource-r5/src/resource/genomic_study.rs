use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct GenomicStudy {
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
    /// Identifiers for this genomic study
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// registered | available | cancelled | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// The type of the study (e.g., Familial variant segregation, Functional variation detection, or Gene expression profiling)
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The primary subject of the genomic study
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// The healthcare event with which this genomics study is associated
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false)]
    pub encounter: Option<Reference>,
    /// When the genomic study was started
    #[fhir(name="startDate", min="0", max="1", summary=false, modifier=false)]
    pub start_date: Option<DateTimeDt>,
    /// Event resources that the genomic study is based on
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false)]
    pub based_on: Option<Vec<Reference>>,
    /// Healthcare professional who requested or referred the genomic study
    #[fhir(name="referrer", min="0", max="1", summary=false, modifier=false)]
    pub referrer: Option<Reference>,
    /// Healthcare professionals who interpreted the genomic study
    #[fhir(name="interpreter", min="0", max="*", summary=false, modifier=false)]
    pub interpreter: Option<Vec<Reference>>,
    /// Why the genomic study was performed
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// The defined protocol that describes the study
    #[fhir(name="instantiatesCanonical", min="0", max="1", summary=false, modifier=false)]
    pub instantiates_canonical: Option<CanonicalDt>,
    /// The URL pointing to an externally maintained protocol that describes the study
    #[fhir(name="instantiatesUri", min="0", max="1", summary=false, modifier=false)]
    pub instantiates_uri: Option<UriDt>,
    /// Comments related to the genomic study
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Description of the genomic study
    #[fhir(name="description", min="0", max="1", summary=false, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Genomic Analysis Event
    #[fhir(name="analysis", min="0", max="*", summary=false, modifier=false)]
    pub analysis: Option<Vec<GenomicStudyAnalysisBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GenomicStudyAnalysisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifiers for the analysis event
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Type of the methods used in the analysis (e.g., FISH, Karyotyping, MSI)
    #[fhir(name="methodType", min="0", max="*", summary=true, modifier=false)]
    pub method_type: Option<Vec<CodeableConcept>>,
    /// Type of the genomic changes studied in the analysis (e.g., DNA, RNA, or AA change)
    #[fhir(name="changeType", min="0", max="*", summary=false, modifier=false)]
    pub change_type: Option<Vec<CodeableConcept>>,
    /// Genome build that is used in this analysis
    #[fhir(name="genomeBuild", min="0", max="1", summary=false, modifier=false)]
    pub genome_build: Option<CodeableConcept>,
    /// The defined protocol that describes the analysis
    #[fhir(name="instantiatesCanonical", min="0", max="1", summary=false, modifier=false)]
    pub instantiates_canonical: Option<CanonicalDt>,
    /// The URL pointing to an externally maintained protocol that describes the analysis
    #[fhir(name="instantiatesUri", min="0", max="1", summary=false, modifier=false)]
    pub instantiates_uri: Option<UriDt>,
    /// Name of the analysis event (human friendly)
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
    /// What the genomic analysis is about, when it is not about the subject of record
    #[fhir(name="focus", min="0", max="*", summary=true, modifier=false)]
    pub focus: Option<Vec<Reference>>,
    /// The specimen used in the analysis event
    #[fhir(name="specimen", min="0", max="*", summary=true, modifier=false)]
    pub specimen: Option<Vec<Reference>>,
    /// The date of the analysis event
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false)]
    pub date: Option<DateTimeDt>,
    /// Any notes capture with the analysis event
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// The protocol that was performed for the analysis event
    #[fhir(name="protocolPerformed", min="0", max="1", summary=false, modifier=false)]
    pub protocol_performed: Option<Reference>,
    /// The genomic regions to be studied in the analysis (BED file)
    #[fhir(name="regionsStudied", min="0", max="*", summary=false, modifier=false)]
    pub regions_studied: Option<Vec<Reference>>,
    /// Genomic regions actually called in the analysis event (BED file)
    #[fhir(name="regionsCalled", min="0", max="*", summary=false, modifier=false)]
    pub regions_called: Option<Vec<Reference>>,
    /// Inputs for the analysis event
    #[fhir(name="input", min="0", max="*", summary=false, modifier=false)]
    pub input: Option<Vec<GenomicStudyAnalysisInputBackboneElement>>,
    /// Outputs for the analysis event
    #[fhir(name="output", min="0", max="*", summary=false, modifier=false)]
    pub output: Option<Vec<GenomicStudyAnalysisOutputBackboneElement>>,
    /// Performer for the analysis event
    #[fhir(name="performer", min="0", max="*", summary=false, modifier=false)]
    pub performer: Option<Vec<GenomicStudyAnalysisPerformerBackboneElement>>,
    /// Devices used for the analysis (e.g., instruments, software), with settings and parameters
    #[fhir(name="device", min="0", max="*", summary=false, modifier=false)]
    pub device: Option<Vec<GenomicStudyAnalysisDeviceBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GenomicStudyAnalysisInputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// File containing input data
    #[fhir(name="file", min="0", max="1", summary=true, modifier=false)]
    pub file: Option<Reference>,
    /// Type of input data (e.g., BAM, CRAM, or FASTA)
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The analysis event or other GenomicStudy that generated this input file
    #[fhir(name="generatedBy", min="0", max="1", summary=false, modifier=false)]
    pub generated_by: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GenomicStudyAnalysisPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The organization, healthcare professional, or others who participated in performing this analysis
    #[fhir(name="actor", min="0", max="1", summary=false, modifier=false)]
    pub actor: Option<Reference>,
    /// Role of the actor for this analysis
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false)]
    pub role: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GenomicStudyAnalysisOutputBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// File containing output data
    #[fhir(name="file", min="0", max="1", summary=true, modifier=false)]
    pub file: Option<Reference>,
    /// Type of output data (e.g., VCF, MAF, or BAM)
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct GenomicStudyAnalysisDeviceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Device used for the analysis
    #[fhir(name="device", min="0", max="1", summary=false, modifier=false)]
    pub device: Option<Reference>,
    /// Specific function for the device used for the analysis
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false)]
    pub function: Option<CodeableConcept>,
}


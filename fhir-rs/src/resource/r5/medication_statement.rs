use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MedicationStatement {
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
    /// External identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=false, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// recorded | entered-in-error | draft
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Type of medication statement
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// What medication was taken
    #[fhir(name="medication", min="1", max="1", summary=true, modifier=false, choice="")]
    pub medication: Option<CodeableReference>,
    /// Who is/was taking  the medication
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Encounter associated with MedicationStatement
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// The date/time or interval when the medication is/was/will be taken
    #[fhir(name="effective", min="0", max="1", summary=true, modifier=false, choice="")]
    pub effective: Option<Timing>,
    /// When the usage was asserted?
    #[fhir(name="dateAsserted", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date_asserted: Option<DateTimeDt>,
    /// Person or organization that provided the information about the taking of this medication
    #[fhir(name="informationSource", min="0", max="*", summary=false, modifier=false, choice="")]
    pub information_source: Option<Vec<Reference>>,
    /// Link to information used to derive the MedicationStatement
    #[fhir(name="derivedFrom", min="0", max="*", summary=false, modifier=false, choice="")]
    pub derived_from: Option<Vec<Reference>>,
    /// Reason for why the medication is being/was taken
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reason: Option<Vec<CodeableReference>>,
    /// Further information about the usage
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Link to information relevant to the usage of a medication
    #[fhir(name="relatedClinicalInformation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_clinical_information: Option<Vec<Reference>>,
    /// Full representation of the dosage instructions
    #[fhir(name="renderedDosageInstruction", min="0", max="1", summary=false, modifier=false, choice="")]
    pub rendered_dosage_instruction: Option<MarkdownDt>,
    /// Details of how medication is/was taken or should be taken
    #[fhir(name="dosage", min="0", max="*", summary=false, modifier=false, choice="")]
    pub dosage: Option<Vec<Dosage>>,
    /// Indicates whether the medication is or is not being consumed or administered
    #[fhir(name="adherence", min="0", max="1", summary=true, modifier=false, choice="")]
    pub adherence: Option<MedicationStatementAdherenceBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicationStatementAdherenceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of adherence
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Details of the reason for the current use of the medication
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reason: Option<CodeableConcept>,
}


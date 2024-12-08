use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MedicationKnowledge {
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
    /// Business identifier for this medication
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Code that identifies this medication
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// active | entered-in-error | inactive
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Creator or owner of the knowledge or information about the medication
    #[fhir(name="author", min="0", max="1", summary=false, modifier=false, choice="")]
    pub author: Option<Reference>,
    /// Codes that identify the different jurisdictions for which the information of this resource was created
    #[fhir(name="intendedJurisdiction", min="0", max="*", summary=false, modifier=false, choice="")]
    pub intended_jurisdiction: Option<Vec<CodeableConcept>>,
    /// A name associated with the medication being described
    #[fhir(name="name", min="0", max="*", summary=true, modifier=false, choice="")]
    pub name: Option<Vec<StringDt>>,
    /// Associated or related medication information
    #[fhir(name="relatedMedicationKnowledge", min="0", max="*", summary=false, modifier=false, choice="")]
    pub related_medication_knowledge: Option<Vec<MedicationKnowledgeRelatedMedicationKnowledgeBackboneElement>>,
    /// The set of medication resources that are associated with this medication
    #[fhir(name="associatedMedication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub associated_medication: Option<Vec<Reference>>,
    /// Category of the medication or product
    #[fhir(name="productType", min="0", max="*", summary=false, modifier=false, choice="")]
    pub product_type: Option<Vec<CodeableConcept>>,
    /// Associated documentation about the medication
    #[fhir(name="monograph", min="0", max="*", summary=false, modifier=false, choice="")]
    pub monograph: Option<Vec<MedicationKnowledgeMonographBackboneElement>>,
    /// The instructions for preparing the medication
    #[fhir(name="preparationInstruction", min="0", max="1", summary=false, modifier=false, choice="")]
    pub preparation_instruction: Option<MarkdownDt>,
    /// The pricing of the medication
    #[fhir(name="cost", min="0", max="*", summary=false, modifier=false, choice="")]
    pub cost: Option<Vec<MedicationKnowledgeCostBackboneElement>>,
    /// Program under which a medication is reviewed
    #[fhir(name="monitoringProgram", min="0", max="*", summary=true, modifier=false, choice="")]
    pub monitoring_program: Option<Vec<MedicationKnowledgeMonitoringProgramBackboneElement>>,
    /// Guidelines or protocols for administration of the medication for an indication
    #[fhir(name="indicationGuideline", min="0", max="*", summary=false, modifier=false, choice="")]
    pub indication_guideline: Option<Vec<MedicationKnowledgeIndicationGuidelineBackboneElement>>,
    /// Categorization of the medication within a formulary or classification system
    #[fhir(name="medicineClassification", min="0", max="*", summary=false, modifier=false, choice="")]
    pub medicine_classification: Option<Vec<MedicationKnowledgeMedicineClassificationBackboneElement>>,
    /// Details about packaged medications
    #[fhir(name="packaging", min="0", max="*", summary=false, modifier=false, choice="")]
    pub packaging: Option<Vec<MedicationKnowledgePackagingBackboneElement>>,
    /// Potential clinical issue with or between medication(s)
    #[fhir(name="clinicalUseIssue", min="0", max="*", summary=false, modifier=false, choice="")]
    pub clinical_use_issue: Option<Vec<Reference>>,
    /// How the medication should be stored
    #[fhir(name="storageGuideline", min="0", max="*", summary=false, modifier=false, choice="")]
    pub storage_guideline: Option<Vec<MedicationKnowledgeStorageGuidelineBackboneElement>>,
    /// Regulatory information about a medication
    #[fhir(name="regulatory", min="0", max="*", summary=false, modifier=false, choice="")]
    pub regulatory: Option<Vec<MedicationKnowledgeRegulatoryBackboneElement>>,
    /// Minimal definition information about the medication
    #[fhir(name="definitional", min="0", max="1", summary=false, modifier=false, choice="")]
    pub definitional: Option<MedicationKnowledgeDefinitionalBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeIndicationGuidelineBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Indication for use that applies to the specific administration guideline
    #[fhir(name="indication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub indication: Option<Vec<CodeableReference>>,
    /// Guidelines for dosage of the medication
    #[fhir(name="dosingGuideline", min="0", max="*", summary=false, modifier=false, choice="")]
    pub dosing_guideline: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelineBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelineBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Intention of the treatment
    #[fhir(name="treatmentIntent", min="0", max="1", summary=false, modifier=false, choice="")]
    pub treatment_intent: Option<CodeableConcept>,
    /// Dosage for the medication for the specific guidelines
    #[fhir(name="dosage", min="0", max="*", summary=false, modifier=false, choice="")]
    pub dosage: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelineDosageBackboneElement>>,
    /// Type of treatment the guideline applies to
    #[fhir(name="administrationTreatment", min="0", max="1", summary=false, modifier=false, choice="")]
    pub administration_treatment: Option<CodeableConcept>,
    /// Characteristics of the patient that are relevant to the administration guidelines
    #[fhir(name="patientCharacteristic", min="0", max="*", summary=false, modifier=false, choice="")]
    pub patient_characteristic: Option<Vec<MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelinePatientCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Categorization of specific characteristic that is relevant to the administration guideline
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// The specific characteristic
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Range>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeIndicationGuidelineDosingGuidelineDosageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Category of dosage for a medication
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Dosage for the medication for the specific guidelines
    #[fhir(name="dosage", min="1", max="*", summary=false, modifier=false, choice="")]
    pub dosage: Option<Vec<Dosage>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeMedicineClassificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of category for the medication (for example, therapeutic classification, therapeutic sub-classification)
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// The source of the classification
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false, choice="")]
    pub source: Option<UriDt>,
    /// Specific category assigned to the medication
    #[fhir(name="classification", min="0", max="*", summary=false, modifier=false, choice="")]
    pub classification: Option<Vec<CodeableConcept>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeRegulatoryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specifies the authority of the regulation
    #[fhir(name="regulatoryAuthority", min="1", max="1", summary=false, modifier=false, choice="")]
    pub regulatory_authority: Option<Reference>,
    /// Specifies if changes are allowed when dispensing a medication from a regulatory perspective
    #[fhir(name="substitution", min="0", max="*", summary=false, modifier=false, choice="")]
    pub substitution: Option<Vec<MedicationKnowledgeRegulatorySubstitutionBackboneElement>>,
    /// Specifies the schedule of a medication in jurisdiction
    #[fhir(name="schedule", min="0", max="*", summary=false, modifier=false, choice="")]
    pub schedule: Option<Vec<CodeableConcept>>,
    /// The maximum number of units of the medication that can be dispensed in a period
    #[fhir(name="maxDispense", min="0", max="1", summary=false, modifier=false, choice="")]
    pub max_dispense: Option<MedicationKnowledgeRegulatoryMaxDispenseBackboneElement>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeRegulatoryMaxDispenseBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The maximum number of units of the medication that can be dispensed
    #[fhir(name="quantity", min="1", max="1", summary=false, modifier=false, choice="")]
    pub quantity: Option<Quantity>,
    /// The period that applies to the maximum number of units
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Duration>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeRegulatorySubstitutionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Specifies the type of substitution allowed
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Specifies if regulation allows for changes in the medication when dispensing
    #[fhir(name="allowed", min="1", max="1", summary=false, modifier=false, choice="")]
    pub allowed: Option<BooleanDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeStorageGuidelineBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to additional information
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reference: Option<UriDt>,
    /// Additional storage notes
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Duration remains stable
    #[fhir(name="stabilityDuration", min="0", max="1", summary=false, modifier=false, choice="")]
    pub stability_duration: Option<Duration>,
    /// Setting or value of environment for adequate storage
    #[fhir(name="environmentalSetting", min="0", max="*", summary=false, modifier=false, choice="")]
    pub environmental_setting: Option<Vec<MedicationKnowledgeStorageGuidelineEnvironmentalSettingBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeStorageGuidelineEnvironmentalSettingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Categorization of the setting
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Value of the setting
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeDefinitionalBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Definitional resources that provide more information about this medication
    #[fhir(name="definition", min="0", max="*", summary=false, modifier=false, choice="")]
    pub definition: Option<Vec<Reference>>,
    /// powder | tablets | capsule +
    #[fhir(name="doseForm", min="0", max="1", summary=false, modifier=false, choice="")]
    pub dose_form: Option<CodeableConcept>,
    /// The intended or approved route of administration
    #[fhir(name="intendedRoute", min="0", max="*", summary=false, modifier=false, choice="")]
    pub intended_route: Option<Vec<CodeableConcept>>,
    /// Active or inactive ingredient
    #[fhir(name="ingredient", min="0", max="*", summary=true, modifier=false, choice="")]
    pub ingredient: Option<Vec<MedicationKnowledgeDefinitionalIngredientBackboneElement>>,
    /// Specifies descriptive properties of the medicine
    #[fhir(name="drugCharacteristic", min="0", max="*", summary=false, modifier=false, choice="")]
    pub drug_characteristic: Option<Vec<MedicationKnowledgeDefinitionalDrugCharacteristicBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeDefinitionalDrugCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code specifying the type of characteristic of medication
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Description of the characteristic
    #[fhir(name="value", min="0", max="1", summary=false, modifier=false, choice="")]
    pub value: Option<Attachment>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeDefinitionalIngredientBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Substances contained in the medication
    #[fhir(name="item", min="1", max="1", summary=true, modifier=false, choice="")]
    pub item: Option<CodeableReference>,
    /// A code that defines the type of ingredient, active, base, etc
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Quantity of ingredient present
    #[fhir(name="strength", min="0", max="1", summary=false, modifier=false, choice="")]
    pub strength: Option<Quantity>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgePackagingBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Cost of the packaged medication
    #[fhir(name="cost", min="0", max="*", summary=false, modifier=false, choice="")]
    pub cost: Option<Vec<MedicationKnowledgeCostBackboneElement>>,
    /// The packaged medication that is being priced
    #[fhir(name="packagedProduct", min="0", max="1", summary=false, modifier=false, choice="")]
    pub packaged_product: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeRelatedMedicationKnowledgeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Category of medicationKnowledge
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Associated documentation about the associated medication knowledge
    #[fhir(name="reference", min="1", max="*", summary=false, modifier=false, choice="")]
    pub reference: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeMonographBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The category of medication document
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Associated documentation about the medication
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false, choice="")]
    pub source: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeCostBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The date range for which the cost is effective
    #[fhir(name="effectiveDate", min="0", max="*", summary=false, modifier=false, choice="")]
    pub effective_date: Option<Vec<Period>>,
    /// The category of the cost information
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// The source or owner for the price information
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false, choice="")]
    pub source: Option<StringDt>,
    /// The price or category of the cost of the medication
    #[fhir(name="cost", min="1", max="1", summary=false, modifier=false, choice="")]
    pub cost: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct MedicationKnowledgeMonitoringProgramBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of program under which the medication is monitored
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Name of the reviewing program
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<StringDt>,
}


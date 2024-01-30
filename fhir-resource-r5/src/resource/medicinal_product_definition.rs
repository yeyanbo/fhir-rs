use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct MedicinalProductDefinition {
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
    /// Business identifier for this product. Could be an MPID
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Regulatory type, e.g. Investigational or Authorized
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// If this medicine applies to human or veterinary uses
    #[fhir(name="domain", min="0", max="1", summary=true, modifier=false)]
    pub domain: Option<CodeableConcept>,
    /// A business identifier relating to a specific version of the product
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// The status within the lifecycle of this product record
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeableConcept>,
    /// The date at which the given status became applicable
    #[fhir(name="statusDate", min="0", max="1", summary=true, modifier=false)]
    pub status_date: Option<DateTimeDt>,
    /// General description of this product
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// The dose form for a single part product, or combined form of a multiple part product
    #[fhir(name="combinedPharmaceuticalDoseForm", min="0", max="1", summary=true, modifier=false)]
    pub combined_pharmaceutical_dose_form: Option<CodeableConcept>,
    /// The path by which the product is taken into or makes contact with the body
    #[fhir(name="route", min="0", max="*", summary=true, modifier=false)]
    pub route: Option<Vec<CodeableConcept>>,
    /// Description of indication(s) for this product, used when structured indications are not required
    #[fhir(name="indication", min="0", max="1", summary=true, modifier=false)]
    pub indication: Option<MarkdownDt>,
    /// The legal status of supply of the medicinal product as classified by the regulator
    #[fhir(name="legalStatusOfSupply", min="0", max="1", summary=true, modifier=false)]
    pub legal_status_of_supply: Option<CodeableConcept>,
    /// Whether the Medicinal Product is subject to additional monitoring for regulatory reasons
    #[fhir(name="additionalMonitoringIndicator", min="0", max="1", summary=true, modifier=false)]
    pub additional_monitoring_indicator: Option<CodeableConcept>,
    /// Whether the Medicinal Product is subject to special measures for regulatory reasons
    #[fhir(name="specialMeasures", min="0", max="*", summary=true, modifier=false)]
    pub special_measures: Option<Vec<CodeableConcept>>,
    /// If authorised for use in children
    #[fhir(name="pediatricUseIndicator", min="0", max="1", summary=true, modifier=false)]
    pub pediatric_use_indicator: Option<CodeableConcept>,
    /// Allows the product to be classified by various systems
    #[fhir(name="classification", min="0", max="*", summary=true, modifier=false)]
    pub classification: Option<Vec<CodeableConcept>>,
    /// Marketing status of the medicinal product, in contrast to marketing authorization
    #[fhir(name="marketingStatus", min="0", max="*", summary=true, modifier=false)]
    pub marketing_status: Option<Vec<MarketingStatus>>,
    /// Package type for the product
    #[fhir(name="packagedMedicinalProduct", min="0", max="*", summary=true, modifier=false)]
    pub packaged_medicinal_product: Option<Vec<CodeableConcept>>,
    /// Types of medicinal manufactured items and/or devices that this product consists of, such as tablets, capsule, or syringes
    #[fhir(name="comprisedOf", min="0", max="*", summary=true, modifier=false)]
    pub comprised_of: Option<Vec<Reference>>,
    /// The ingredients of this medicinal product - when not detailed in other resources
    #[fhir(name="ingredient", min="0", max="*", summary=true, modifier=false)]
    pub ingredient: Option<Vec<CodeableConcept>>,
    /// Any component of the drug product which is not the chemical entity defined as the drug substance, or an excipient in the drug product
    #[fhir(name="impurity", min="0", max="*", summary=true, modifier=false)]
    pub impurity: Option<Vec<CodeableReference>>,
    /// Additional documentation about the medicinal product
    #[fhir(name="attachedDocument", min="0", max="*", summary=true, modifier=false)]
    pub attached_document: Option<Vec<Reference>>,
    /// A master file for the medicinal product (e.g. Pharmacovigilance System Master File)
    #[fhir(name="masterFile", min="0", max="*", summary=true, modifier=false)]
    pub master_file: Option<Vec<Reference>>,
    /// A product specific contact, person (in a role), or an organization
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false)]
    pub contact: Option<Vec<MedicinalProductDefinitionContactBackboneElement>>,
    /// Clinical trials or studies that this product is involved in
    #[fhir(name="clinicalTrial", min="0", max="*", summary=true, modifier=false)]
    pub clinical_trial: Option<Vec<Reference>>,
    /// A code that this product is known by, within some formal terminology
    #[fhir(name="code", min="0", max="*", summary=true, modifier=false)]
    pub code: Option<Vec<Coding>>,
    /// The product's name, including full name and possibly coded parts
    #[fhir(name="name", min="1", max="*", summary=true, modifier=false)]
    pub name: Option<Vec<MedicinalProductDefinitionNameBackboneElement>>,
    /// Reference to another product, e.g. for linking authorised to investigational product
    #[fhir(name="crossReference", min="0", max="*", summary=true, modifier=false)]
    pub cross_reference: Option<Vec<MedicinalProductDefinitionCrossReferenceBackboneElement>>,
    /// A manufacturing or administrative process for the medicinal product
    #[fhir(name="operation", min="0", max="*", summary=true, modifier=false)]
    pub operation: Option<Vec<MedicinalProductDefinitionOperationBackboneElement>>,
    /// Key product features such as "sugar free", "modified release"
    #[fhir(name="characteristic", min="0", max="*", summary=true, modifier=false)]
    pub characteristic: Option<Vec<MedicinalProductDefinitionCharacteristicBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionCrossReferenceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to another product, e.g. for linking authorised to investigational product
    #[fhir(name="product", min="1", max="1", summary=true, modifier=false)]
    pub product: Option<CodeableReference>,
    /// The type of relationship, for instance branded to generic or virtual to actual product
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionOperationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of manufacturing operation e.g. manufacturing itself, re-packaging
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableReference>,
    /// Date range of applicability
    #[fhir(name="effectiveDate", min="0", max="1", summary=true, modifier=false)]
    pub effective_date: Option<Period>,
    /// The organization responsible for the particular process, e.g. the manufacturer or importer
    #[fhir(name="organization", min="0", max="*", summary=true, modifier=false)]
    pub organization: Option<Vec<Reference>>,
    /// Specifies whether this process is considered proprietary or confidential
    #[fhir(name="confidentialityIndicator", min="0", max="1", summary=true, modifier=false)]
    pub confidentiality_indicator: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionNameBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The full product name
    #[fhir(name="productName", min="1", max="1", summary=true, modifier=false)]
    pub product_name: Option<StringDt>,
    /// Type of product name, such as rINN, BAN, Proprietary, Non-Proprietary
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Coding words or phrases of the name
    #[fhir(name="part", min="0", max="*", summary=true, modifier=false)]
    pub part: Option<Vec<MedicinalProductDefinitionNamePartBackboneElement>>,
    /// Country and jurisdiction where the name applies
    #[fhir(name="usage", min="0", max="*", summary=true, modifier=false)]
    pub usage: Option<Vec<MedicinalProductDefinitionNameUsageBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionNameUsageBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Country code for where this name applies
    #[fhir(name="country", min="1", max="1", summary=true, modifier=false)]
    pub country: Option<CodeableConcept>,
    /// Jurisdiction code for where this name applies
    #[fhir(name="jurisdiction", min="0", max="1", summary=true, modifier=false)]
    pub jurisdiction: Option<CodeableConcept>,
    /// Language code for this name
    #[fhir(name="language", min="1", max="1", summary=true, modifier=false)]
    pub language: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionNamePartBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A fragment of a product name
    #[fhir(name="part", min="1", max="1", summary=true, modifier=false)]
    pub part: Option<StringDt>,
    /// Identifying type for this part of the name (e.g. strength part)
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionCharacteristicBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A code expressing the type of characteristic
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// A value for the characteristic
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<Attachment>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct MedicinalProductDefinitionContactBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Allows the contact to be classified, for example QPPV, Pharmacovigilance Enquiry Information
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// A product specific contact, person (in a role), or an organization
    #[fhir(name="contact", min="1", max="1", summary=true, modifier=false)]
    pub contact: Option<Reference>,
}


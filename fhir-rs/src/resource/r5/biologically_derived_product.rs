use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct BiologicallyDerivedProduct {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// organ | tissue | fluid | cells | biologicalAgent
    #[fhir(name="productCategory", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_category: Option<Coding>,
    /// A code that identifies the kind of this biologically derived product
    #[fhir(name="productCode", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_code: Option<CodeableConcept>,
    /// The parent biologically-derived product
    #[fhir(name="parent", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub parent: Option<Vec<Reference>>,
    /// Request to obtain and/or infuse this product
    #[fhir(name="request", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub request: Option<Vec<Reference>>,
    /// Instance identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// An identifier that supports traceability to the event during which material in this product from one or more biological entities was obtained or pooled
    #[fhir(name="biologicalSourceEvent", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub biological_source_event: Option<Identifier>,
    /// Processing facilities responsible for the labeling and distribution of this biologically derived product
    #[fhir(name="processingFacility", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub processing_facility: Option<Vec<Reference>>,
    /// A unique identifier for an aliquot of a product
    #[fhir(name="division", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub division: Option<StringDt>,
    /// available | unavailable
    #[fhir(name="productStatus", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub product_status: Option<Coding>,
    /// Date, and where relevant time, of expiration
    #[fhir(name="expirationDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub expiration_date: Option<DateTimeDt>,
    /// How this product was collected
    #[fhir(name="collection", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub collection: Option<BiologicallyDerivedProductCollectionBackboneElement>,
    /// Product storage temperature requirements
    #[fhir(name="storageTempRequirements", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub storage_temp_requirements: Option<Range>,
    /// A property that is specific to this BiologicallyDerviedProduct instance
    #[fhir(name="property", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub property: Option<Vec<BiologicallyDerivedProductPropertyBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BiologicallyDerivedProductCollectionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Individual performing collection
    #[fhir(name="collector", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub collector: Option<Reference>,
    /// The patient who underwent the medical procedure to collect the product or the organization that facilitated the collection
    #[fhir(name="source", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub source: Option<Reference>,
    /// Time of product collection
    #[fhir(name="collected", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub collected: Option<Period>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BiologicallyDerivedProductPropertyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that specifies the property
    #[fhir(name="type", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub type_: Option<CodeableConcept>,
    /// Property values
    #[fhir(name="value", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub value: Option<Attachment>,
}


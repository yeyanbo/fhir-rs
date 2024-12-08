use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct SubstanceReferenceInformation {
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
    /// Todo
    #[fhir(name="comment", min="0", max="1", summary=true, modifier=false, choice="")]
    pub comment: Option<StringDt>,
    /// Todo
    #[fhir(name="gene", min="0", max="*", summary=true, modifier=false, choice="")]
    pub gene: Option<Vec<SubstanceReferenceInformationGeneBackboneElement>>,
    /// Todo
    #[fhir(name="geneElement", min="0", max="*", summary=true, modifier=false, choice="")]
    pub gene_element: Option<Vec<SubstanceReferenceInformationGeneElementBackboneElement>>,
    /// Todo
    #[fhir(name="target", min="0", max="*", summary=true, modifier=false, choice="")]
    pub target: Option<Vec<SubstanceReferenceInformationTargetBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct SubstanceReferenceInformationGeneBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Todo
    #[fhir(name="geneSequenceOrigin", min="0", max="1", summary=true, modifier=false, choice="")]
    pub gene_sequence_origin: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="gene", min="0", max="1", summary=true, modifier=false, choice="")]
    pub gene: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="source", min="0", max="*", summary=true, modifier=false, choice="")]
    pub source: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct SubstanceReferenceInformationTargetBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Todo
    #[fhir(name="target", min="0", max="1", summary=true, modifier=false, choice="")]
    pub target: Option<Identifier>,
    /// Todo
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="interaction", min="0", max="1", summary=true, modifier=false, choice="")]
    pub interaction: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="organism", min="0", max="1", summary=true, modifier=false, choice="")]
    pub organism: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="organismType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub organism_type: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false, choice="")]
    pub amount: Option<StringDt>,
    /// Todo
    #[fhir(name="amountType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub amount_type: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="source", min="0", max="*", summary=true, modifier=false, choice="")]
    pub source: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct SubstanceReferenceInformationGeneElementBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Todo
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Todo
    #[fhir(name="element", min="0", max="1", summary=true, modifier=false, choice="")]
    pub element: Option<Identifier>,
    /// Todo
    #[fhir(name="source", min="0", max="*", summary=true, modifier=false, choice="")]
    pub source: Option<Vec<Reference>>,
}


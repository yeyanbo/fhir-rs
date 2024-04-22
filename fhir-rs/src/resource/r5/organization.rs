use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Organization {
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
    /// Identifies this organization  across multiple systems
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether the organization's record is still in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=true)]
    pub active: Option<BooleanDt>,
    /// Kind of organization
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Name used for the organization
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false, choice="")]
    pub name: Option<StringDt>,
    /// A list of alternate names that the organization is known as, or was known as in the past
    #[fhir(name="alias", min="0", max="*", summary=false, modifier=false, choice="")]
    pub alias: Option<Vec<StringDt>>,
    /// Additional details about the Organization that could be displayed as further information to identify the Organization beyond its name
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// Official contact details for the Organization
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// The organization of which this organization forms a part
    #[fhir(name="partOf", min="0", max="1", summary=true, modifier=false, choice="")]
    pub part_of: Option<Reference>,
    /// Technical endpoints providing access to services operated for the organization
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false, choice="")]
    pub endpoint: Option<Vec<Reference>>,
    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    #[fhir(name="qualification", min="0", max="*", summary=false, modifier=false, choice="")]
    pub qualification: Option<Vec<OrganizationQualificationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct OrganizationQualificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// An identifier for this qualification for the organization
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Coded representation of the qualification
    #[fhir(name="code", min="1", max="1", summary=false, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Period during which the qualification is valid
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Organization that regulates and issues the qualification
    #[fhir(name="issuer", min="0", max="1", summary=false, modifier=false, choice="")]
    pub issuer: Option<Reference>,
}


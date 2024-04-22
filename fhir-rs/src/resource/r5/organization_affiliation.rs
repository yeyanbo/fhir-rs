use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct OrganizationAffiliation {
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
    /// Business identifiers that are specific to this role
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this organization affiliation record is in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=false, choice="")]
    pub active: Option<BooleanDt>,
    /// The period during which the participatingOrganization is affiliated with the primary organization
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Organization where the role is available
    #[fhir(name="organization", min="0", max="1", summary=true, modifier=false, choice="")]
    pub organization: Option<Reference>,
    /// Organization that provides/performs the role (e.g. providing services or is a member of)
    #[fhir(name="participatingOrganization", min="0", max="1", summary=true, modifier=false, choice="")]
    pub participating_organization: Option<Reference>,
    /// The network in which the participatingOrganization provides the role's services (if defined) at the indicated locations (if defined)
    #[fhir(name="network", min="0", max="*", summary=true, modifier=false, choice="")]
    pub network: Option<Vec<Reference>>,
    /// Definition of the role the participatingOrganization plays
    #[fhir(name="code", min="0", max="*", summary=true, modifier=false, choice="")]
    pub code: Option<Vec<CodeableConcept>>,
    /// Specific specialty of the participatingOrganization in the context of the role
    #[fhir(name="specialty", min="0", max="*", summary=true, modifier=false, choice="")]
    pub specialty: Option<Vec<CodeableConcept>>,
    /// The location(s) at which the role occurs
    #[fhir(name="location", min="0", max="*", summary=true, modifier=false, choice="")]
    pub location: Option<Vec<Reference>>,
    /// Healthcare services provided through the role
    #[fhir(name="healthcareService", min="0", max="*", summary=false, modifier=false, choice="")]
    pub healthcare_service: Option<Vec<Reference>>,
    /// Official contact details at the participatingOrganization relevant to this Affiliation
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// Technical endpoints providing access to services operated for this role
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false, choice="")]
    pub endpoint: Option<Vec<Reference>>,
}


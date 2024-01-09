use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct PractitionerRole {
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
    /// Identifiers for a role/location
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this practitioner role record is in active use
    #[fhir(name="active", min="0", max="1", summary="true", modifier="false")]
    pub active: Option<BooleanDt>,
    /// The period during which the practitioner is authorized to perform in these role(s)
    #[fhir(name="period", min="0", max="1", summary="true", modifier="false")]
    pub period: Option<Period>,
    /// Practitioner that provides services for the organization
    #[fhir(name="practitioner", min="0", max="1", summary="true", modifier="false")]
    pub practitioner: Option<Reference>,
    /// Organization where the roles are available
    #[fhir(name="organization", min="0", max="1", summary="true", modifier="false")]
    pub organization: Option<Reference>,
    /// Roles which this practitioner may perform
    #[fhir(name="code", min="0", max="*", summary="true", modifier="false")]
    pub code: Option<Vec<CodeableConcept>>,
    /// Specific specialty of the practitioner
    #[fhir(name="specialty", min="0", max="*", summary="true", modifier="false")]
    pub specialty: Option<Vec<CodeableConcept>>,
    /// Location(s) where the practitioner provides care
    #[fhir(name="location", min="0", max="*", summary="true", modifier="false")]
    pub location: Option<Vec<Reference>>,
    /// Healthcare services provided for this role's Organization/Location(s)
    #[fhir(name="healthcareService", min="0", max="*", summary="false", modifier="false")]
    pub healthcare_service: Option<Vec<Reference>>,
    /// Official contact details relating to this PractitionerRole
    #[fhir(name="contact", min="0", max="*", summary="false", modifier="false")]
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// Collection of characteristics (attributes)
    #[fhir(name="characteristic", min="0", max="*", summary="false", modifier="false")]
    pub characteristic: Option<Vec<CodeableConcept>>,
    /// A language the practitioner (in this role) can use in patient communication
    #[fhir(name="communication", min="0", max="*", summary="false", modifier="false")]
    pub communication: Option<Vec<CodeableConcept>>,
    /// Times the Practitioner is available at this location and/or healthcare service (including exceptions)
    #[fhir(name="availability", min="0", max="*", summary="false", modifier="false")]
    pub availability: Option<Vec<Availability>>,
    /// Endpoints for interacting with the practitioner in this role
    #[fhir(name="endpoint", min="0", max="*", summary="false", modifier="false")]
    pub endpoint: Option<Vec<Reference>>,
}


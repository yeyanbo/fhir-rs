use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct HealthcareService {
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
    /// External identifiers for this item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this HealthcareService record is in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=true)]
    pub active: Option<BooleanDt>,
    /// Organization that provides this service
    #[fhir(name="providedBy", min="0", max="1", summary=true, modifier=false)]
    pub provided_by: Option<Reference>,
    /// The service within which this service is offered
    #[fhir(name="offeredIn", min="0", max="*", summary=false, modifier=false)]
    pub offered_in: Option<Vec<Reference>>,
    /// Broad category of service being performed or delivered
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of service that may be delivered or performed
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Specialties handled by the HealthcareService
    #[fhir(name="specialty", min="0", max="*", summary=true, modifier=false)]
    pub specialty: Option<Vec<CodeableConcept>>,
    /// Location(s) where service may be provided
    #[fhir(name="location", min="0", max="*", summary=true, modifier=false)]
    pub location: Option<Vec<Reference>>,
    /// Description of service as presented to a consumer while searching
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Additional description and/or any specific issues not covered elsewhere
    #[fhir(name="comment", min="0", max="1", summary=true, modifier=false)]
    pub comment: Option<MarkdownDt>,
    /// Extra details about the service that can't be placed in the other fields
    #[fhir(name="extraDetails", min="0", max="1", summary=false, modifier=false)]
    pub extra_details: Option<MarkdownDt>,
    /// Facilitates quick identification of the service
    #[fhir(name="photo", min="0", max="1", summary=true, modifier=false)]
    pub photo: Option<Attachment>,
    /// Official contact details for the HealthcareService
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false)]
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// Location(s) service is intended for/available to
    #[fhir(name="coverageArea", min="0", max="*", summary=false, modifier=false)]
    pub coverage_area: Option<Vec<Reference>>,
    /// Conditions under which service is available/offered
    #[fhir(name="serviceProvisionCode", min="0", max="*", summary=false, modifier=false)]
    pub service_provision_code: Option<Vec<CodeableConcept>>,
    /// Specific eligibility requirements required to use the service
    #[fhir(name="eligibility", min="0", max="*", summary=false, modifier=false)]
    pub eligibility: Option<Vec<HealthcareServiceEligibilityBackboneElement>>,
    /// Programs that this service is applicable to
    #[fhir(name="program", min="0", max="*", summary=false, modifier=false)]
    pub program: Option<Vec<CodeableConcept>>,
    /// Collection of characteristics (attributes)
    #[fhir(name="characteristic", min="0", max="*", summary=false, modifier=false)]
    pub characteristic: Option<Vec<CodeableConcept>>,
    /// The language that this service is offered in
    #[fhir(name="communication", min="0", max="*", summary=false, modifier=false)]
    pub communication: Option<Vec<CodeableConcept>>,
    /// Ways that the service accepts referrals
    #[fhir(name="referralMethod", min="0", max="*", summary=false, modifier=false)]
    pub referral_method: Option<Vec<CodeableConcept>>,
    /// If an appointment is required for access to this service
    #[fhir(name="appointmentRequired", min="0", max="1", summary=false, modifier=false)]
    pub appointment_required: Option<BooleanDt>,
    /// Times the healthcare service is available (including exceptions)
    #[fhir(name="availability", min="0", max="*", summary=false, modifier=false)]
    pub availability: Option<Vec<Availability>>,
    /// Technical endpoints providing access to electronic services operated for the healthcare service
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false)]
    pub endpoint: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct HealthcareServiceEligibilityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Coded value for the eligibility
    #[fhir(name="code", min="0", max="1", summary=false, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// Describes the eligibility conditions for the service
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false)]
    pub comment: Option<MarkdownDt>,
}


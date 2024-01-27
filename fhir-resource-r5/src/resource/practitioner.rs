use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct Practitioner {
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
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// An identifier for the person as this agent
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this practitioner's record is in active use
    #[fhir(name="active", min="0", max="1", summary="true", modifier="true")]
    pub active: Option<BooleanDt>,
    /// The name(s) associated with the practitioner
    #[fhir(name="name", min="0", max="*", summary="true", modifier="false")]
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the practitioner (that apply to all roles)
    #[fhir(name="telecom", min="0", max="*", summary="true", modifier="false")]
    pub telecom: Option<Vec<ContactPoint>>,
    /// male | female | other | unknown
    #[fhir(name="gender", min="0", max="1", summary="true", modifier="false")]
    pub gender: Option<CodeDt>,
    /// The date  on which the practitioner was born
    #[fhir(name="birthDate", min="0", max="1", summary="true", modifier="false")]
    pub birth_date: Option<DateDt>,
    /// Indicates if the practitioner is deceased or not
    #[fhir(name="deceased", min="0", max="1", summary="true", modifier="false")]
    pub deceased: Option<DateTimeDt>,
    /// Address(es) of the practitioner that are not role specific (typically home address)
    #[fhir(name="address", min="0", max="*", summary="true", modifier="false")]
    pub address: Option<Vec<Address>>,
    /// Image of the person
    #[fhir(name="photo", min="0", max="*", summary="false", modifier="false")]
    pub photo: Option<Vec<Attachment>>,
    /// Qualifications, certifications, accreditations, licenses, training, etc. pertaining to the provision of care
    #[fhir(name="qualification", min="0", max="*", summary="false", modifier="false")]
    pub qualification: Option<Vec<PractitionerQualificationBackboneElement>>,
    /// A language which may be used to communicate with the practitioner
    #[fhir(name="communication", min="0", max="*", summary="false", modifier="false")]
    pub communication: Option<Vec<PractitionerCommunicationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PractitionerQualificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// An identifier for this qualification for the practitioner
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Coded representation of the qualification
    #[fhir(name="code", min="1", max="1", summary="false", modifier="false")]
    pub code: Option<CodeableConcept>,
    /// Period during which the qualification is valid
    #[fhir(name="period", min="0", max="1", summary="false", modifier="false")]
    pub period: Option<Period>,
    /// Organization that regulates and issues the qualification
    #[fhir(name="issuer", min="0", max="1", summary="false", modifier="false")]
    pub issuer: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PractitionerCommunicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The language code used to communicate with the practitioner
    #[fhir(name="language", min="1", max="1", summary="false", modifier="false")]
    pub language: Option<CodeableConcept>,
    /// Language preference indicator
    #[fhir(name="preferred", min="0", max="1", summary="false", modifier="false")]
    pub preferred: Option<BooleanDt>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Patient {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice="")]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice="")]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true, choice="")]
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
    /// An identifier for this patient
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this patient's record is in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=true, choice="")]
    pub active: Option<BooleanDt>,
    /// A name associated with the patient
    #[fhir(name="name", min="0", max="*", summary=true, modifier=false, choice="")]
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the individual
    #[fhir(name="telecom", min="0", max="*", summary=true, modifier=false, choice="")]
    pub telecom: Option<Vec<ContactPoint>>,
    /// male | female | other | unknown
    #[fhir(name="gender", min="0", max="1", summary=true, modifier=false, choice="")]
    pub gender: Option<CodeDt>,
    /// The date of birth for the individual
    #[fhir(name="birthDate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub birth_date: Option<DateDt>,
    /// Indicates if the individual is deceased or not
    #[fhir(name="deceased", min="0", max="1", summary=true, modifier=true, choice="Boolean|DateTime")]
    pub deceased: Option<AnyType>,
    /// An address for the individual
    #[fhir(name="address", min="0", max="*", summary=true, modifier=false, choice="")]
    pub address: Option<Vec<Address>>,
    /// Marital (civil) status of a patient
    #[fhir(name="maritalStatus", min="0", max="1", summary=false, modifier=false, choice="")]
    pub marital_status: Option<CodeableConcept>,
    /// Whether patient is part of a multiple birth
    #[fhir(name="multipleBirth", min="0", max="1", summary=false, modifier=false, choice="")]
    pub multiple_birth: Option<IntegerDt>,
    /// Image of the patient
    #[fhir(name="photo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub photo: Option<Vec<Attachment>>,
    /// A contact party (e.g. guardian, partner, friend) for the patient
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contact: Option<Vec<PatientContactBackboneElement>>,
    /// A language which may be used to communicate with the patient about his or her health
    #[fhir(name="communication", min="0", max="*", summary=false, modifier=false, choice="")]
    pub communication: Option<Vec<PatientCommunicationBackboneElement>>,
    /// Patient's nominated primary care provider
    #[fhir(name="generalPractitioner", min="0", max="*", summary=false, modifier=false, choice="")]
    pub general_practitioner: Option<Vec<Reference>>,
    /// Organization that is the custodian of the patient record
    #[fhir(name="managingOrganization", min="0", max="1", summary=true, modifier=false, choice="")]
    pub managing_organization: Option<Reference>,
    /// Link to a Patient or RelatedPerson resource that concerns the same actual individual
    #[fhir(name="link", min="0", max="*", summary=true, modifier=true, choice="")]
    pub link: Option<Vec<PatientLinkBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct PatientLinkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The other patient or related person resource that the link refers to
    #[fhir(name="other", min="1", max="1", summary=true, modifier=false, choice="")]
    pub other: Option<Reference>,
    /// replaced-by | replaces | refer | seealso
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct PatientContactBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The kind of relationship
    #[fhir(name="relationship", min="0", max="*", summary=false, modifier=false, choice="")]
    pub relationship: Option<Vec<CodeableConcept>>,
    /// A name associated with the contact person
    #[fhir(name="name", min="0", max="1", summary=false, modifier=false, choice="")]
    pub name: Option<HumanName>,
    /// A contact detail for the person
    #[fhir(name="telecom", min="0", max="*", summary=false, modifier=false, choice="")]
    pub telecom: Option<Vec<ContactPoint>>,
    /// Address for the contact person
    #[fhir(name="address", min="0", max="1", summary=false, modifier=false, choice="")]
    pub address: Option<Address>,
    /// male | female | other | unknown
    #[fhir(name="gender", min="0", max="1", summary=false, modifier=false, choice="")]
    pub gender: Option<CodeDt>,
    /// Organization that is associated with the contact
    #[fhir(name="organization", min="0", max="1", summary=false, modifier=false, choice="")]
    pub organization: Option<Reference>,
    /// The period during which this contact person or organization is valid to be contacted relating to this patient
    #[fhir(name="period", min="0", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct PatientCommunicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The language which can be used to communicate with the patient about his or her health
    #[fhir(name="language", min="1", max="1", summary=false, modifier=false, choice="")]
    pub language: Option<CodeableConcept>,
    /// Language preference indicator
    #[fhir(name="preferred", min="0", max="1", summary=false, modifier=false, choice="")]
    pub preferred: Option<BooleanDt>,
}
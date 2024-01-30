use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Person {
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
    /// A human identifier for this person
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// This person's record is in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=true)]
    pub active: Option<BooleanDt>,
    /// A name associated with the person
    #[fhir(name="name", min="0", max="*", summary=true, modifier=false)]
    pub name: Option<Vec<HumanName>>,
    /// A contact detail for the person
    #[fhir(name="telecom", min="0", max="*", summary=true, modifier=false)]
    pub telecom: Option<Vec<ContactPoint>>,
    /// male | female | other | unknown
    #[fhir(name="gender", min="0", max="1", summary=true, modifier=false)]
    pub gender: Option<CodeDt>,
    /// The date on which the person was born
    #[fhir(name="birthDate", min="0", max="1", summary=true, modifier=false)]
    pub birth_date: Option<DateDt>,
    /// Indicates if the individual is deceased or not
    #[fhir(name="deceased", min="0", max="1", summary=true, modifier=false)]
    pub deceased: Option<DateTimeDt>,
    /// One or more addresses for the person
    #[fhir(name="address", min="0", max="*", summary=true, modifier=false)]
    pub address: Option<Vec<Address>>,
    /// Marital (civil) status of a person
    #[fhir(name="maritalStatus", min="0", max="1", summary=false, modifier=false)]
    pub marital_status: Option<CodeableConcept>,
    /// Image of the person
    #[fhir(name="photo", min="0", max="*", summary=false, modifier=false)]
    pub photo: Option<Vec<Attachment>>,
    /// A language which may be used to communicate with the person about his or her health
    #[fhir(name="communication", min="0", max="*", summary=false, modifier=false)]
    pub communication: Option<Vec<PersonCommunicationBackboneElement>>,
    /// The organization that is the custodian of the person record
    #[fhir(name="managingOrganization", min="0", max="1", summary=true, modifier=false)]
    pub managing_organization: Option<Reference>,
    /// Link to a resource that concerns the same actual person
    #[fhir(name="link", min="0", max="*", summary=false, modifier=false)]
    pub link: Option<Vec<PersonLinkBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PersonLinkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The resource to which this actual person is associated
    #[fhir(name="target", min="1", max="1", summary=false, modifier=false)]
    pub target: Option<Reference>,
    /// level1 | level2 | level3 | level4
    #[fhir(name="assurance", min="0", max="1", summary=false, modifier=false)]
    pub assurance: Option<CodeDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PersonCommunicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The language which can be used to communicate with the person about his or her health
    #[fhir(name="language", min="1", max="1", summary=false, modifier=false)]
    pub language: Option<CodeableConcept>,
    /// Language preference indicator
    #[fhir(name="preferred", min="0", max="1", summary=false, modifier=false)]
    pub preferred: Option<BooleanDt>,
}


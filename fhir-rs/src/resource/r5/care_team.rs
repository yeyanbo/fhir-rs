use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CareTeam {
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
    /// External Ids for this team
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// proposed | active | suspended | inactive | entered-in-error
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Type of team
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// Name of the team, such as crisis assessment team
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Who care team is for
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// Time period team covers
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
    /// Members of the team
    #[fhir(name="participant", min="0", max="*", summary=false, modifier=false)]
    pub participant: Option<Vec<CareTeamParticipantBackboneElement>>,
    /// Why the care team exists
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// Organization responsible for the care team
    #[fhir(name="managingOrganization", min="0", max="*", summary=true, modifier=false)]
    pub managing_organization: Option<Vec<Reference>>,
    /// A contact detail for the care team (that applies to all members)
    #[fhir(name="telecom", min="0", max="*", summary=false, modifier=false)]
    pub telecom: Option<Vec<ContactPoint>>,
    /// Comments made about the CareTeam
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct CareTeamParticipantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of involvement
    #[fhir(name="role", min="0", max="1", summary=true, modifier=false)]
    pub role: Option<CodeableConcept>,
    /// Who is involved
    #[fhir(name="member", min="0", max="1", summary=true, modifier=false)]
    pub member: Option<Reference>,
    /// Organization of the practitioner
    #[fhir(name="onBehalfOf", min="0", max="1", summary=true, modifier=false)]
    pub on_behalf_of: Option<Reference>,
    /// When the member is generally available within this care team
    #[fhir(name="coverage", min="0", max="1", summary=false, modifier=false)]
    pub coverage: Option<Timing>,
}


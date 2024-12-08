use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct CarePlan {
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
    /// External Ids for this plan
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR protocol or definition
    #[fhir(name="instantiatesCanonical", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_canonical: Option<Vec<CanonicalDt>>,
    /// Instantiates external protocol or definition
    #[fhir(name="instantiatesUri", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instantiates_uri: Option<Vec<UriDt>>,
    /// Fulfills plan, proposal or order
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// CarePlan replaced by this CarePlan
    #[fhir(name="replaces", min="0", max="*", summary=true, modifier=false, choice="")]
    pub replaces: Option<Vec<Reference>>,
    /// Part of referenced CarePlan
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// draft | active | on-hold | revoked | completed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// proposal | plan | order | option | directive
    #[fhir(name="intent", min="1", max="1", summary=true, modifier=true)]
    pub intent: Option<CodeDt>,
    /// Type of plan
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Human-friendly name for the care plan
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false, choice="")]
    pub title: Option<StringDt>,
    /// Summary of nature of plan
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<StringDt>,
    /// Who the care plan is for
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// The Encounter during which this CarePlan was created
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Time period plan covers
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Date record was first recorded
    #[fhir(name="created", min="0", max="1", summary=true, modifier=false, choice="")]
    pub created: Option<DateTimeDt>,
    /// Who is the designated responsible party
    #[fhir(name="custodian", min="0", max="1", summary=true, modifier=false, choice="")]
    pub custodian: Option<Reference>,
    /// Who provided the content of the care plan
    #[fhir(name="contributor", min="0", max="*", summary=false, modifier=false, choice="")]
    pub contributor: Option<Vec<Reference>>,
    /// Who's involved in plan?
    #[fhir(name="careTeam", min="0", max="*", summary=false, modifier=false, choice="")]
    pub care_team: Option<Vec<Reference>>,
    /// Health issues this plan addresses
    #[fhir(name="addresses", min="0", max="*", summary=true, modifier=false, choice="")]
    pub addresses: Option<Vec<CodeableReference>>,
    /// Information considered as part of plan
    #[fhir(name="supportingInfo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub supporting_info: Option<Vec<Reference>>,
    /// Desired outcome of plan
    #[fhir(name="goal", min="0", max="*", summary=false, modifier=false, choice="")]
    pub goal: Option<Vec<Reference>>,
    /// Action to occur or has occurred as part of plan
    #[fhir(name="activity", min="0", max="*", summary=false, modifier=false, choice="")]
    pub activity: Option<Vec<CarePlanActivityBackboneElement>>,
    /// Comments about the plan
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct CarePlanActivityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Results of the activity (concept, or Appointment, Encounter, Procedure, etc.)
    #[fhir(name="performedActivity", min="0", max="*", summary=false, modifier=false, choice="")]
    pub performed_activity: Option<Vec<CodeableReference>>,
    /// Comments about the activity status/progress
    #[fhir(name="progress", min="0", max="*", summary=false, modifier=false, choice="")]
    pub progress: Option<Vec<Annotation>>,
    /// Activity that is intended to be part of the care plan
    #[fhir(name="plannedActivityReference", min="0", max="1", summary=false, modifier=false, choice="")]
    pub planned_activity_reference: Option<Reference>,
}


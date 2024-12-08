use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct EpisodeOfCare {
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
    /// Business Identifier(s) relevant for this EpisodeOfCare
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Past list of status codes (the current status may be included to cover the start date of the status)
    #[fhir(name="statusHistory", min="0", max="*", summary=false, modifier=false, choice="")]
    pub status_history: Option<Vec<EpisodeOfCareStatusHistoryBackboneElement>>,
    /// Type/class  - e.g. specialist referral, disease management
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// The list of medical reasons that are expected to be addressed during the episode of care
    #[fhir(name="reason", min="0", max="*", summary=true, modifier=false, choice="")]
    pub reason: Option<Vec<EpisodeOfCareReasonBackboneElement>>,
    /// The list of medical conditions that were addressed during the episode of care
    #[fhir(name="diagnosis", min="0", max="*", summary=true, modifier=false, choice="")]
    pub diagnosis: Option<Vec<EpisodeOfCareDiagnosisBackboneElement>>,
    /// The patient who is the focus of this episode of care
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false, choice="")]
    pub patient: Option<Reference>,
    /// Organization that assumes responsibility for care coordination
    #[fhir(name="managingOrganization", min="0", max="1", summary=true, modifier=false, choice="")]
    pub managing_organization: Option<Reference>,
    /// Interval during responsibility is assumed
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Originating Referral Request(s)
    #[fhir(name="referralRequest", min="0", max="*", summary=false, modifier=false, choice="")]
    pub referral_request: Option<Vec<Reference>>,
    /// Care manager/care coordinator for the patient
    #[fhir(name="careManager", min="0", max="1", summary=false, modifier=false, choice="")]
    pub care_manager: Option<Reference>,
    /// Other practitioners facilitating this episode of care
    #[fhir(name="careTeam", min="0", max="*", summary=false, modifier=false, choice="")]
    pub care_team: Option<Vec<Reference>>,
    /// The set of accounts that may be used for billing for this EpisodeOfCare
    #[fhir(name="account", min="0", max="*", summary=false, modifier=false, choice="")]
    pub account: Option<Vec<Reference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EpisodeOfCareDiagnosisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The medical condition that was addressed during the episode of care
    #[fhir(name="condition", min="0", max="*", summary=true, modifier=false, choice="")]
    pub condition: Option<Vec<CodeableReference>>,
    /// Role that this diagnosis has within the episode of care (e.g. admission, billing, discharge …)
    #[fhir(name="use", min="0", max="1", summary=true, modifier=false, choice="")]
    pub use_: Option<CodeableConcept>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EpisodeOfCareReasonBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What the reason value should be used for/as
    #[fhir(name="use", min="0", max="1", summary=true, modifier=false, choice="")]
    pub use_: Option<CodeableConcept>,
    /// Medical reason to be addressed
    #[fhir(name="value", min="0", max="*", summary=true, modifier=false, choice="")]
    pub value: Option<Vec<CodeableReference>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct EpisodeOfCareStatusHistoryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// planned | waitlist | active | onhold | finished | cancelled | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=false, modifier=false, choice="")]
    pub status: Option<CodeDt>,
    /// Duration the EpisodeOfCare was in the specified status
    #[fhir(name="period", min="1", max="1", summary=false, modifier=false, choice="")]
    pub period: Option<Period>,
}


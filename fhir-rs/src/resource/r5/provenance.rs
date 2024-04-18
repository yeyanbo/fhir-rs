use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Provenance {
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
    /// Target Reference(s) (usually version specific)
    #[fhir(name="target", min="1", max="*", summary=true, modifier=false)]
    pub target: Option<Vec<Reference>>,
    /// When the activity occurred
    #[fhir(name="occurred", min="0", max="1", summary=false, modifier=false)]
    pub occurred: Option<DateTimeDt>,
    /// When the activity was recorded / updated
    #[fhir(name="recorded", min="0", max="1", summary=true, modifier=false)]
    pub recorded: Option<InstantDt>,
    /// Policy or plan the activity was defined by
    #[fhir(name="policy", min="0", max="*", summary=false, modifier=false)]
    pub policy: Option<Vec<UriDt>>,
    /// Where the activity occurred, if relevant
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false)]
    pub location: Option<Reference>,
    /// Authorization (purposeOfUse) related to the event
    #[fhir(name="authorization", min="0", max="*", summary=false, modifier=false)]
    pub authorization: Option<Vec<CodeableReference>>,
    /// Activity that occurred
    #[fhir(name="activity", min="0", max="1", summary=false, modifier=false)]
    pub activity: Option<CodeableConcept>,
    /// Workflow authorization within which this event occurred
    #[fhir(name="basedOn", min="0", max="*", summary=false, modifier=false)]
    pub based_on: Option<Vec<Reference>>,
    /// The patient is the subject of the data created/updated (.target) by the activity
    #[fhir(name="patient", min="0", max="1", summary=false, modifier=false)]
    pub patient: Option<Reference>,
    /// Encounter within which this event occurred or which the event is tightly associated
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false)]
    pub encounter: Option<Reference>,
    /// Actor involved
    #[fhir(name="agent", min="1", max="*", summary=true, modifier=false)]
    pub agent: Option<Vec<ProvenanceAgentBackboneElement>>,
    /// An entity used in this activity
    #[fhir(name="entity", min="0", max="*", summary=true, modifier=false)]
    pub entity: Option<Vec<ProvenanceEntityBackboneElement>>,
    /// Signature on target
    #[fhir(name="signature", min="0", max="*", summary=false, modifier=false)]
    pub signature: Option<Vec<Signature>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ProvenanceAgentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// How the agent participated
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// What the agents role was
    #[fhir(name="role", min="0", max="*", summary=false, modifier=false)]
    pub role: Option<Vec<CodeableConcept>>,
    /// The agent that participated in the event
    #[fhir(name="who", min="1", max="1", summary=true, modifier=false)]
    pub who: Option<Reference>,
    /// The agent that delegated
    #[fhir(name="onBehalfOf", min="0", max="1", summary=false, modifier=false)]
    pub on_behalf_of: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ProvenanceEntityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// revision | quotation | source | instantiates | removal
    #[fhir(name="role", min="1", max="1", summary=true, modifier=false)]
    pub role: Option<CodeDt>,
    /// Identity of entity
    #[fhir(name="what", min="1", max="1", summary=true, modifier=false)]
    pub what: Option<Reference>,
    /// Entity is attributed to this agent
    #[fhir(name="agent", min="0", max="*", summary=false, modifier=false)]
    pub agent: Option<Vec<ProvenanceAgentBackboneElement>>,
}


use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Schedule {
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
    /// External Ids for this item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this schedule is in active use
    #[fhir(name="active", min="0", max="1", summary=true, modifier=true)]
    pub active: Option<BooleanDt>,
    /// High-level category
    #[fhir(name="serviceCategory", min="0", max="*", summary=true, modifier=false)]
    pub service_category: Option<Vec<CodeableConcept>>,
    /// Specific service
    #[fhir(name="serviceType", min="0", max="*", summary=true, modifier=false)]
    pub service_type: Option<Vec<CodeableReference>>,
    /// Type of specialty needed
    #[fhir(name="specialty", min="0", max="*", summary=true, modifier=false)]
    pub specialty: Option<Vec<CodeableConcept>>,
    /// Human-readable label
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Resource(s) that availability information is being provided for
    #[fhir(name="actor", min="1", max="*", summary=true, modifier=false)]
    pub actor: Option<Vec<Reference>>,
    /// Period of time covered by schedule
    #[fhir(name="planningHorizon", min="0", max="1", summary=true, modifier=false)]
    pub planning_horizon: Option<Period>,
    /// Comments on availability
    #[fhir(name="comment", min="0", max="1", summary=false, modifier=false)]
    pub comment: Option<MarkdownDt>,
}


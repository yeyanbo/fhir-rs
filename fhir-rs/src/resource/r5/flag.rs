use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Flag {
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
    /// Business identifier
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | inactive | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Clinical, administrative, etc
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Coded or textual message to display to user
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Who/What is flag about?
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Time period when flag is active
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false, choice="")]
    pub period: Option<Period>,
    /// Alert relevant during encounter
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Flag creator
    #[fhir(name="author", min="0", max="1", summary=true, modifier=false, choice="")]
    pub author: Option<Reference>,
}


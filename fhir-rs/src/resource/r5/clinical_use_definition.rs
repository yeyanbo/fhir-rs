use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ClinicalUseDefinition {
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
    /// Business identifier for this issue
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// indication | contraindication | interaction | undesirable-effect | warning
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// A categorisation of the issue, primarily for dividing warnings into subject heading areas such as "Pregnancy", "Overdose"
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// The medication, product, substance, device, procedure etc. for which this is an indication
    #[fhir(name="subject", min="0", max="*", summary=true, modifier=false, choice="")]
    pub subject: Option<Vec<Reference>>,
    /// Whether this is a current issue or one that has been retired etc
    #[fhir(name="status", min="0", max="1", summary=true, modifier=false, choice="")]
    pub status: Option<CodeableConcept>,
    /// Specifics for when this is a contraindication
    #[fhir(name="contraindication", min="0", max="1", summary=true, modifier=false, choice="")]
    pub contraindication: Option<ClinicalUseDefinitionContraindicationBackboneElement>,
    /// Specifics for when this is an indication
    #[fhir(name="indication", min="0", max="1", summary=true, modifier=false, choice="")]
    pub indication: Option<ClinicalUseDefinitionIndicationBackboneElement>,
    /// Specifics for when this is an interaction
    #[fhir(name="interaction", min="0", max="1", summary=true, modifier=false, choice="")]
    pub interaction: Option<ClinicalUseDefinitionInteractionBackboneElement>,
    /// The population group to which this applies
    #[fhir(name="population", min="0", max="*", summary=true, modifier=false, choice="")]
    pub population: Option<Vec<Reference>>,
    /// Logic used by the clinical use definition
    #[fhir(name="library", min="0", max="*", summary=true, modifier=false, choice="")]
    pub library: Option<Vec<CanonicalDt>>,
    /// A possible negative outcome from the use of this treatment
    #[fhir(name="undesirableEffect", min="0", max="1", summary=true, modifier=false, choice="")]
    pub undesirable_effect: Option<ClinicalUseDefinitionUndesirableEffectBackboneElement>,
    /// Critical environmental, health or physical risks or hazards. For example 'Do not operate heavy machinery', 'May cause drowsiness'
    #[fhir(name="warning", min="0", max="1", summary=true, modifier=false, choice="")]
    pub warning: Option<ClinicalUseDefinitionWarningBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionInteractionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The specific medication, product, food etc. or laboratory test that interacts
    #[fhir(name="interactant", min="0", max="*", summary=true, modifier=false, choice="")]
    pub interactant: Option<Vec<ClinicalUseDefinitionInteractionInteractantBackboneElement>>,
    /// The type of the interaction e.g. drug-drug interaction, drug-lab test interaction
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// The effect of the interaction, for example "reduced gastric absorption of primary medication"
    #[fhir(name="effect", min="0", max="1", summary=true, modifier=false, choice="")]
    pub effect: Option<CodeableReference>,
    /// The incidence of the interaction, e.g. theoretical, observed
    #[fhir(name="incidence", min="0", max="1", summary=true, modifier=false, choice="")]
    pub incidence: Option<CodeableConcept>,
    /// Actions for managing the interaction
    #[fhir(name="management", min="0", max="*", summary=true, modifier=false, choice="")]
    pub management: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionInteractionInteractantBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The specific medication, product, food etc. or laboratory test that interacts
    #[fhir(name="item", min="1", max="1", summary=true, modifier=false, choice="")]
    pub item: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionContraindicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The situation that is being documented as contraindicating against this item
    #[fhir(name="diseaseSymptomProcedure", min="0", max="1", summary=true, modifier=false, choice="")]
    pub disease_symptom_procedure: Option<CodeableReference>,
    /// The status of the disease or symptom for the contraindication
    #[fhir(name="diseaseStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub disease_status: Option<CodeableReference>,
    /// A comorbidity (concurrent condition) or coinfection
    #[fhir(name="comorbidity", min="0", max="*", summary=true, modifier=false, choice="")]
    pub comorbidity: Option<Vec<CodeableReference>>,
    /// The indication which this is a contraidication for
    #[fhir(name="indication", min="0", max="*", summary=true, modifier=false, choice="")]
    pub indication: Option<Vec<Reference>>,
    /// An expression that returns true or false, indicating whether the indication is applicable or not, after having applied its other elements
    #[fhir(name="applicability", min="0", max="1", summary=false, modifier=false, choice="")]
    pub applicability: Option<Expression>,
    /// Information about use of the product in relation to other therapies described as part of the contraindication
    #[fhir(name="otherTherapy", min="0", max="*", summary=true, modifier=false, choice="")]
    pub other_therapy: Option<Vec<ClinicalUseDefinitionContraindicationOtherTherapyBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionContraindicationOtherTherapyBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The type of relationship between the product indication/contraindication and another therapy
    #[fhir(name="relationshipType", min="1", max="1", summary=true, modifier=false, choice="")]
    pub relationship_type: Option<CodeableConcept>,
    /// Reference to a specific medication, substance etc. as part of an indication or contraindication
    #[fhir(name="treatment", min="1", max="1", summary=true, modifier=false, choice="")]
    pub treatment: Option<CodeableReference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionUndesirableEffectBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The situation in which the undesirable effect may manifest
    #[fhir(name="symptomConditionEffect", min="0", max="1", summary=true, modifier=false, choice="")]
    pub symptom_condition_effect: Option<CodeableReference>,
    /// High level classification of the effect
    #[fhir(name="classification", min="0", max="1", summary=true, modifier=false, choice="")]
    pub classification: Option<CodeableConcept>,
    /// How often the effect is seen
    #[fhir(name="frequencyOfOccurrence", min="0", max="1", summary=true, modifier=false, choice="")]
    pub frequency_of_occurrence: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionWarningBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// A textual definition of this warning, with formatting
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false, choice="")]
    pub description: Option<MarkdownDt>,
    /// A coded or unformatted textual definition of this warning
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ClinicalUseDefinitionIndicationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The situation that is being documented as an indicaton for this item
    #[fhir(name="diseaseSymptomProcedure", min="0", max="1", summary=true, modifier=false, choice="")]
    pub disease_symptom_procedure: Option<CodeableReference>,
    /// The status of the disease or symptom for the indication
    #[fhir(name="diseaseStatus", min="0", max="1", summary=true, modifier=false, choice="")]
    pub disease_status: Option<CodeableReference>,
    /// A comorbidity or coinfection as part of the indication
    #[fhir(name="comorbidity", min="0", max="*", summary=true, modifier=false, choice="")]
    pub comorbidity: Option<Vec<CodeableReference>>,
    /// The intended effect, aim or strategy to be achieved
    #[fhir(name="intendedEffect", min="0", max="1", summary=true, modifier=false, choice="")]
    pub intended_effect: Option<CodeableReference>,
    /// Timing or duration information
    #[fhir(name="duration", min="0", max="1", summary=true, modifier=false, choice="")]
    pub duration: Option<StringDt>,
    /// An unwanted side effect or negative outcome of the subject of this resource when being used for this indication
    #[fhir(name="undesirableEffect", min="0", max="*", summary=true, modifier=false, choice="")]
    pub undesirable_effect: Option<Vec<Reference>>,
    /// An expression that returns true or false, indicating whether the indication is applicable or not, after having applied its other elements
    #[fhir(name="applicability", min="0", max="1", summary=false, modifier=false, choice="")]
    pub applicability: Option<Expression>,
    /// The use of the medicinal product in relation to other therapies described as part of the indication
    #[fhir(name="otherTherapy", min="0", max="*", summary=true, modifier=false, choice="")]
    pub other_therapy: Option<Vec<ClinicalUseDefinitionContraindicationOtherTherapyBackboneElement>>,
}


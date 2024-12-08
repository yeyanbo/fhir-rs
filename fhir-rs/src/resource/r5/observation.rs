use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Observation {
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
    /// Business Identifier for observation
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// Instantiates FHIR ObservationDefinition
    #[fhir(name="instantiates", min="0", max="1", summary=true, modifier=false, choice="")]
    pub instantiates: Option<Reference>,
    /// Fulfills plan, proposal or order
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Triggering observation(s)
    #[fhir(name="triggeredBy", min="0", max="*", summary=false, modifier=false, choice="")]
    pub triggered_by: Option<Vec<ObservationTriggeredByBackboneElement>>,
    /// Part of referenced event
    #[fhir(name="partOf", min="0", max="*", summary=true, modifier=false, choice="")]
    pub part_of: Option<Vec<Reference>>,
    /// registered | preliminary | final | amended +
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Classification of  type of observation
    #[fhir(name="category", min="0", max="*", summary=false, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Type of observation (code / type)
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Who and/or what the observation is about
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// What the observation is about, when it is not about the subject of record
    #[fhir(name="focus", min="0", max="*", summary=true, modifier=false, choice="")]
    pub focus: Option<Vec<Reference>>,
    /// Healthcare event during which this observation is made
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false, choice="")]
    pub encounter: Option<Reference>,
    /// Clinically relevant time/time-period for observation
    #[fhir(name="effective", min="0", max="1", summary=true, modifier=false, choice="")]
    pub effective: Option<InstantDt>,
    /// Date/Time this version was made available
    #[fhir(name="issued", min="0", max="1", summary=true, modifier=false, choice="")]
    pub issued: Option<InstantDt>,
    /// Who is responsible for the observation
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false, choice="")]
    pub performer: Option<Vec<Reference>>,
    /// Actual result
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false, choice="")]
    pub value: Option<Reference>,
    /// Why the result is missing
    #[fhir(name="dataAbsentReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// High, low, normal, etc
    #[fhir(name="interpretation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub interpretation: Option<Vec<CodeableConcept>>,
    /// Comments about the observation
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false, choice="")]
    pub note: Option<Vec<Annotation>>,
    /// Observed body part
    #[fhir(name="bodySite", min="0", max="1", summary=false, modifier=false, choice="")]
    pub body_site: Option<CodeableConcept>,
    /// Observed body structure
    #[fhir(name="bodyStructure", min="0", max="1", summary=false, modifier=false, choice="")]
    pub body_structure: Option<Reference>,
    /// How it was done
    #[fhir(name="method", min="0", max="1", summary=false, modifier=false, choice="")]
    pub method: Option<CodeableConcept>,
    /// Specimen used for this observation
    #[fhir(name="specimen", min="0", max="1", summary=false, modifier=false, choice="")]
    pub specimen: Option<Reference>,
    /// A reference to the device that generates the measurements or the device settings for the device
    #[fhir(name="device", min="0", max="1", summary=false, modifier=false, choice="")]
    pub device: Option<Reference>,
    /// Provides guide for interpretation
    #[fhir(name="referenceRange", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reference_range: Option<Vec<ObservationReferenceRangeBackboneElement>>,
    /// Related resource that belongs to the Observation group
    #[fhir(name="hasMember", min="0", max="*", summary=true, modifier=false, choice="")]
    pub has_member: Option<Vec<Reference>>,
    /// Related resource from which the observation is made
    #[fhir(name="derivedFrom", min="0", max="*", summary=true, modifier=false, choice="")]
    pub derived_from: Option<Vec<Reference>>,
    /// Component results
    #[fhir(name="component", min="0", max="*", summary=true, modifier=false, choice="")]
    pub component: Option<Vec<ObservationComponentBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ObservationTriggeredByBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Triggering observation
    #[fhir(name="observation", min="1", max="1", summary=true, modifier=false, choice="")]
    pub observation: Option<Reference>,
    /// reflex | repeat | re-run
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false, choice="")]
    pub type_: Option<CodeDt>,
    /// Reason that the observation was triggered
    #[fhir(name="reason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub reason: Option<StringDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ObservationReferenceRangeBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Low Range, if relevant
    #[fhir(name="low", min="0", max="1", summary=false, modifier=false, choice="")]
    pub low: Option<Quantity>,
    /// High Range, if relevant
    #[fhir(name="high", min="0", max="1", summary=false, modifier=false, choice="")]
    pub high: Option<Quantity>,
    /// Normal value, if relevant
    #[fhir(name="normalValue", min="0", max="1", summary=false, modifier=false, choice="")]
    pub normal_value: Option<CodeableConcept>,
    /// Reference range qualifier
    #[fhir(name="type", min="0", max="1", summary=false, modifier=false, choice="")]
    pub type_: Option<CodeableConcept>,
    /// Reference range population
    #[fhir(name="appliesTo", min="0", max="*", summary=false, modifier=false, choice="")]
    pub applies_to: Option<Vec<CodeableConcept>>,
    /// Applicable age range, if relevant
    #[fhir(name="age", min="0", max="1", summary=false, modifier=false, choice="")]
    pub age: Option<Range>,
    /// Text based reference range in an observation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice="")]
    pub text: Option<MarkdownDt>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ObservationComponentBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of component observation (code / type)
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// Actual component result
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false, choice="")]
    pub value: Option<Reference>,
    /// Why the component result is missing
    #[fhir(name="dataAbsentReason", min="0", max="1", summary=false, modifier=false, choice="")]
    pub data_absent_reason: Option<CodeableConcept>,
    /// High, low, normal, etc
    #[fhir(name="interpretation", min="0", max="*", summary=false, modifier=false, choice="")]
    pub interpretation: Option<Vec<CodeableConcept>>,
    /// Provides guide for interpretation of component result
    #[fhir(name="referenceRange", min="0", max="*", summary=false, modifier=false, choice="")]
    pub reference_range: Option<Vec<ObservationReferenceRangeBackboneElement>>,
}


use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Immunization {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary=true, modifier=true)]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Business identifier
    #[fhir(name="identifier", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Authority that the immunization event is based on
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub based_on: Option<Vec<Reference>>,
    /// completed | entered-in-error | not-done
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Reason for current status
    #[fhir(name="statusReason", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub status_reason: Option<CodeableConcept>,
    /// Vaccine administered
    #[fhir(name="vaccineCode", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub vaccine_code: Option<CodeableConcept>,
    /// Product that was administered
    #[fhir(name="administeredProduct", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub administered_product: Option<CodeableReference>,
    /// Vaccine manufacturer
    #[fhir(name="manufacturer", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub manufacturer: Option<CodeableReference>,
    /// Vaccine lot number
    #[fhir(name="lotNumber", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub lot_number: Option<StringDt>,
    /// Vaccine expiration date
    #[fhir(name="expirationDate", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub expiration_date: Option<DateDt>,
    /// Who was immunized
    #[fhir(name="patient", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub patient: Option<Reference>,
    /// Encounter immunization was part of
    #[fhir(name="encounter", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub encounter: Option<Reference>,
    /// Additional information in support of the immunization
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub supporting_information: Option<Vec<Reference>>,
    /// Vaccine administration date
    #[fhir(name="occurrence", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub occurrence: Option<StringDt>,
    /// Indicates context the data was captured in
    #[fhir(name="primarySource", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub primary_source: Option<BooleanDt>,
    /// Indicates the source of a  reported record
    #[fhir(name="informationSource", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub information_source: Option<CodeableReference>,
    /// Where immunization occurred
    #[fhir(name="location", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub location: Option<Reference>,
    /// Body site vaccine  was administered
    #[fhir(name="site", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub site: Option<CodeableConcept>,
    /// How vaccine entered body
    #[fhir(name="route", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub route: Option<CodeableConcept>,
    /// Amount of vaccine administered
    #[fhir(name="doseQuantity", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub dose_quantity: Option<Quantity>,
    /// Who performed event
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub performer: Option<Vec<ImmunizationPerformerBackboneElement>>,
    /// Additional immunization notes
    #[fhir(name="note", min="0", max="*", summary=true, modifier=false, choice=false)]
    pub note: Option<Vec<Annotation>>,
    /// Why immunization occurred
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub reason: Option<Vec<CodeableReference>>,
    /// Dose potency
    #[fhir(name="isSubpotent", min="0", max="1", summary=true, modifier=true)]
    pub is_subpotent: Option<BooleanDt>,
    /// Reason for being subpotent
    #[fhir(name="subpotentReason", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub subpotent_reason: Option<Vec<CodeableConcept>>,
    /// Patient eligibility for a specific vaccination program
    #[fhir(name="programEligibility", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub program_eligibility: Option<Vec<ImmunizationProgramEligibilityBackboneElement>>,
    /// Funding source for the vaccine
    #[fhir(name="fundingSource", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub funding_source: Option<CodeableConcept>,
    /// Details of a reaction that follows immunization
    #[fhir(name="reaction", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub reaction: Option<Vec<ImmunizationReactionBackboneElement>>,
    /// Protocol followed by the provider
    #[fhir(name="protocolApplied", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub protocol_applied: Option<Vec<ImmunizationProtocolAppliedBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImmunizationProtocolAppliedBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Name of vaccine series
    #[fhir(name="series", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub series: Option<StringDt>,
    /// Who is responsible for publishing the recommendations
    #[fhir(name="authority", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub authority: Option<Reference>,
    /// Vaccine preventatable disease being targeted
    #[fhir(name="targetDisease", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub target_disease: Option<Vec<CodeableConcept>>,
    /// Dose number within series
    #[fhir(name="doseNumber", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub dose_number: Option<StringDt>,
    /// Recommended number of doses for immunity
    #[fhir(name="seriesDoses", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub series_doses: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImmunizationReactionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// When reaction started
    #[fhir(name="date", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub date: Option<DateTimeDt>,
    /// Additional information on reaction
    #[fhir(name="manifestation", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub manifestation: Option<CodeableReference>,
    /// Indicates self-reported reaction
    #[fhir(name="reported", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub reported: Option<BooleanDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImmunizationPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What type of performance was done
    #[fhir(name="function", min="0", max="1", summary=true, modifier=false, choice=false)]
    pub function: Option<CodeableConcept>,
    /// Individual or organization who was performing
    #[fhir(name="actor", min="1", max="1", summary=true, modifier=false, choice=false)]
    pub actor: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ImmunizationProgramEligibilityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The program that eligibility is declared for
    #[fhir(name="program", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub program: Option<CodeableConcept>,
    /// The patient's eligibility status for the program
    #[fhir(name="programStatus", min="1", max="1", summary=false, modifier=false, choice=false)]
    pub program_status: Option<CodeableConcept>,
}


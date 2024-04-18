use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ChargeItem {
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
    /// Business Identifier for item
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// Defining information about the code of this charge item
    #[fhir(name="definitionUri", min="0", max="*", summary=false, modifier=false)]
    pub definition_uri: Option<Vec<UriDt>>,
    /// Resource defining the code of this ChargeItem
    #[fhir(name="definitionCanonical", min="0", max="*", summary=false, modifier=false)]
    pub definition_canonical: Option<Vec<CanonicalDt>>,
    /// planned | billable | not-billable | aborted | billed | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Part of referenced ChargeItem
    #[fhir(name="partOf", min="0", max="*", summary=false, modifier=false)]
    pub part_of: Option<Vec<Reference>>,
    /// A code that identifies the charge, like a billing code
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// Individual service was done for/to
    #[fhir(name="subject", min="1", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// Encounter associated with this ChargeItem
    #[fhir(name="encounter", min="0", max="1", summary=true, modifier=false)]
    pub encounter: Option<Reference>,
    /// When the charged service was applied
    #[fhir(name="occurrence", min="0", max="1", summary=true, modifier=false)]
    pub occurrence: Option<Timing>,
    /// Who performed charged service
    #[fhir(name="performer", min="0", max="*", summary=false, modifier=false)]
    pub performer: Option<Vec<ChargeItemPerformerBackboneElement>>,
    /// Organization providing the charged service
    #[fhir(name="performingOrganization", min="0", max="1", summary=false, modifier=false)]
    pub performing_organization: Option<Reference>,
    /// Organization requesting the charged service
    #[fhir(name="requestingOrganization", min="0", max="1", summary=false, modifier=false)]
    pub requesting_organization: Option<Reference>,
    /// Organization that has ownership of the (potential, future) revenue
    #[fhir(name="costCenter", min="0", max="1", summary=false, modifier=false)]
    pub cost_center: Option<Reference>,
    /// Quantity of which the charge item has been serviced
    #[fhir(name="quantity", min="0", max="1", summary=true, modifier=false)]
    pub quantity: Option<Quantity>,
    /// Anatomical location, if relevant
    #[fhir(name="bodysite", min="0", max="*", summary=true, modifier=false)]
    pub bodysite: Option<Vec<CodeableConcept>>,
    /// Unit price overriding the associated rules
    #[fhir(name="unitPriceComponent", min="0", max="1", summary=false, modifier=false)]
    pub unit_price_component: Option<MonetaryComponent>,
    /// Total price overriding the associated rules
    #[fhir(name="totalPriceComponent", min="0", max="1", summary=false, modifier=false)]
    pub total_price_component: Option<MonetaryComponent>,
    /// Reason for overriding the list price/factor
    #[fhir(name="overrideReason", min="0", max="1", summary=false, modifier=false)]
    pub override_reason: Option<CodeableConcept>,
    /// Individual who was entering
    #[fhir(name="enterer", min="0", max="1", summary=true, modifier=false)]
    pub enterer: Option<Reference>,
    /// Date the charge item was entered
    #[fhir(name="enteredDate", min="0", max="1", summary=true, modifier=false)]
    pub entered_date: Option<DateTimeDt>,
    /// Why was the charged  service rendered?
    #[fhir(name="reason", min="0", max="*", summary=false, modifier=false)]
    pub reason: Option<Vec<CodeableConcept>>,
    /// Which rendered service is being charged?
    #[fhir(name="service", min="0", max="*", summary=false, modifier=false)]
    pub service: Option<Vec<CodeableReference>>,
    /// Product charged
    #[fhir(name="product", min="0", max="*", summary=false, modifier=false)]
    pub product: Option<Vec<CodeableReference>>,
    /// Account to place this charge
    #[fhir(name="account", min="0", max="*", summary=true, modifier=false)]
    pub account: Option<Vec<Reference>>,
    /// Comments made about the ChargeItem
    #[fhir(name="note", min="0", max="*", summary=false, modifier=false)]
    pub note: Option<Vec<Annotation>>,
    /// Further information supporting this charge
    #[fhir(name="supportingInformation", min="0", max="*", summary=false, modifier=false)]
    pub supporting_information: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ChargeItemPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// What type of performance was done
    #[fhir(name="function", min="0", max="1", summary=false, modifier=false)]
    pub function: Option<CodeableConcept>,
    /// Individual who was performing
    #[fhir(name="actor", min="1", max="1", summary=false, modifier=false)]
    pub actor: Option<Reference>,
}


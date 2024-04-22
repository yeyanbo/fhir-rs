use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct VerificationResult {
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
    /// A resource that was validated
    #[fhir(name="target", min="0", max="*", summary=true, modifier=false, choice="")]
    pub target: Option<Vec<Reference>>,
    /// The fhirpath location(s) within the resource that was validated
    #[fhir(name="targetLocation", min="0", max="*", summary=true, modifier=false, choice="")]
    pub target_location: Option<Vec<StringDt>>,
    /// none | initial | periodic
    #[fhir(name="need", min="0", max="1", summary=true, modifier=false, choice="")]
    pub need: Option<CodeableConcept>,
    /// attested | validated | in-process | req-revalid | val-fail | reval-fail | entered-in-error
    #[fhir(name="status", min="1", max="1", summary=true, modifier=false, choice="")]
    pub status: Option<CodeDt>,
    /// When the validation status was updated
    #[fhir(name="statusDate", min="0", max="1", summary=true, modifier=false, choice="")]
    pub status_date: Option<DateTimeDt>,
    /// nothing | primary | multiple
    #[fhir(name="validationType", min="0", max="1", summary=true, modifier=false, choice="")]
    pub validation_type: Option<CodeableConcept>,
    /// The primary process by which the target is validated (edit check; value set; primary source; multiple sources; standalone; in context)
    #[fhir(name="validationProcess", min="0", max="*", summary=true, modifier=false, choice="")]
    pub validation_process: Option<Vec<CodeableConcept>>,
    /// Frequency of revalidation
    #[fhir(name="frequency", min="0", max="1", summary=false, modifier=false, choice="")]
    pub frequency: Option<Timing>,
    /// The date/time validation was last completed (including failed validations)
    #[fhir(name="lastPerformed", min="0", max="1", summary=false, modifier=false, choice="")]
    pub last_performed: Option<DateTimeDt>,
    /// The date when target is next validated, if appropriate
    #[fhir(name="nextScheduled", min="0", max="1", summary=false, modifier=false, choice="")]
    pub next_scheduled: Option<DateDt>,
    /// fatal | warn | rec-only | none
    #[fhir(name="failureAction", min="0", max="1", summary=true, modifier=false, choice="")]
    pub failure_action: Option<CodeableConcept>,
    /// Information about the primary source(s) involved in validation
    #[fhir(name="primarySource", min="0", max="*", summary=false, modifier=false, choice="")]
    pub primary_source: Option<Vec<VerificationResultPrimarySourceBackboneElement>>,
    /// Information about the entity attesting to information
    #[fhir(name="attestation", min="0", max="1", summary=false, modifier=false, choice="")]
    pub attestation: Option<VerificationResultAttestationBackboneElement>,
    /// Information about the entity validating information
    #[fhir(name="validator", min="0", max="*", summary=false, modifier=false, choice="")]
    pub validator: Option<Vec<VerificationResultValidatorBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct VerificationResultAttestationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The individual or organization attesting to information
    #[fhir(name="who", min="0", max="1", summary=true, modifier=false, choice="")]
    pub who: Option<Reference>,
    /// When the who is asserting on behalf of another (organization or individual)
    #[fhir(name="onBehalfOf", min="0", max="1", summary=true, modifier=false, choice="")]
    pub on_behalf_of: Option<Reference>,
    /// The method by which attested information was submitted/retrieved
    #[fhir(name="communicationMethod", min="0", max="1", summary=true, modifier=false, choice="")]
    pub communication_method: Option<CodeableConcept>,
    /// The date the information was attested to
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false, choice="")]
    pub date: Option<DateDt>,
    /// A digital identity certificate associated with the attestation source
    #[fhir(name="sourceIdentityCertificate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub source_identity_certificate: Option<StringDt>,
    /// A digital identity certificate associated with the proxy entity submitting attested information on behalf of the attestation source
    #[fhir(name="proxyIdentityCertificate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub proxy_identity_certificate: Option<StringDt>,
    /// Proxy signature (digital or image)
    #[fhir(name="proxySignature", min="0", max="1", summary=false, modifier=false, choice="")]
    pub proxy_signature: Option<Signature>,
    /// Attester signature (digital or image)
    #[fhir(name="sourceSignature", min="0", max="1", summary=false, modifier=false, choice="")]
    pub source_signature: Option<Signature>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct VerificationResultPrimarySourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to the primary source
    #[fhir(name="who", min="0", max="1", summary=false, modifier=false, choice="")]
    pub who: Option<Reference>,
    /// Type of primary source (License Board; Primary Education; Continuing Education; Postal Service; Relationship owner; Registration Authority; legal source; issuing source; authoritative source)
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false, choice="")]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Method for exchanging information with the primary source
    #[fhir(name="communicationMethod", min="0", max="*", summary=true, modifier=false, choice="")]
    pub communication_method: Option<Vec<CodeableConcept>>,
    /// successful | failed | unknown
    #[fhir(name="validationStatus", min="0", max="1", summary=false, modifier=false, choice="")]
    pub validation_status: Option<CodeableConcept>,
    /// When the target was validated against the primary source
    #[fhir(name="validationDate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub validation_date: Option<DateTimeDt>,
    /// yes | no | undetermined
    #[fhir(name="canPushUpdates", min="0", max="1", summary=true, modifier=false, choice="")]
    pub can_push_updates: Option<CodeableConcept>,
    /// specific | any | source
    #[fhir(name="pushTypeAvailable", min="0", max="*", summary=false, modifier=false, choice="")]
    pub push_type_available: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct VerificationResultValidatorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference to the organization validating information
    #[fhir(name="organization", min="1", max="1", summary=false, modifier=false, choice="")]
    pub organization: Option<Reference>,
    /// A digital identity certificate associated with the validator
    #[fhir(name="identityCertificate", min="0", max="1", summary=false, modifier=false, choice="")]
    pub identity_certificate: Option<StringDt>,
    /// Validator signature (digital or image)
    #[fhir(name="attestationSignature", min="0", max="1", summary=false, modifier=false, choice="")]
    pub attestation_signature: Option<Signature>,
}


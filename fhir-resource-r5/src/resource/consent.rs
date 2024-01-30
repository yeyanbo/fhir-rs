use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Consent {
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
    /// Identifier for this record (external references)
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// draft | active | inactive | not-done | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Classification of the consent statement - for indexing/retrieval
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false)]
    pub category: Option<Vec<CodeableConcept>>,
    /// Who the consent applies to
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// Fully executed date of the consent
    #[fhir(name="date", min="0", max="1", summary=true, modifier=false)]
    pub date: Option<DateDt>,
    /// Effective period for this Consent
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
    /// Who is granting rights according to the policy and rules
    #[fhir(name="grantor", min="0", max="*", summary=true, modifier=false)]
    pub grantor: Option<Vec<Reference>>,
    /// Who is agreeing to the policy and rules
    #[fhir(name="grantee", min="0", max="*", summary=true, modifier=false)]
    pub grantee: Option<Vec<Reference>>,
    /// Consent workflow management
    #[fhir(name="manager", min="0", max="*", summary=false, modifier=false)]
    pub manager: Option<Vec<Reference>>,
    /// Consent Enforcer
    #[fhir(name="controller", min="0", max="*", summary=false, modifier=false)]
    pub controller: Option<Vec<Reference>>,
    /// Source from which this consent is taken
    #[fhir(name="sourceAttachment", min="0", max="*", summary=false, modifier=false)]
    pub source_attachment: Option<Vec<Attachment>>,
    /// Source from which this consent is taken
    #[fhir(name="sourceReference", min="0", max="*", summary=false, modifier=false)]
    pub source_reference: Option<Vec<Reference>>,
    /// Regulations establishing base Consent
    #[fhir(name="regulatoryBasis", min="0", max="*", summary=false, modifier=false)]
    pub regulatory_basis: Option<Vec<CodeableConcept>>,
    /// Computable version of the backing policy
    #[fhir(name="policyBasis", min="0", max="1", summary=false, modifier=false)]
    pub policy_basis: Option<ConsentPolicyBasisBackboneElement>,
    /// Human Readable Policy
    #[fhir(name="policyText", min="0", max="*", summary=false, modifier=false)]
    pub policy_text: Option<Vec<Reference>>,
    /// Consent Verified by patient or family
    #[fhir(name="verification", min="0", max="*", summary=true, modifier=false)]
    pub verification: Option<Vec<ConsentVerificationBackboneElement>>,
    /// deny | permit
    #[fhir(name="decision", min="0", max="1", summary=true, modifier=true)]
    pub decision: Option<CodeDt>,
    /// Constraints to the base Consent.policyRule/Consent.policy
    #[fhir(name="provision", min="0", max="*", summary=true, modifier=false)]
    pub provision: Option<Vec<ConsentProvisionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConsentProvisionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Timeframe for this provision
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
    /// Who|what controlled by this provision (or group, by role)
    #[fhir(name="actor", min="0", max="*", summary=false, modifier=false)]
    pub actor: Option<Vec<ConsentProvisionActorBackboneElement>>,
    /// Actions controlled by this provision
    #[fhir(name="action", min="0", max="*", summary=true, modifier=false)]
    pub action: Option<Vec<CodeableConcept>>,
    /// Security Labels that define affected resources
    #[fhir(name="securityLabel", min="0", max="*", summary=true, modifier=false)]
    pub security_label: Option<Vec<Coding>>,
    /// Context of activities covered by this provision
    #[fhir(name="purpose", min="0", max="*", summary=true, modifier=false)]
    pub purpose: Option<Vec<Coding>>,
    /// e.g. Resource Type, Profile, CDA, etc
    #[fhir(name="documentType", min="0", max="*", summary=true, modifier=false)]
    pub document_type: Option<Vec<Coding>>,
    /// e.g. Resource Type, Profile, etc
    #[fhir(name="resourceType", min="0", max="*", summary=true, modifier=false)]
    pub resource_type: Option<Vec<Coding>>,
    /// e.g. LOINC or SNOMED CT code, etc. in the content
    #[fhir(name="code", min="0", max="*", summary=true, modifier=false)]
    pub code: Option<Vec<CodeableConcept>>,
    /// Timeframe for data controlled by this provision
    #[fhir(name="dataPeriod", min="0", max="1", summary=true, modifier=false)]
    pub data_period: Option<Period>,
    /// Data controlled by this provision
    #[fhir(name="data", min="0", max="*", summary=true, modifier=false)]
    pub data: Option<Vec<ConsentProvisionDataBackboneElement>>,
    /// A computable expression of the consent
    #[fhir(name="expression", min="0", max="1", summary=false, modifier=false)]
    pub expression: Option<Expression>,
    /// Nested Exception Provisions
    #[fhir(name="provision", min="0", max="*", summary=false, modifier=false)]
    pub provision: Option<Vec<ConsentProvisionBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConsentProvisionActorBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// How the actor is involved
    #[fhir(name="role", min="0", max="1", summary=false, modifier=false)]
    pub role: Option<CodeableConcept>,
    /// Resource for the actor (or group, by role)
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false)]
    pub reference: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConsentProvisionDataBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// instance | related | dependents | authoredby
    #[fhir(name="meaning", min="1", max="1", summary=true, modifier=false)]
    pub meaning: Option<CodeDt>,
    /// The actual data reference
    #[fhir(name="reference", min="1", max="1", summary=true, modifier=false)]
    pub reference: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConsentVerificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Has been verified
    #[fhir(name="verified", min="1", max="1", summary=true, modifier=false)]
    pub verified: Option<BooleanDt>,
    /// Business case of verification
    #[fhir(name="verificationType", min="0", max="1", summary=false, modifier=false)]
    pub verification_type: Option<CodeableConcept>,
    /// Person conducting verification
    #[fhir(name="verifiedBy", min="0", max="1", summary=false, modifier=false)]
    pub verified_by: Option<Reference>,
    /// Person who verified
    #[fhir(name="verifiedWith", min="0", max="1", summary=false, modifier=false)]
    pub verified_with: Option<Reference>,
    /// When consent verified
    #[fhir(name="verificationDate", min="0", max="*", summary=false, modifier=false)]
    pub verification_date: Option<Vec<DateTimeDt>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct ConsentPolicyBasisBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Reference backing policy resource
    #[fhir(name="reference", min="0", max="1", summary=false, modifier=false)]
    pub reference: Option<Reference>,
    /// URL to a computable backing policy
    #[fhir(name="url", min="0", max="1", summary=false, modifier=false)]
    pub url: Option<UrlDt>,
}


use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Permission {
    /// Logical id of this artifact
    #[fhir(name="id", min="0", max="1", summary="true", modifier="false")]
    pub id: Option<Id>,
    /// Metadata about the resource
    #[fhir(name="meta", min="0", max="1", summary="true", modifier="false")]
    pub meta: Option<Meta>,
    /// A set of rules under which this content was created
    #[fhir(name="implicitRules", min="0", max="1", summary="true", modifier="true")]
    pub implicit_rules: Option<UriDt>,
    /// Language of the resource content
    #[fhir(name="language", min="0", max="1", summary="false", modifier="false")]
    pub language: Option<CodeDt>,
    /// Text summary of the resource, for human interpretation
    #[fhir(name="text", min="0", max="1", summary="false", modifier="false")]
    pub text: Option<Narrative>,
    /// Contained, inline Resources
    #[fhir(name="contained", min="0", max="*", summary="false", modifier="false")]
    pub contained: Option<Vec<AnyResource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// active | entered-in-error | draft | rejected
    #[fhir(name="status", min="1", max="1", summary="true", modifier="false")]
    pub status: Option<CodeDt>,
    /// The person or entity that asserts the permission
    #[fhir(name="asserter", min="0", max="1", summary="true", modifier="false")]
    pub asserter: Option<Reference>,
    /// The date that permission was asserted
    #[fhir(name="date", min="0", max="*", summary="true", modifier="false")]
    pub date: Option<Vec<DateTimeDt>>,
    /// The period in which the permission is active
    #[fhir(name="validity", min="0", max="1", summary="true", modifier="false")]
    pub validity: Option<Period>,
    /// The asserted justification for using the data
    #[fhir(name="justification", min="0", max="1", summary="true", modifier="false")]
    pub justification: Option<PermissionJustificationBackboneElement>,
    /// deny-overrides | permit-overrides | ordered-deny-overrides | ordered-permit-overrides | deny-unless-permit | permit-unless-deny
    #[fhir(name="combining", min="1", max="1", summary="true", modifier="true")]
    pub combining: Option<CodeDt>,
    /// Constraints to the Permission
    #[fhir(name="rule", min="0", max="*", summary="true", modifier="false")]
    pub rule: Option<Vec<PermissionRuleBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PermissionJustificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The regulatory grounds upon which this Permission builds
    #[fhir(name="basis", min="0", max="*", summary="true", modifier="false")]
    pub basis: Option<Vec<CodeableConcept>>,
    /// Justifing rational
    #[fhir(name="evidence", min="0", max="*", summary="true", modifier="false")]
    pub evidence: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PermissionRuleBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// deny | permit
    #[fhir(name="type", min="0", max="1", summary="true", modifier="true")]
    pub type_: Option<CodeDt>,
    /// The selection criteria to identify data that is within scope of this provision
    #[fhir(name="data", min="0", max="*", summary="true", modifier="false")]
    pub data: Option<Vec<PermissionRuleDataBackboneElement>>,
    /// A description or definition of which activities are allowed to be done on the data
    #[fhir(name="activity", min="0", max="*", summary="true", modifier="false")]
    pub activity: Option<Vec<PermissionRuleActivityBackboneElement>>,
    /// What limits apply to the use of the data
    #[fhir(name="limit", min="0", max="*", summary="true", modifier="false")]
    pub limit: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PermissionRuleActivityBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Authorized actor(s)
    #[fhir(name="actor", min="0", max="*", summary="true", modifier="false")]
    pub actor: Option<Vec<Reference>>,
    /// Actions controlled by this rule
    #[fhir(name="action", min="0", max="*", summary="true", modifier="false")]
    pub action: Option<Vec<CodeableConcept>>,
    /// The purpose for which the permission is given
    #[fhir(name="purpose", min="0", max="*", summary="true", modifier="false")]
    pub purpose: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PermissionRuleDataBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Explicit FHIR Resource references
    #[fhir(name="resource", min="0", max="*", summary="true", modifier="false")]
    pub resource: Option<Vec<PermissionRuleDataResourceBackboneElement>>,
    /// Security tag code on .meta.security
    #[fhir(name="security", min="0", max="*", summary="true", modifier="false")]
    pub security: Option<Vec<Coding>>,
    /// Timeframe encompasing data create/update
    #[fhir(name="period", min="0", max="*", summary="true", modifier="false")]
    pub period: Option<Vec<Period>>,
    /// Expression identifying the data
    #[fhir(name="expression", min="0", max="1", summary="true", modifier="false")]
    pub expression: Option<Expression>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct PermissionRuleDataResourceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// instance | related | dependents | authoredby
    #[fhir(name="meaning", min="1", max="1", summary="true", modifier="false")]
    pub meaning: Option<CodeDt>,
    /// The actual data reference
    #[fhir(name="reference", min="1", max="1", summary="true", modifier="false")]
    pub reference: Option<Reference>,
}


use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct RegulatedAuthorization {
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
    /// Business identifier for the authorization, typically assigned by the authorizing body
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// The product type, treatment, facility or activity that is being authorized
    #[fhir(name="subject", min="0", max="*", summary="true", modifier="false")]
    pub subject: Option<Vec<Reference>>,
    /// Overall type of this authorization, for example drug marketing approval, orphan drug designation
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// General textual supporting information
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// The territory in which the authorization has been granted
    #[fhir(name="region", min="0", max="*", summary="true", modifier="false")]
    pub region: Option<Vec<CodeableConcept>>,
    /// The status that is authorised e.g. approved. Intermediate states can be tracked with cases and applications
    #[fhir(name="status", min="0", max="1", summary="true", modifier="false")]
    pub status: Option<CodeableConcept>,
    /// The date at which the current status was assigned
    #[fhir(name="statusDate", min="0", max="1", summary="true", modifier="false")]
    pub status_date: Option<DateTimeDt>,
    /// The time period in which the regulatory approval etc. is in effect, e.g. a Marketing Authorization includes the date of authorization and/or expiration date
    #[fhir(name="validityPeriod", min="0", max="1", summary="true", modifier="false")]
    pub validity_period: Option<Period>,
    /// Condition for which the use of the regulated product applies
    #[fhir(name="indication", min="0", max="*", summary="true", modifier="false")]
    pub indication: Option<Vec<CodeableReference>>,
    /// The intended use of the product, e.g. prevention, treatment
    #[fhir(name="intendedUse", min="0", max="1", summary="true", modifier="false")]
    pub intended_use: Option<CodeableConcept>,
    /// The legal/regulatory framework or reasons under which this authorization is granted
    #[fhir(name="basis", min="0", max="*", summary="true", modifier="false")]
    pub basis: Option<Vec<CodeableConcept>>,
    /// The organization that has been granted this authorization, by the regulator
    #[fhir(name="holder", min="0", max="1", summary="true", modifier="false")]
    pub holder: Option<Reference>,
    /// The regulatory authority or authorizing body granting the authorization
    #[fhir(name="regulator", min="0", max="1", summary="true", modifier="false")]
    pub regulator: Option<Reference>,
    /// Additional information or supporting documentation about the authorization
    #[fhir(name="attachedDocument", min="0", max="*", summary="true", modifier="false")]
    pub attached_document: Option<Vec<Reference>>,
    /// The case or regulatory procedure for granting or amending a regulated authorization. Note: This area is subject to ongoing review and the workgroup is seeking implementer feedback on its use (see link at bottom of page)
    #[fhir(name="case", min="0", max="1", summary="true", modifier="false")]
    pub case: Option<RegulatedAuthorizationCaseBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct RegulatedAuthorizationCaseBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Identifier by which this case can be referenced
    #[fhir(name="identifier", min="0", max="1", summary="true", modifier="false")]
    pub identifier: Option<Identifier>,
    /// The defining type of case
    #[fhir(name="type", min="0", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeableConcept>,
    /// The status associated with the case
    #[fhir(name="status", min="0", max="1", summary="true", modifier="false")]
    pub status: Option<CodeableConcept>,
    /// Relevant date for this case
    #[fhir(name="date", min="0", max="1", summary="true", modifier="false")]
    pub date: Option<DateTimeDt>,
    /// Applications submitted to obtain a regulated authorization. Steps within the longer running case or procedure
    #[fhir(name="application", min="0", max="*", summary="true", modifier="false")]
    pub application: Option<Vec<RegulatedAuthorizationCaseBackboneElement>>,
}


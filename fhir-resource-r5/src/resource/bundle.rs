use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct Bundle {
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
    /// Persistent identifier for the bundle
    #[fhir(name="identifier", min="0", max="1", summary="true", modifier="false")]
    pub identifier: Option<Identifier>,
    /// document | message | transaction | transaction-response | batch | batch-response | history | searchset | collection | subscription-notification
    #[fhir(name="type", min="1", max="1", summary="true", modifier="false")]
    pub type_: Option<CodeDt>,
    /// When the bundle was assembled
    #[fhir(name="timestamp", min="0", max="1", summary="true", modifier="false")]
    pub timestamp: Option<InstantDt>,
    /// If search, the total number of matches
    #[fhir(name="total", min="0", max="1", summary="true", modifier="false")]
    pub total: Option<UnsignedIntDt>,
    /// Links related to this Bundle
    #[fhir(name="link", min="0", max="*", summary="true", modifier="false")]
    pub link: Option<Vec<BundleLinkBackboneElement>>,
    /// Entry in the bundle - will have a resource or information
    #[fhir(name="entry", min="0", max="*", summary="true", modifier="false")]
    pub entry: Option<Vec<BundleEntryBackboneElement>>,
    /// Digital Signature
    #[fhir(name="signature", min="0", max="1", summary="true", modifier="false")]
    pub signature: Option<Signature>,
    /// Issues with the Bundle
    #[fhir(name="issues", min="0", max="1", summary="true", modifier="false")]
    pub issues: Option<Resource>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BundleEntryBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Links related to this entry
    #[fhir(name="link", min="0", max="*", summary="true", modifier="false")]
    pub link: Option<Vec<BundleLinkBackboneElement>>,
    /// URI for resource (e.g. the absolute URL server address, URI for UUID/OID, etc.)
    #[fhir(name="fullUrl", min="0", max="1", summary="true", modifier="false")]
    pub full_url: Option<UriDt>,
    /// A resource in the bundle
    #[fhir(name="resource", min="0", max="1", summary="true", modifier="false")]
    pub resource: Option<Resource>,
    /// Search related information
    #[fhir(name="search", min="0", max="1", summary="true", modifier="false")]
    pub search: Option<BundleEntrySearchBackboneElement>,
    /// Additional execution information (transaction/batch/history)
    #[fhir(name="request", min="0", max="1", summary="true", modifier="false")]
    pub request: Option<BundleEntryRequestBackboneElement>,
    /// Results of execution (transaction/batch/history)
    #[fhir(name="response", min="0", max="1", summary="true", modifier="false")]
    pub response: Option<BundleEntryResponseBackboneElement>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BundleEntrySearchBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// match | include - why this is in the result set
    #[fhir(name="mode", min="0", max="1", summary="true", modifier="false")]
    pub mode: Option<CodeDt>,
    /// Search ranking (between 0 and 1)
    #[fhir(name="score", min="0", max="1", summary="true", modifier="false")]
    pub score: Option<DecimalDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BundleEntryRequestBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// GET | HEAD | POST | PUT | DELETE | PATCH
    #[fhir(name="method", min="1", max="1", summary="true", modifier="false")]
    pub method: Option<CodeDt>,
    /// URL for HTTP equivalent of this entry
    #[fhir(name="url", min="1", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
    /// For managing cache validation
    #[fhir(name="ifNoneMatch", min="0", max="1", summary="true", modifier="false")]
    pub if_none_match: Option<StringDt>,
    /// For managing cache currency
    #[fhir(name="ifModifiedSince", min="0", max="1", summary="true", modifier="false")]
    pub if_modified_since: Option<InstantDt>,
    /// For managing update contention
    #[fhir(name="ifMatch", min="0", max="1", summary="true", modifier="false")]
    pub if_match: Option<StringDt>,
    /// For conditional creates
    #[fhir(name="ifNoneExist", min="0", max="1", summary="true", modifier="false")]
    pub if_none_exist: Option<StringDt>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BundleEntryResponseBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Status response code (text optional)
    #[fhir(name="status", min="1", max="1", summary="true", modifier="false")]
    pub status: Option<StringDt>,
    /// The location (if the operation returns a location)
    #[fhir(name="location", min="0", max="1", summary="true", modifier="false")]
    pub location: Option<UriDt>,
    /// The Etag for the resource (if relevant)
    #[fhir(name="etag", min="0", max="1", summary="true", modifier="false")]
    pub etag: Option<StringDt>,
    /// Server's date time modified
    #[fhir(name="lastModified", min="0", max="1", summary="true", modifier="false")]
    pub last_modified: Option<InstantDt>,
    /// OperationOutcome with hints and warnings (for batch/transaction)
    #[fhir(name="outcome", min="0", max="1", summary="true", modifier="false")]
    pub outcome: Option<Resource>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BundleLinkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// See http://www.iana.org/assignments/link-relations/link-relations.xhtml#link-relations-1
    #[fhir(name="relation", min="1", max="1", summary="true", modifier="false")]
    pub relation: Option<CodeDt>,
    /// Reference details for the link
    #[fhir(name="url", min="1", max="1", summary="true", modifier="false")]
    pub url: Option<UriDt>,
}


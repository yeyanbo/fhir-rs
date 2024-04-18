use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct Location {
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
    /// Unique code or number identifying the location to its users
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false)]
    pub identifier: Option<Vec<Identifier>>,
    /// active | suspended | inactive
    #[fhir(name="status", min="0", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// The operational status of the location (typically only for a bed/room)
    #[fhir(name="operationalStatus", min="0", max="1", summary=true, modifier=false)]
    pub operational_status: Option<Coding>,
    /// Name of the location as used by humans
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// A list of alternate names that the location is known as, or was known as, in the past
    #[fhir(name="alias", min="0", max="*", summary=false, modifier=false)]
    pub alias: Option<Vec<StringDt>>,
    /// Additional details about the location that could be displayed as further information to identify the location beyond its name
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// instance | kind
    #[fhir(name="mode", min="0", max="1", summary=true, modifier=false)]
    pub mode: Option<CodeDt>,
    /// Type of function performed
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false)]
    pub type_: Option<Vec<CodeableConcept>>,
    /// Official contact details for the location
    #[fhir(name="contact", min="0", max="*", summary=false, modifier=false)]
    pub contact: Option<Vec<ExtendedContactDetail>>,
    /// Physical location
    #[fhir(name="address", min="0", max="1", summary=false, modifier=false)]
    pub address: Option<Address>,
    /// Physical form of the location
    #[fhir(name="form", min="0", max="1", summary=true, modifier=false)]
    pub form: Option<CodeableConcept>,
    /// The absolute geographic location
    #[fhir(name="position", min="0", max="1", summary=false, modifier=false)]
    pub position: Option<LocationPositionBackboneElement>,
    /// Organization responsible for provisioning and upkeep
    #[fhir(name="managingOrganization", min="0", max="1", summary=true, modifier=false)]
    pub managing_organization: Option<Reference>,
    /// Another Location this one is physically a part of
    #[fhir(name="partOf", min="0", max="1", summary=false, modifier=false)]
    pub part_of: Option<Reference>,
    /// Collection of characteristics (attributes)
    #[fhir(name="characteristic", min="0", max="*", summary=false, modifier=false)]
    pub characteristic: Option<Vec<CodeableConcept>>,
    /// What days/times during a week is this location usually open (including exceptions)
    #[fhir(name="hoursOfOperation", min="0", max="*", summary=false, modifier=false)]
    pub hours_of_operation: Option<Vec<Availability>>,
    /// Connection details of a virtual service (e.g. conference call)
    #[fhir(name="virtualService", min="0", max="*", summary=false, modifier=false)]
    pub virtual_service: Option<Vec<VirtualServiceDetail>>,
    /// Technical endpoints providing access to services operated for the location
    #[fhir(name="endpoint", min="0", max="*", summary=false, modifier=false)]
    pub endpoint: Option<Vec<Reference>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct LocationPositionBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Longitude with WGS84 datum
    #[fhir(name="longitude", min="1", max="1", summary=false, modifier=false)]
    pub longitude: Option<DecimalDt>,
    /// Latitude with WGS84 datum
    #[fhir(name="latitude", min="1", max="1", summary=false, modifier=false)]
    pub latitude: Option<DecimalDt>,
    /// Altitude with WGS84 datum
    #[fhir(name="altitude", min="0", max="1", summary=false, modifier=false)]
    pub altitude: Option<DecimalDt>,
}


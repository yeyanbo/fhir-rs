use fhir_derive::Element;

use crate::prelude::*;

#[derive(Complex, Debug, Clone, Default)]
pub struct Coding {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Identity of the terminology system
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Version of the system - if relevant
    #[fhir(name="version", min="0", max="1", summary=true, modifier=false)]
    pub version: Option<StringDt>,
    /// Symbol in syntax defined by the system
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
    /// Representation defined by the system
    #[fhir(name="display", min="0", max="1", summary=true, modifier=false)]
    pub display: Option<StringDt>,
    /// If this coding was chosen directly by the user
    #[fhir(name="userSelected", min="0", max="1", summary=true, modifier=false)]
    pub user_selected: Option<BooleanDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Address {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// home | work | temp | old | billing - purpose of this address
    #[fhir(name="use", min="0", max="1", summary=true, modifier=true)]
    pub use_: Option<CodeDt>,
    /// postal | physical | both
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Text representation of the address
    #[fhir(name="text", min="0", max="1", summary=true, modifier=false)]
    pub text: Option<StringDt>,
    /// Street name, number, direction & P.O. Box etc.
    #[fhir(name="line", min="0", max="*", summary=true, modifier=false)]
    pub line: Option<Vec<StringDt>>,
    /// Name of city, town etc.
    #[fhir(name="city", min="0", max="1", summary=true, modifier=false)]
    pub city: Option<StringDt>,
    /// District name (aka county)
    #[fhir(name="district", min="0", max="1", summary=true, modifier=false)]
    pub district: Option<StringDt>,
    /// Sub-unit of country (abbreviations ok)
    #[fhir(name="state", min="0", max="1", summary=true, modifier=false)]
    pub state: Option<StringDt>,
    /// Postal code for area
    #[fhir(name="postalCode", min="0", max="1", summary=true, modifier=false)]
    pub postal_code: Option<StringDt>,
    /// Country (e.g. may be ISO 3166 2 or 3 letter code)
    #[fhir(name="country", min="0", max="1", summary=true, modifier=false)]
    pub country: Option<StringDt>,
    /// Time period when address was/is in use
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Age {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Annotation {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Individual responsible for the annotation
    #[fhir(name="author", min="0", max="1", summary=true, modifier=false)]
    pub author: Option<StringDt>,
    /// When the annotation was made
    #[fhir(name="time", min="0", max="1", summary=true, modifier=false)]
    pub time: Option<DateTimeDt>,
    /// The annotation  - text content (as markdown)
    #[fhir(name="text", min="1", max="1", summary=true, modifier=false)]
    pub text: Option<MarkdownDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Attachment {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Mime type of the content, with charset etc.
    #[fhir(name="contentType", min="0", max="1", summary=true, modifier=false)]
    pub content_type: Option<CodeDt>,
    /// Human language of the content (BCP-47)
    #[fhir(name="language", min="0", max="1", summary=true, modifier=false)]
    pub language: Option<CodeDt>,
    /// Data inline, base64ed
    #[fhir(name="data", min="0", max="1", summary=false, modifier=false)]
    pub data: Option<Base64BinaryDt>,
    /// Uri where the data can be found
    #[fhir(name="url", min="0", max="1", summary=true, modifier=false)]
    pub url: Option<UrlDt>,
    /// Number of bytes of content (if url provided)
    #[fhir(name="size", min="0", max="1", summary=true, modifier=false)]
    pub size: Option<Integer64Dt>,
    /// Hash of the data (sha-1, base64ed)
    #[fhir(name="hash", min="0", max="1", summary=true, modifier=false)]
    pub hash: Option<Base64BinaryDt>,
    /// Label to display in place of the data
    #[fhir(name="title", min="0", max="1", summary=true, modifier=false)]
    pub title: Option<StringDt>,
    /// Date attachment was first created
    #[fhir(name="creation", min="0", max="1", summary=true, modifier=false)]
    pub creation: Option<DateTimeDt>,
    /// Height of the image in pixels (photo/video)
    #[fhir(name="height", min="0", max="1", summary=false, modifier=false)]
    pub height: Option<PositiveIntDt>,
    /// Width of the image in pixels (photo/video)
    #[fhir(name="width", min="0", max="1", summary=false, modifier=false)]
    pub width: Option<PositiveIntDt>,
    /// Number of frames if > 1 (photo)
    #[fhir(name="frames", min="0", max="1", summary=false, modifier=false)]
    pub frames: Option<PositiveIntDt>,
    /// Length in seconds (audio / video)
    #[fhir(name="duration", min="0", max="1", summary=false, modifier=false)]
    pub duration: Option<DecimalDt>,
    /// Number of printed pages
    #[fhir(name="pages", min="0", max="1", summary=false, modifier=false)]
    pub pages: Option<PositiveIntDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Availability {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Times the {item} is available
    #[fhir(name="availableTime", min="0", max="*", summary=true, modifier=false)]
    pub available_time: Option<Vec<AvailabilityAvailableTimeElement>>,
    /// Not available during this time due to provided reason
    #[fhir(name="notAvailableTime", min="0", max="*", summary=true, modifier=false)]
    pub not_available_time: Option<Vec<AvailabilityNotAvailableTimeElement>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct AvailabilityAvailableTimeElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// mon | tue | wed | thu | fri | sat | sun
    #[fhir(name="daysOfWeek", min="0", max="*", summary=true, modifier=false)]
    pub days_of_week: Option<Vec<CodeDt>>,
    /// Always available? i.e. 24 hour service
    #[fhir(name="allDay", min="0", max="1", summary=true, modifier=false)]
    pub all_day: Option<BooleanDt>,
    /// Opening time of day (ignored if allDay = true)
    #[fhir(name="availableStartTime", min="0", max="1", summary=true, modifier=false)]
    pub available_start_time: Option<TimeDt>,
    /// Closing time of day (ignored if allDay = true)
    #[fhir(name="availableEndTime", min="0", max="1", summary=true, modifier=false)]
    pub available_end_time: Option<TimeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct AvailabilityNotAvailableTimeElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Reason presented to the user explaining why time not available
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<StringDt>,
    /// Service not available during this period
    #[fhir(name="during", min="0", max="1", summary=true, modifier=false)]
    pub during: Option<Period>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct BackboneType {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct CodeableConcept {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Code defined by a terminology system
    #[fhir(name="coding", min="0", max="*", summary=true, modifier=false)]
    pub coding: Option<Vec<Coding>>,
    /// Plain text representation of the concept
    #[fhir(name="text", min="0", max="1", summary=true, modifier=false)]
    pub text: Option<StringDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct CodeableReference {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Reference to a concept (by class)
    #[fhir(name="concept", min="0", max="1", summary=true, modifier=false)]
    pub concept: Option<CodeableConcept>,
    /// Reference to a resource (by instance)
    #[fhir(name="reference", min="0", max="1", summary=true, modifier=false)]
    pub reference: Option<Reference>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct ContactDetail {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Name of an individual to contact
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Contact details for individual or organization
    #[fhir(name="telecom", min="0", max="*", summary=true, modifier=false)]
    pub telecom: Option<Vec<ContactPoint>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct ContactPoint {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// phone | fax | email | pager | url | sms | other
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<CodeDt>,
    /// The actual contact point details
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<StringDt>,
    /// home | work | temp | old | mobile - purpose of this contact point
    #[fhir(name="use", min="0", max="1", summary=true, modifier=true)]
    pub use_: Option<CodeDt>,
    /// Specify preferred order of use (1 = highest)
    #[fhir(name="rank", min="0", max="1", summary=true, modifier=false)]
    pub rank: Option<PositiveIntDt>,
    /// Time period when the contact point was/is in use
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Contributor {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// author | editor | reviewer | endorser
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Who contributed the content
    #[fhir(name="name", min="1", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Contact details of the contributor
    #[fhir(name="contact", min="0", max="*", summary=true, modifier=false)]
    pub contact: Option<Vec<ContactDetail>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Count {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct DataRequirement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// The type of the required data
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// The profile of the required data
    #[fhir(name="profile", min="0", max="*", summary=true, modifier=false)]
    pub profile: Option<Vec<CanonicalDt>>,
    /// E.g. Patient, Practitioner, RelatedPerson, Organization, Location, Device
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false)]
    pub subject: Option<Reference>,
    /// Indicates specific structure elements that are referenced by the knowledge module
    #[fhir(name="mustSupport", min="0", max="*", summary=true, modifier=false)]
    pub must_support: Option<Vec<StringDt>>,
    /// What codes are expected
    #[fhir(name="codeFilter", min="0", max="*", summary=true, modifier=false)]
    pub code_filter: Option<Vec<DataRequirementCodeFilterElement>>,
    /// What dates/date ranges are expected
    #[fhir(name="dateFilter", min="0", max="*", summary=true, modifier=false)]
    pub date_filter: Option<Vec<DataRequirementDateFilterElement>>,
    /// What values are expected
    #[fhir(name="valueFilter", min="0", max="*", summary=true, modifier=false)]
    pub value_filter: Option<Vec<DataRequirementValueFilterElement>>,
    /// Number of results
    #[fhir(name="limit", min="0", max="1", summary=true, modifier=false)]
    pub limit: Option<PositiveIntDt>,
    /// Order of the results
    #[fhir(name="sort", min="0", max="*", summary=true, modifier=false)]
    pub sort: Option<Vec<DataRequirementSortElement>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct DataRequirementSortElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// The name of the attribute to perform the sort
    #[fhir(name="path", min="1", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
    /// ascending | descending
    #[fhir(name="direction", min="1", max="1", summary=true, modifier=false)]
    pub direction: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct DataRequirementCodeFilterElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// A code-valued attribute to filter on
    #[fhir(name="path", min="0", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
    /// A coded (token) parameter to search on
    #[fhir(name="searchParam", min="0", max="1", summary=true, modifier=false)]
    pub search_param: Option<StringDt>,
    /// ValueSet for the filter
    #[fhir(name="valueSet", min="0", max="1", summary=true, modifier=false)]
    pub value_set: Option<CanonicalDt>,
    /// What code is expected
    #[fhir(name="code", min="0", max="*", summary=true, modifier=false)]
    pub code: Option<Vec<Coding>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct DataRequirementValueFilterElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// An attribute to filter on
    #[fhir(name="path", min="0", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
    /// A parameter to search on
    #[fhir(name="searchParam", min="0", max="1", summary=true, modifier=false)]
    pub search_param: Option<StringDt>,
    /// eq | gt | lt | ge | le | sa | eb
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=false)]
    pub comparator: Option<CodeDt>,
    /// The value of the filter, as a Period, DateTime, or Duration value
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<Duration>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct DataRequirementDateFilterElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// A date-valued attribute to filter on
    #[fhir(name="path", min="0", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
    /// A date valued parameter to search on
    #[fhir(name="searchParam", min="0", max="1", summary=true, modifier=false)]
    pub search_param: Option<StringDt>,
    /// The value of the filter, as a Period, DateTime, or Duration value
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<Duration>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Complex {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Distance {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Dosage {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The order of the dosage instructions
    #[fhir(name="sequence", min="0", max="1", summary=true, modifier=false)]
    pub sequence: Option<IntegerDt>,
    /// Free text dosage instructions e.g. SIG
    #[fhir(name="text", min="0", max="1", summary=true, modifier=false)]
    pub text: Option<StringDt>,
    /// Supplemental instruction or warnings to the patient - e.g. "with meals", "may cause drowsiness"
    #[fhir(name="additionalInstruction", min="0", max="*", summary=true, modifier=false)]
    pub additional_instruction: Option<Vec<CodeableConcept>>,
    /// Patient or consumer oriented instructions
    #[fhir(name="patientInstruction", min="0", max="1", summary=true, modifier=false)]
    pub patient_instruction: Option<StringDt>,
    /// When medication should be administered
    #[fhir(name="timing", min="0", max="1", summary=true, modifier=false)]
    pub timing: Option<Timing>,
    /// Take "as needed"
    #[fhir(name="asNeeded", min="0", max="1", summary=true, modifier=false)]
    pub as_needed: Option<BooleanDt>,
    /// Take "as needed" (for x)
    #[fhir(name="asNeededFor", min="0", max="*", summary=true, modifier=false)]
    pub as_needed_for: Option<Vec<CodeableConcept>>,
    /// Body site to administer to
    #[fhir(name="site", min="0", max="1", summary=true, modifier=false)]
    pub site: Option<CodeableConcept>,
    /// How drug should enter body
    #[fhir(name="route", min="0", max="1", summary=true, modifier=false)]
    pub route: Option<CodeableConcept>,
    /// Technique for administering medication
    #[fhir(name="method", min="0", max="1", summary=true, modifier=false)]
    pub method: Option<CodeableConcept>,
    /// Amount of medication administered, to be administered or typical amount to be administered
    #[fhir(name="doseAndRate", min="0", max="*", summary=true, modifier=false)]
    pub dose_and_rate: Option<Vec<DosageDoseAndRateElement>>,
    /// Upper limit on medication per unit of time
    #[fhir(name="maxDosePerPeriod", min="0", max="*", summary=true, modifier=false)]
    pub max_dose_per_period: Option<Vec<Ratio>>,
    /// Upper limit on medication per administration
    #[fhir(name="maxDosePerAdministration", min="0", max="1", summary=true, modifier=false)]
    pub max_dose_per_administration: Option<Quantity>,
    /// Upper limit on medication per lifetime of the patient
    #[fhir(name="maxDosePerLifetime", min="0", max="1", summary=true, modifier=false)]
    pub max_dose_per_lifetime: Option<Quantity>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct DosageDoseAndRateElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// The kind of dose or rate specified
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// Amount of medication per dose
    #[fhir(name="dose", min="0", max="1", summary=true, modifier=false)]
    pub dose: Option<Quantity>,
    /// Amount of medication per unit of time
    #[fhir(name="rate", min="0", max="1", summary=true, modifier=false)]
    pub rate: Option<Quantity>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Duration {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct ElementDefinition {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Path of the element in the hierarchy of elements
    #[fhir(name="path", min="1", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
    /// xmlAttr | xmlText | typeAttr | cdaText | xhtml
    #[fhir(name="representation", min="0", max="*", summary=true, modifier=false)]
    pub representation: Option<Vec<CodeDt>>,
    /// Name for this particular element (in a set of slices)
    #[fhir(name="sliceName", min="0", max="1", summary=true, modifier=false)]
    pub slice_name: Option<StringDt>,
    /// If this slice definition constrains an inherited slice definition (or not)
    #[fhir(name="sliceIsConstraining", min="0", max="1", summary=true, modifier=false)]
    pub slice_is_constraining: Option<BooleanDt>,
    /// Name for element to display with or prompt for element
    #[fhir(name="label", min="0", max="1", summary=true, modifier=false)]
    pub label: Option<StringDt>,
    /// Corresponding codes in terminologies
    #[fhir(name="code", min="0", max="*", summary=true, modifier=false)]
    pub code: Option<Vec<Coding>>,
    /// This element is sliced - slices follow
    #[fhir(name="slicing", min="0", max="1", summary=true, modifier=false)]
    pub slicing: Option<ElementDefinitionSlicingElement>,
    /// Concise definition for space-constrained presentation
    #[fhir(name="short", min="0", max="1", summary=true, modifier=false)]
    pub short: Option<StringDt>,
    /// Full formal definition as narrative text
    #[fhir(name="definition", min="0", max="1", summary=true, modifier=false)]
    pub definition: Option<MarkdownDt>,
    /// Comments about the use of this element
    #[fhir(name="comment", min="0", max="1", summary=true, modifier=false)]
    pub comment: Option<MarkdownDt>,
    /// Why this resource has been created
    #[fhir(name="requirements", min="0", max="1", summary=true, modifier=false)]
    pub requirements: Option<MarkdownDt>,
    /// Other names
    #[fhir(name="alias", min="0", max="*", summary=true, modifier=false)]
    pub alias: Option<Vec<StringDt>>,
    /// Minimum Cardinality
    #[fhir(name="min", min="0", max="1", summary=true, modifier=false)]
    pub min: Option<UnsignedIntDt>,
    /// Maximum Cardinality (a number or *)
    #[fhir(name="max", min="0", max="1", summary=true, modifier=false)]
    pub max: Option<StringDt>,
    // Base definition information for tools
    #[fhir(name="base", min="0", max="1", summary=true, modifier=false)]
    pub base: Option<ElementDefinitionBaseElement>,
    /// Reference to definition of content for the element
    #[fhir(name="contentReference", min="0", max="1", summary=true, modifier=false)]
    pub content_reference: Option<UriDt>,
    // Data type and Profile for this element
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false)]
    pub type_: Option<Vec<ElementDefinitionTypeElement>>,
    /// Specified value if missing from instance
    #[fhir(name="defaultValue", min="0", max="1", summary=true, modifier=false)]
    pub default_value: Option<Meta>,
    /// Implicit meaning when this element is missing
    #[fhir(name="meaningWhenMissing", min="0", max="1", summary=true, modifier=false)]
    pub meaning_when_missing: Option<MarkdownDt>,
    /// What the order of the elements means
    #[fhir(name="orderMeaning", min="0", max="1", summary=true, modifier=false)]
    pub order_meaning: Option<StringDt>,
    /// Value must be exactly this
    #[fhir(name="fixed", min="0", max="1", summary=true, modifier=false)]
    pub fixed: Option<Meta>,
    /// Value must have at least these property values
    #[fhir(name="pattern", min="0", max="1", summary=true, modifier=false)]
    pub pattern: Option<Meta>,
    /// Example value (as defined for type)
    #[fhir(name="example", min="0", max="*", summary=true, modifier=false)]
    pub example: Option<Vec<ElementDefinitionExampleElement>>,
    /// Minimum Allowed Value (for some types)
    #[fhir(name="minValue", min="0", max="1", summary=true, modifier=false)]
    pub min_value: Option<Quantity>,
    /// Maximum Allowed Value (for some types)
    #[fhir(name="maxValue", min="0", max="1", summary=true, modifier=false)]
    pub max_value: Option<Quantity>,
    /// Max length for string type data
    #[fhir(name="maxLength", min="0", max="1", summary=true, modifier=false)]
    pub max_length: Option<IntegerDt>,
    /// Reference to invariant about presence
    #[fhir(name="condition", min="0", max="*", summary=true, modifier=false)]
    pub condition: Option<Vec<IdDt>>,
    /// Condition that must evaluate to true
    #[fhir(name="constraint", min="0", max="*", summary=true, modifier=false)]
    pub constraint: Option<Vec<ElementDefinitionConstraintElement>>,
    /// For primitives, that a value must be present - not replaced by an extension
    #[fhir(name="mustHaveValue", min="0", max="1", summary=true, modifier=false)]
    pub must_have_value: Option<BooleanDt>,
    /// Extensions that are allowed to replace a primitive value
    #[fhir(name="valueAlternatives", min="0", max="*", summary=true, modifier=false)]
    pub value_alternatives: Option<Vec<CanonicalDt>>,
    /// If the element must be supported (discouraged - see obligations)
    #[fhir(name="mustSupport", min="0", max="1", summary=true, modifier=false)]
    pub must_support: Option<BooleanDt>,
    /// If this modifies the meaning of other elements
    #[fhir(name="isModifier", min="0", max="1", summary=true, modifier=false)]
    pub is_modifier: Option<BooleanDt>,
    /// Reason that this element is marked as a modifier
    #[fhir(name="isModifierReason", min="0", max="1", summary=true, modifier=false)]
    pub is_modifier_reason: Option<StringDt>,
    /// Include when _summary = true?
    #[fhir(name="isSummary", min="0", max="1", summary=true, modifier=false)]
    pub is_summary: Option<BooleanDt>,
    /// ValueSet details if this is coded
    #[fhir(name="binding", min="0", max="1", summary=true, modifier=false)]
    pub binding: Option<ElementDefinitionBindingElement>,
    // /// Map element to another set of definitions
    // #[fhir(name="mapping", min="0", max="*", summary=true, modifier=false)]
    // pub mapping: Option<Vec<ElementDefinitionMappingElement>>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionBaseElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Path that identifies the base element
    #[fhir(name="path", min="1", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
    /// Min cardinality of the base element
    #[fhir(name="min", min="1", max="1", summary=true, modifier=false)]
    pub min: Option<UnsignedIntDt>,
    /// Max cardinality of the base element
    #[fhir(name="max", min="1", max="1", summary=true, modifier=false)]
    pub max: Option<StringDt>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionSlicingElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Element values that are used to distinguish the slices
    #[fhir(name="discriminator", min="0", max="*", summary=true, modifier=false)]
    pub discriminator: Option<Vec<ElementDefinitionSlicingDiscriminatorElement>>,
    /// Text description of how slicing works (or not)
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<StringDt>,
    /// If elements must be in same order as slices
    #[fhir(name="ordered", min="0", max="1", summary=true, modifier=false)]
    pub ordered: Option<BooleanDt>,
    /// closed | open | openAtEnd
    #[fhir(name="rules", min="1", max="1", summary=true, modifier=false)]
    pub rules: Option<CodeDt>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionSlicingDiscriminatorElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// value | exists | type | profile | position
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Path to element value
    #[fhir(name="path", min="1", max="1", summary=true, modifier=false)]
    pub path: Option<StringDt>,
}

// #[derive(Element, Debug, Clone, Default)]
// pub struct ElementDefinitionMappingElement {
//     /// Unique id for inter-element referencing
//     #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
//     pub id: Option<String>,
//     /// Additional content defined by implementations
//     #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
//     pub extension: Option<Vec<Extension>>,
//     /// Reference to mapping declaration
//     #[fhir(name="identity", min="1", max="1", summary=true, modifier=false)]
//     pub identity: Option<IdDt>,
//     /// Computable language of mapping
//     #[fhir(name="language", min="0", max="1", summary=true, modifier=false)]
//     pub language: Option<CodeDt>,
//     /// Details of the mapping
//     #[fhir(name="map", min="1", max="1", summary=true, modifier=false)]
//     pub map: Option<StringDt>,
//     /// Comments about the mapping or its use
//     #[fhir(name="comment", min="0", max="1", summary=true, modifier=false)]
//     pub comment: Option<MarkdownDt>,
// }

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionTypeElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Data type or Resource (reference to definition)
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<UriDt>,
    /// Profiles (StructureDefinition or IG) - one must apply
    #[fhir(name="profile", min="0", max="*", summary=true, modifier=false)]
    pub profile: Option<Vec<CanonicalDt>>,
    /// Profile (StructureDefinition or IG) on the Reference/canonical target - one must apply
    #[fhir(name="targetProfile", min="0", max="*", summary=true, modifier=false)]
    pub target_profile: Option<Vec<CanonicalDt>>,
    /// contained | referenced | bundled - how aggregated
    #[fhir(name="aggregation", min="0", max="*", summary=true, modifier=false)]
    pub aggregation: Option<Vec<CodeDt>>,
    /// either | independent | specific
    #[fhir(name="versioning", min="0", max="1", summary=true, modifier=false)]
    pub versioning: Option<CodeDt>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionBindingElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// required | extensible | preferred | example
    #[fhir(name="strength", min="1", max="1", summary=true, modifier=false)]
    pub strength: Option<CodeDt>,
    /// Intended use of codes in the bound value set
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<MarkdownDt>,
    /// Source of value set
    #[fhir(name="valueSet", min="0", max="1", summary=true, modifier=false)]
    pub value_set: Option<CanonicalDt>,
    /// Additional Bindings - more rules about the binding
    #[fhir(name="additional", min="0", max="*", summary=true, modifier=false)]
    pub additional: Option<Vec<ElementDefinitionBindingAdditionalElement>>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionBindingAdditionalElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// maximum | minimum | required | extensible | candidate | current | preferred | ui | starter | component
    #[fhir(name="purpose", min="1", max="1", summary=true, modifier=false)]
    pub purpose: Option<CodeDt>,
    /// The value set for the additional binding
    #[fhir(name="valueSet", min="1", max="1", summary=true, modifier=false)]
    pub value_set: Option<CanonicalDt>,
    /// Documentation of the purpose of use of the binding
    #[fhir(name="documentation", min="0", max="1", summary=true, modifier=false)]
    pub documentation: Option<MarkdownDt>,
    /// Concise documentation - for summary tables
    #[fhir(name="shortDoco", min="0", max="1", summary=true, modifier=false)]
    pub short_doco: Option<StringDt>,
    /// Qualifies the usage - jurisdiction, gender, workflow status etc.
    #[fhir(name="usage", min="0", max="*", summary=true, modifier=false)]
    pub usage: Option<Vec<UsageContext>>,
    /// Whether binding can applies to all repeats, or just one
    #[fhir(name="any", min="0", max="1", summary=true, modifier=false)]
    pub any: Option<BooleanDt>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionExampleElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Describes the purpose of this example
    #[fhir(name="label", min="1", max="1", summary=true, modifier=false)]
    pub label: Option<StringDt>,
    /// Value of Example (one of allowed types)
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false)]
    pub value: Option<Meta>,
}

#[derive(Element, Debug, Clone, Default)]
pub struct ElementDefinitionConstraintElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Target of 'condition' reference above
    #[fhir(name="key", min="1", max="1", summary=true, modifier=false)]
    pub key: Option<IdDt>,
    /// Why this constraint is necessary or appropriate
    #[fhir(name="requirements", min="0", max="1", summary=true, modifier=false)]
    pub requirements: Option<MarkdownDt>,
    /// error | warning
    #[fhir(name="severity", min="1", max="1", summary=true, modifier=false)]
    pub severity: Option<CodeDt>,
    /// Suppress warning or hint in profile
    #[fhir(name="suppress", min="0", max="1", summary=true, modifier=false)]
    pub suppress: Option<BooleanDt>,
    /// Human description of constraint
    #[fhir(name="human", min="1", max="1", summary=true, modifier=false)]
    pub human: Option<StringDt>,
    /// FHIRPath expression of constraint
    #[fhir(name="expression", min="0", max="1", summary=true, modifier=false)]
    pub expression: Option<StringDt>,
    /// Reference to original source of constraint
    #[fhir(name="source", min="0", max="1", summary=true, modifier=false)]
    pub source: Option<CanonicalDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Expression {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Natural language description of the condition
    #[fhir(name="description", min="0", max="1", summary=true, modifier=false)]
    pub description: Option<StringDt>,
    /// Short name assigned to expression for reuse
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<CodeDt>,
    /// text/cql | text/fhirpath | application/x-fhir-query | etc.
    #[fhir(name="language", min="0", max="1", summary=true, modifier=false)]
    pub language: Option<CodeDt>,
    /// Expression in specified language
    #[fhir(name="expression", min="0", max="1", summary=true, modifier=false)]
    pub expression: Option<StringDt>,
    /// Where the expression is found
    #[fhir(name="reference", min="0", max="1", summary=true, modifier=false)]
    pub reference: Option<UriDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct ExtendedContactDetail {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// The type of contact
    #[fhir(name="purpose", min="0", max="1", summary=true, modifier=false)]
    pub purpose: Option<CodeableConcept>,
    /// Name of an individual to contact
    #[fhir(name="name", min="0", max="*", summary=true, modifier=false)]
    pub name: Option<Vec<HumanName>>,
    /// Contact details (e.g.phone/fax/url)
    #[fhir(name="telecom", min="0", max="*", summary=true, modifier=false)]
    pub telecom: Option<Vec<ContactPoint>>,
    /// Address for the contact
    #[fhir(name="address", min="0", max="1", summary=true, modifier=false)]
    pub address: Option<Address>,
    /// This contact detail is handled/monitored by a specific organization
    #[fhir(name="organization", min="0", max="1", summary=true, modifier=false)]
    pub organization: Option<Reference>,
    /// Period that this contact was valid for usage
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
}



#[derive(Complex, Debug, Clone, Default)]
pub struct HumanName {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// usual | official | temp | nickname | anonymous | old | maiden
    #[fhir(name="use", min="0", max="1", summary=true, modifier=true)]
    pub use_: Option<CodeDt>,
    /// Text representation of the full name
    #[fhir(name="text", min="0", max="1", summary=true, modifier=false)]
    pub text: Option<StringDt>,
    /// Family name (often called 'Surname')
    #[fhir(name="family", min="0", max="1", summary=true, modifier=false)]
    pub family: Option<StringDt>,
    /// Given names (not always 'first'). Includes middle names
    #[fhir(name="given", min="0", max="*", summary=true, modifier=false)]
    pub given: Option<Vec<StringDt>>,
    /// Parts that come before the name
    #[fhir(name="prefix", min="0", max="*", summary=true, modifier=false)]
    pub prefix: Option<Vec<StringDt>>,
    /// Parts that come after the name
    #[fhir(name="suffix", min="0", max="*", summary=true, modifier=false)]
    pub suffix: Option<Vec<StringDt>>,
    /// Time period when name was/is in use
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Identifier {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// usual | official | temp | secondary | old (If known)
    #[fhir(name="use", min="0", max="1", summary=true, modifier=true)]
    pub use_: Option<CodeDt>,
    /// Description of identifier
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The namespace for the identifier value
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// The value that is unique
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<StringDt>,
    /// Time period when id is/was valid for use
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<Period>,
    /// Organization that issued id (may be just text)
    #[fhir(name="assigner", min="0", max="1", summary=true, modifier=false)]
    pub assigner: Option<Reference>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct MarketingStatus {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// The country in which the marketing authorization has been granted shall be specified It should be specified using the ISO 3166 ‑ 1 alpha-2 code elements
    #[fhir(name="country", min="0", max="1", summary=true, modifier=false)]
    pub country: Option<CodeableConcept>,
    /// Where a Medicines Regulatory Agency has granted a marketing authorization for which specific provisions within a jurisdiction apply, the jurisdiction can be specified using an appropriate controlled terminology The controlled term and the controlled term identifier shall be specified
    #[fhir(name="jurisdiction", min="0", max="1", summary=true, modifier=false)]
    pub jurisdiction: Option<CodeableConcept>,
    /// This attribute provides information on the status of the marketing of the medicinal product See ISO/TS 20443 for more information and examples
    #[fhir(name="status", min="1", max="1", summary=true, modifier=false)]
    pub status: Option<CodeableConcept>,
    /// The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain
    #[fhir(name="dateRange", min="0", max="1", summary=true, modifier=false)]
    pub date_range: Option<Period>,
    /// The date when the Medicinal Product is placed on the market by the Marketing Authorization Holder (or where applicable, the manufacturer/distributor) in a country and/or jurisdiction shall be provided A complete date consisting of day, month and year shall be specified using the ISO 8601 date format NOTE “Placed on the market” refers to the release of the Medicinal Product into the distribution chain
    #[fhir(name="restoreDate", min="0", max="1", summary=true, modifier=false)]
    pub restore_date: Option<DateTimeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Meta {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Version specific identifier
    #[fhir(name="versionId", min="0", max="1", summary=true, modifier=false)]
    pub version_id: Option<IdDt>,
    /// When the resource version last changed
    #[fhir(name="lastUpdated", min="0", max="1", summary=true, modifier=false)]
    pub last_updated: Option<InstantDt>,
    /// Identifies where the resource comes from
    #[fhir(name="source", min="0", max="1", summary=true, modifier=false)]
    pub source: Option<UriDt>,
    /// Profiles this resource claims to conform to
    #[fhir(name="profile", min="0", max="*", summary=true, modifier=false)]
    pub profile: Option<Vec<CanonicalDt>>,
    /// Security Labels applied to this resource
    #[fhir(name="security", min="0", max="*", summary=true, modifier=false)]
    pub security: Option<Vec<Coding>>,
    /// Tags applied to this resource
    #[fhir(name="tag", min="0", max="*", summary=true, modifier=false)]
    pub tag: Option<Vec<Coding>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct MonetaryComponent {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// base | surcharge | deduction | discount | tax | informational
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Codes may be used to differentiate between kinds of taxes, surcharges, discounts etc.
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// Factor used for calculating this component
    #[fhir(name="factor", min="0", max="1", summary=true, modifier=false)]
    pub factor: Option<DecimalDt>,
    /// Explicit value amount to be used
    #[fhir(name="amount", min="0", max="1", summary=true, modifier=false)]
    pub amount: Option<Money>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Money {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// ISO 4217 Currency Code
    #[fhir(name="currency", min="0", max="1", summary=true, modifier=false)]
    pub currency: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Narrative {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// generated | extensions | additional | empty
    #[fhir(name="status", min="1", max="1", summary=false, modifier=false)]
    pub status: Option<CodeDt>,
    /// Limited xhtml content
    #[fhir(name="div", min="1", max="1", summary=false, modifier=false)]
    pub div: Option<XhtmlDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct ParameterDefinition {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Name used to access the parameter value
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<CodeDt>,
    /// in | out
    #[fhir(name="use", min="1", max="1", summary=true, modifier=false)]
    pub use_: Option<CodeDt>,
    /// Minimum cardinality
    #[fhir(name="min", min="0", max="1", summary=true, modifier=false)]
    pub min: Option<IntegerDt>,
    /// Maximum cardinality (a number of *)
    #[fhir(name="max", min="0", max="1", summary=true, modifier=false)]
    pub max: Option<StringDt>,
    /// A brief description of the parameter
    #[fhir(name="documentation", min="0", max="1", summary=true, modifier=false)]
    pub documentation: Option<StringDt>,
    /// What type of value
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// What profile the value is expected to be
    #[fhir(name="profile", min="0", max="1", summary=true, modifier=false)]
    pub profile: Option<CanonicalDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Period {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Starting time with inclusive boundary
    #[fhir(name="start", min="0", max="1", summary=true, modifier=false)]
    pub start: Option<DateTimeDt>,
    /// End time with inclusive boundary, if not ongoing
    #[fhir(name="end", min="0", max="1", summary=true, modifier=false)]
    pub end: Option<DateTimeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct PrimitiveType {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct ProductShelfLife {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// This describes the shelf life, taking into account various scenarios such as shelf life of the packaged Medicinal Product itself, shelf life after transformation where necessary and shelf life after the first opening of a bottle, etc. The shelf life type shall be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeableConcept>,
    /// The shelf life time period can be specified using a numerical value for the period of time and its unit of time measurement The unit of measurement shall be specified in accordance with ISO 11240 and the resulting terminology The symbol and the symbol identifier shall be used
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<StringDt>,
    /// Special precautions for storage, if any, can be specified using an appropriate controlled vocabulary The controlled term and the controlled term identifier shall be specified
    #[fhir(name="specialPrecautionsForStorage", min="0", max="*", summary=true, modifier=false)]
    pub special_precautions_for_storage: Option<Vec<CodeableConcept>>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Quantity {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Range {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Low limit
    #[fhir(name="low", min="0", max="1", summary=true, modifier=false)]
    pub low: Option<Quantity>,
    /// High limit
    #[fhir(name="high", min="0", max="1", summary=true, modifier=false)]
    pub high: Option<Quantity>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Ratio {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerator value
    #[fhir(name="numerator", min="0", max="1", summary=true, modifier=false)]
    pub numerator: Option<Quantity>,
    /// Denominator value
    #[fhir(name="denominator", min="0", max="1", summary=true, modifier=false)]
    pub denominator: Option<Quantity>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct RatioRange {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Low Numerator limit
    #[fhir(name="lowNumerator", min="0", max="1", summary=true, modifier=false)]
    pub low_numerator: Option<Quantity>,
    /// High Numerator limit
    #[fhir(name="highNumerator", min="0", max="1", summary=true, modifier=false)]
    pub high_numerator: Option<Quantity>,
    /// Denominator value
    #[fhir(name="denominator", min="0", max="1", summary=true, modifier=false)]
    pub denominator: Option<Quantity>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Reference {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Literal reference, Relative, internal or absolute URL
    #[fhir(name="reference", min="0", max="1", summary=true, modifier=false)]
    pub reference: Option<StringDt>,
    /// Type the reference refers to (e.g. "Patient") - must be a resource in resources
    #[fhir(name="type", min="0", max="1", summary=true, modifier=false)]
    pub type_: Option<UriDt>,
    /// Logical reference, when literal reference is not known
    #[fhir(name="identifier", min="0", max="1", summary=true, modifier=false)]
    pub identifier: Option<Box<Identifier>>,
    /// Text alternative for the resource
    #[fhir(name="display", min="0", max="1", summary=true, modifier=false)]
    pub display: Option<StringDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct RelatedArtifact {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// documentation | justification | citation | predecessor | successor | derived-from | depends-on | composed-of | part-of | amends | amended-with | appends | appended-with | cites | cited-by | comments-on | comment-in | contains | contained-in | corrects | correction-in | replaces | replaced-with | retracts | retracted-by | signs | similar-to | supports | supported-with | transforms | transformed-into | transformed-with | documents | specification-of | created-with | cite-as
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Additional classifiers
    #[fhir(name="classifier", min="0", max="*", summary=true, modifier=false)]
    pub classifier: Option<Vec<CodeableConcept>>,
    /// Short label
    #[fhir(name="label", min="0", max="1", summary=true, modifier=false)]
    pub label: Option<StringDt>,
    /// Brief description of the related artifact
    #[fhir(name="display", min="0", max="1", summary=true, modifier=false)]
    pub display: Option<StringDt>,
    /// Bibliographic citation for the artifact
    #[fhir(name="citation", min="0", max="1", summary=true, modifier=false)]
    pub citation: Option<MarkdownDt>,
    /// What document is being referenced
    #[fhir(name="document", min="0", max="1", summary=true, modifier=false)]
    pub document: Option<Attachment>,
    /// What artifact is being referenced
    #[fhir(name="resource", min="0", max="1", summary=true, modifier=false)]
    pub resource: Option<CanonicalDt>,
    /// What artifact, if not a conformance resource
    #[fhir(name="resourceReference", min="0", max="1", summary=true, modifier=false)]
    pub resource_reference: Option<Reference>,
    /// draft | active | retired | unknown
    #[fhir(name="publicationStatus", min="0", max="1", summary=true, modifier=false)]
    pub publication_status: Option<CodeDt>,
    /// Date of publication of the artifact being referred to
    #[fhir(name="publicationDate", min="0", max="1", summary=true, modifier=false)]
    pub publication_date: Option<DateDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct SampledData {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Zero value and units
    #[fhir(name="origin", min="1", max="1", summary=true, modifier=false)]
    pub origin: Option<Quantity>,
    /// Number of intervalUnits between samples
    #[fhir(name="interval", min="0", max="1", summary=true, modifier=false)]
    pub interval: Option<DecimalDt>,
    /// The measurement unit of the interval between samples
    #[fhir(name="intervalUnit", min="1", max="1", summary=true, modifier=false)]
    pub interval_unit: Option<CodeDt>,
    /// Multiply data by this before adding to origin
    #[fhir(name="factor", min="0", max="1", summary=true, modifier=false)]
    pub factor: Option<DecimalDt>,
    /// Lower limit of detection
    #[fhir(name="lowerLimit", min="0", max="1", summary=true, modifier=false)]
    pub lower_limit: Option<DecimalDt>,
    /// Upper limit of detection
    #[fhir(name="upperLimit", min="0", max="1", summary=true, modifier=false)]
    pub upper_limit: Option<DecimalDt>,
    /// Number of sample points at each time point
    #[fhir(name="dimensions", min="1", max="1", summary=true, modifier=false)]
    pub dimensions: Option<PositiveIntDt>,
    /// Defines the codes used in the data
    #[fhir(name="codeMap", min="0", max="1", summary=false, modifier=false)]
    pub code_map: Option<CanonicalDt>,
    /// Offsets, typically in time, at which data values were taken
    #[fhir(name="offsets", min="0", max="1", summary=false, modifier=false)]
    pub offsets: Option<StringDt>,
    /// Decimal values with spaces, or "E" | "U" | "L", or another code
    #[fhir(name="data", min="0", max="1", summary=false, modifier=false)]
    pub data: Option<StringDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Signature {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Indication of the reason the entity signed the object(s)
    #[fhir(name="type", min="0", max="*", summary=true, modifier=false)]
    pub type_: Option<Vec<Coding>>,
    /// When the signature was created
    #[fhir(name="when", min="0", max="1", summary=true, modifier=false)]
    pub when: Option<InstantDt>,
    /// Who signed
    #[fhir(name="who", min="0", max="1", summary=true, modifier=false)]
    pub who: Option<Reference>,
    /// The party represented
    #[fhir(name="onBehalfOf", min="0", max="1", summary=true, modifier=false)]
    pub on_behalf_of: Option<Reference>,
    /// The technical format of the signed resources
    #[fhir(name="targetFormat", min="0", max="1", summary=false, modifier=false)]
    pub target_format: Option<CodeDt>,
    /// The technical format of the signature
    #[fhir(name="sigFormat", min="0", max="1", summary=false, modifier=false)]
    pub sig_format: Option<CodeDt>,
    /// The actual signature content (XML DigSig. JWS, picture, etc.)
    #[fhir(name="data", min="0", max="1", summary=false, modifier=false)]
    pub data: Option<Base64BinaryDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct Timing {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// When the event occurs
    #[fhir(name="event", min="0", max="*", summary=true, modifier=false)]
    pub event: Option<Vec<DateTimeDt>>,
    /// When the event is to occur
    #[fhir(name="repeat", min="0", max="1", summary=true, modifier=false)]
    pub repeat: Option<TimingRepeatElement>,
    /// C | BID | TID | QID | AM | PM | QD | QOD | +
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableConcept>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct TimingRepeatElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Length/Range of lengths, or (Start and/or end) limits
    #[fhir(name="bounds", min="0", max="1", summary=true, modifier=false)]
    pub bounds: Option<Period>,
    /// Number of times to repeat
    #[fhir(name="count", min="0", max="1", summary=true, modifier=false)]
    pub count: Option<PositiveIntDt>,
    /// Maximum number of times to repeat
    #[fhir(name="countMax", min="0", max="1", summary=true, modifier=false)]
    pub count_max: Option<PositiveIntDt>,
    /// How long when it happens
    #[fhir(name="duration", min="0", max="1", summary=true, modifier=false)]
    pub duration: Option<DecimalDt>,
    /// How long when it happens (Max)
    #[fhir(name="durationMax", min="0", max="1", summary=true, modifier=false)]
    pub duration_max: Option<DecimalDt>,
    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    #[fhir(name="durationUnit", min="0", max="1", summary=true, modifier=false)]
    pub duration_unit: Option<CodeDt>,
    /// Indicates the number of repetitions that should occur within a period. I.e. Event occurs frequency times per period
    #[fhir(name="frequency", min="0", max="1", summary=true, modifier=false)]
    pub frequency: Option<PositiveIntDt>,
    /// Event occurs up to frequencyMax times per period
    #[fhir(name="frequencyMax", min="0", max="1", summary=true, modifier=false)]
    pub frequency_max: Option<PositiveIntDt>,
    /// The duration to which the frequency applies. I.e. Event occurs frequency times per period
    #[fhir(name="period", min="0", max="1", summary=true, modifier=false)]
    pub period: Option<DecimalDt>,
    /// Upper limit of period (3-4 hours)
    #[fhir(name="periodMax", min="0", max="1", summary=true, modifier=false)]
    pub period_max: Option<DecimalDt>,
    /// s | min | h | d | wk | mo | a - unit of time (UCUM)
    #[fhir(name="periodUnit", min="0", max="1", summary=true, modifier=false)]
    pub period_unit: Option<CodeDt>,
    /// mon | tue | wed | thu | fri | sat | sun
    #[fhir(name="dayOfWeek", min="0", max="*", summary=true, modifier=false)]
    pub day_of_week: Option<Vec<CodeDt>>,
    /// Time of day for action
    #[fhir(name="timeOfDay", min="0", max="*", summary=true, modifier=false)]
    pub time_of_day: Option<Vec<TimeDt>>,
    /// Code for time period of occurrence
    #[fhir(name="when", min="0", max="*", summary=true, modifier=false)]
    pub when: Option<Vec<CodeDt>>,
    /// Minutes from event (before or after)
    #[fhir(name="offset", min="0", max="1", summary=true, modifier=false)]
    pub offset: Option<UnsignedIntDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct TriggerDefinition {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// named-event | periodic | data-changed | data-added | data-modified | data-removed | data-accessed | data-access-ended
    #[fhir(name="type", min="1", max="1", summary=true, modifier=false)]
    pub type_: Option<CodeDt>,
    /// Name or URI that identifies the event
    #[fhir(name="name", min="0", max="1", summary=true, modifier=false)]
    pub name: Option<StringDt>,
    /// Coded definition of the event
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeableConcept>,
    /// What event
    #[fhir(name="subscriptionTopic", min="0", max="1", summary=true, modifier=false)]
    pub subscription_topic: Option<CanonicalDt>,
    /// Timing of the event
    #[fhir(name="timing", min="0", max="1", summary=true, modifier=false)]
    pub timing: Option<DateTimeDt>,
    /// Triggering data of the event (multiple = 'and')
    #[fhir(name="data", min="0", max="*", summary=true, modifier=false)]
    pub data: Option<Vec<DataRequirement>>,
    /// Whether the event triggers (boolean expression)
    #[fhir(name="condition", min="0", max="1", summary=true, modifier=false)]
    pub condition: Option<Expression>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct UsageContext {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Type of context being specified
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false)]
    pub code: Option<Coding>,
    /// Value that defines the context
    #[fhir(name="value", min="1", max="1", summary=true, modifier=false)]
    pub value: Option<Reference>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct VirtualServiceDetail {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Channel Type
    #[fhir(name="channelType", min="0", max="1", summary=true, modifier=false)]
    pub channel_type: Option<Coding>,
    /// Contact address/number
    #[fhir(name="address", min="0", max="1", summary=true, modifier=false)]
    pub address: Option<ExtendedContactDetail>,
    /// Address to see alternative connection details
    #[fhir(name="additionalInfo", min="0", max="*", summary=true, modifier=false)]
    pub additional_info: Option<Vec<UrlDt>>,
    /// Maximum number of participants supported by the virtual service
    #[fhir(name="maxParticipants", min="0", max="1", summary=true, modifier=false)]
    pub max_participants: Option<PositiveIntDt>,
    /// Session Key required by the virtual service
    #[fhir(name="sessionKey", min="0", max="1", summary=true, modifier=false)]
    pub session_key: Option<StringDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct MoneyQuantity {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="1", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}

#[derive(Complex, Debug, Clone, Default)]
pub struct SimpleQuantity {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false)]
    pub id: Option<Id>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false)]
    pub extension: Option<Vec<Extension>>,
    /// Numerical value (with implicit precision)
    #[fhir(name="value", min="0", max="1", summary=true, modifier=false)]
    pub value: Option<DecimalDt>,
    /// < | <= | >= | > | ad - how to understand the value
    #[fhir(name="comparator", min="0", max="0", summary=true, modifier=true)]
    pub comparator: Option<CodeDt>,
    /// Unit representation
    #[fhir(name="unit", min="0", max="1", summary=true, modifier=false)]
    pub unit: Option<StringDt>,
    /// System that defines coded unit form
    #[fhir(name="system", min="0", max="1", summary=true, modifier=false)]
    pub system: Option<UriDt>,
    /// Coded form of the unit
    #[fhir(name="code", min="0", max="1", summary=true, modifier=false)]
    pub code: Option<CodeDt>,
}
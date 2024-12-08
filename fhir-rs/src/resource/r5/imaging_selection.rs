use crate::prelude::*;

#[derive(Resource, Debug, Clone, Default)]
#[fhir(base="DomainResource")]
pub struct ImagingSelection {
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
    /// Business Identifier for Imaging Selection
    #[fhir(name="identifier", min="0", max="*", summary=true, modifier=false, choice="")]
    pub identifier: Option<Vec<Identifier>>,
    /// available | entered-in-error | unknown
    #[fhir(name="status", min="1", max="1", summary=true, modifier=true)]
    pub status: Option<CodeDt>,
    /// Subject of the selected instances
    #[fhir(name="subject", min="0", max="1", summary=true, modifier=false, choice="")]
    pub subject: Option<Reference>,
    /// Date / Time when this imaging selection was created
    #[fhir(name="issued", min="0", max="1", summary=true, modifier=false, choice="")]
    pub issued: Option<InstantDt>,
    /// Selector of the instances (human or machine)
    #[fhir(name="performer", min="0", max="*", summary=true, modifier=false, choice="")]
    pub performer: Option<Vec<ImagingSelectionPerformerBackboneElement>>,
    /// Associated request
    #[fhir(name="basedOn", min="0", max="*", summary=true, modifier=false, choice="")]
    pub based_on: Option<Vec<Reference>>,
    /// Classifies the imaging selection
    #[fhir(name="category", min="0", max="*", summary=true, modifier=false, choice="")]
    pub category: Option<Vec<CodeableConcept>>,
    /// Imaging Selection purpose text or code
    #[fhir(name="code", min="1", max="1", summary=true, modifier=false, choice="")]
    pub code: Option<CodeableConcept>,
    /// DICOM Study Instance UID
    #[fhir(name="studyUid", min="0", max="1", summary=true, modifier=false, choice="")]
    pub study_uid: Option<IdDt>,
    /// The imaging study from which the imaging selection is derived
    #[fhir(name="derivedFrom", min="0", max="*", summary=true, modifier=false, choice="")]
    pub derived_from: Option<Vec<Reference>>,
    /// The network service providing retrieval for the images referenced in the imaging selection
    #[fhir(name="endpoint", min="0", max="*", summary=true, modifier=false, choice="")]
    pub endpoint: Option<Vec<Reference>>,
    /// DICOM Series Instance UID
    #[fhir(name="seriesUid", min="0", max="1", summary=true, modifier=false, choice="")]
    pub series_uid: Option<IdDt>,
    /// DICOM Series Number
    #[fhir(name="seriesNumber", min="0", max="1", summary=true, modifier=false, choice="")]
    pub series_number: Option<UnsignedIntDt>,
    /// The Frame of Reference UID for the selected images
    #[fhir(name="frameOfReferenceUid", min="0", max="1", summary=true, modifier=false, choice="")]
    pub frame_of_reference_uid: Option<IdDt>,
    /// Body part examined
    #[fhir(name="bodySite", min="0", max="1", summary=true, modifier=false, choice="")]
    pub body_site: Option<CodeableReference>,
    /// Related resource that is the focus for the imaging selection
    #[fhir(name="focus", min="0", max="*", summary=true, modifier=false, choice="")]
    pub focus: Option<Vec<Reference>>,
    /// The selected instances
    #[fhir(name="instance", min="0", max="*", summary=true, modifier=false, choice="")]
    pub instance: Option<Vec<ImagingSelectionInstanceBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImagingSelectionPerformerBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Type of performer
    #[fhir(name="function", min="0", max="1", summary=true, modifier=false, choice="")]
    pub function: Option<CodeableConcept>,
    /// Author (human or machine)
    #[fhir(name="actor", min="0", max="1", summary=true, modifier=false, choice="")]
    pub actor: Option<Reference>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImagingSelectionInstanceBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// DICOM SOP Instance UID
    #[fhir(name="uid", min="1", max="1", summary=true, modifier=false, choice="")]
    pub uid: Option<IdDt>,
    /// DICOM Instance Number
    #[fhir(name="number", min="0", max="1", summary=true, modifier=false, choice="")]
    pub number: Option<UnsignedIntDt>,
    /// DICOM SOP Class UID
    #[fhir(name="sopClass", min="0", max="1", summary=false, modifier=false, choice="")]
    pub sop_class: Option<Coding>,
    /// The selected subset of the SOP Instance
    #[fhir(name="subset", min="0", max="*", summary=false, modifier=false, choice="")]
    pub subset: Option<Vec<StringDt>>,
    /// A specific 2D region in a DICOM image / frame
    #[fhir(name="imageRegion2D", min="0", max="*", summary=false, modifier=false, choice="")]
    pub image_region_2_d: Option<Vec<ImagingSelectionInstanceImageRegion2DBackboneElement>>,
    /// A specific 3D region in a DICOM frame of reference
    #[fhir(name="imageRegion3D", min="0", max="*", summary=false, modifier=false, choice="")]
    pub image_region_3_d: Option<Vec<ImagingSelectionInstanceImageRegion3DBackboneElement>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImagingSelectionInstanceImageRegion2DBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// point | polyline | interpolated | circle | ellipse
    #[fhir(name="regionType", min="1", max="1", summary=false, modifier=false, choice="")]
    pub region_type: Option<CodeDt>,
    /// Specifies the coordinates that define the image region
    #[fhir(name="coordinate", min="1", max="*", summary=false, modifier=false, choice="")]
    pub coordinate: Option<Vec<DecimalDt>>,
}

#[derive(Element, BackboneElement, Debug, Clone, Default)]
pub struct ImagingSelectionInstanceImageRegion3DBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary=false, modifier=false, choice="")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary=false, modifier=false, choice="")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary=true, modifier=true)]
    pub modifier_extension: Option<Vec<Extension>>,
    /// point | multipoint | polyline | polygon | ellipse | ellipsoid
    #[fhir(name="regionType", min="1", max="1", summary=false, modifier=false, choice="")]
    pub region_type: Option<CodeDt>,
    /// Specifies the coordinates that define the image region
    #[fhir(name="coordinate", min="1", max="*", summary=false, modifier=false, choice="")]
    pub coordinate: Option<Vec<DecimalDt>>,
}


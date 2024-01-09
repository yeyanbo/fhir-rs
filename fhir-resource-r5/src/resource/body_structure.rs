use fhir_rs::prelude::*;
use crate::Resource;

#[derive(Resource, Debug, Clone, Default)]
pub struct BodyStructure {
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
    pub contained: Option<Vec<Resource>>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Bodystructure identifier
    #[fhir(name="identifier", min="0", max="*", summary="true", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// Whether this record is in active use
    #[fhir(name="active", min="0", max="1", summary="true", modifier="true")]
    pub active: Option<BooleanDt>,
    /// Kind of Structure
    #[fhir(name="morphology", min="0", max="1", summary="true", modifier="false")]
    pub morphology: Option<CodeableConcept>,
    /// Included anatomic location(s)
    #[fhir(name="includedStructure", min="1", max="*", summary="false", modifier="false")]
    pub included_structure: Option<Vec<BodyStructureIncludedStructureBackboneElement>>,
    /// Excluded anatomic locations(s)
    #[fhir(name="excludedStructure", min="0", max="*", summary="false", modifier="false")]
    pub excluded_structure: Option<Vec<BodyStructureIncludedStructureBackboneElement>>,
    /// Text description
    #[fhir(name="description", min="0", max="1", summary="true", modifier="false")]
    pub description: Option<MarkdownDt>,
    /// Attached images
    #[fhir(name="image", min="0", max="*", summary="false", modifier="false")]
    pub image: Option<Vec<Attachment>>,
    /// Who this is about
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BodyStructureIncludedStructureBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Code that represents the included structure
    #[fhir(name="structure", min="1", max="1", summary="true", modifier="false")]
    pub structure: Option<CodeableConcept>,
    /// Code that represents the included structure laterality
    #[fhir(name="laterality", min="0", max="1", summary="false", modifier="false")]
    pub laterality: Option<CodeableConcept>,
    /// Landmark relative location
    #[fhir(name="bodyLandmarkOrientation", min="0", max="*", summary="false", modifier="false")]
    pub body_landmark_orientation: Option<Vec<BodyStructureIncludedStructureBodyLandmarkOrientationBackboneElement>>,
    /// Cartesian reference for structure
    #[fhir(name="spatialReference", min="0", max="*", summary="false", modifier="false")]
    pub spatial_reference: Option<Vec<Reference>>,
    /// Code that represents the included structure qualifier
    #[fhir(name="qualifier", min="0", max="*", summary="false", modifier="false")]
    pub qualifier: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Body ]andmark description
    #[fhir(name="landmarkDescription", min="0", max="*", summary="false", modifier="false")]
    pub landmark_description: Option<Vec<CodeableConcept>>,
    /// Clockface orientation
    #[fhir(name="clockFacePosition", min="0", max="*", summary="false", modifier="false")]
    pub clock_face_position: Option<Vec<CodeableConcept>>,
    /// Landmark relative location
    #[fhir(name="distanceFromLandmark", min="0", max="*", summary="false", modifier="false")]
    pub distance_from_landmark: Option<Vec<BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmarkBackboneElement>>,
    /// Relative landmark surface orientation
    #[fhir(name="surfaceOrientation", min="0", max="*", summary="false", modifier="false")]
    pub surface_orientation: Option<Vec<CodeableConcept>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct BodyStructureIncludedStructureBodyLandmarkOrientationDistanceFromLandmarkBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Measurement device
    #[fhir(name="device", min="0", max="*", summary="false", modifier="false")]
    pub device: Option<Vec<CodeableReference>>,
    /// Measured distance from body landmark
    #[fhir(name="value", min="0", max="*", summary="false", modifier="false")]
    pub value: Option<Vec<Quantity>>,
}


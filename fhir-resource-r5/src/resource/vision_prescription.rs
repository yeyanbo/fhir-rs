use fhir_rs::prelude::*;
use crate::AnyResource;

#[derive(Resource, Debug, Clone, Default)]
pub struct VisionPrescription {
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
    /// Business Identifier for vision prescription
    #[fhir(name="identifier", min="0", max="*", summary="false", modifier="false")]
    pub identifier: Option<Vec<Identifier>>,
    /// active | cancelled | draft | entered-in-error
    #[fhir(name="status", min="1", max="1", summary="true", modifier="true")]
    pub status: Option<CodeDt>,
    /// Response creation date
    #[fhir(name="created", min="1", max="1", summary="true", modifier="false")]
    pub created: Option<DateTimeDt>,
    /// Who prescription is for
    #[fhir(name="patient", min="1", max="1", summary="true", modifier="false")]
    pub patient: Option<Reference>,
    /// Created during encounter / admission / stay
    #[fhir(name="encounter", min="0", max="1", summary="false", modifier="false")]
    pub encounter: Option<Reference>,
    /// When prescription was authorized
    #[fhir(name="dateWritten", min="1", max="1", summary="true", modifier="false")]
    pub date_written: Option<DateTimeDt>,
    /// Who authorized the vision prescription
    #[fhir(name="prescriber", min="1", max="1", summary="true", modifier="false")]
    pub prescriber: Option<Reference>,
    /// Vision lens authorization
    #[fhir(name="lensSpecification", min="1", max="*", summary="true", modifier="false")]
    pub lens_specification: Option<Vec<VisionPrescriptionLensSpecificationBackboneElement>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct VisionPrescriptionLensSpecificationBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Product to be supplied
    #[fhir(name="product", min="1", max="1", summary="true", modifier="false")]
    pub product: Option<CodeableConcept>,
    /// right | left
    #[fhir(name="eye", min="1", max="1", summary="true", modifier="false")]
    pub eye: Option<CodeDt>,
    /// Power of the lens
    #[fhir(name="sphere", min="0", max="1", summary="false", modifier="false")]
    pub sphere: Option<DecimalDt>,
    /// Lens power for astigmatism
    #[fhir(name="cylinder", min="0", max="1", summary="false", modifier="false")]
    pub cylinder: Option<DecimalDt>,
    /// Lens meridian which contain no power for astigmatism
    #[fhir(name="axis", min="0", max="1", summary="false", modifier="false")]
    pub axis: Option<IntegerDt>,
    /// Eye alignment compensation
    #[fhir(name="prism", min="0", max="*", summary="false", modifier="false")]
    pub prism: Option<Vec<VisionPrescriptionLensSpecificationPrismBackboneElement>>,
    /// Added power for multifocal levels
    #[fhir(name="add", min="0", max="1", summary="false", modifier="false")]
    pub add: Option<DecimalDt>,
    /// Contact lens power
    #[fhir(name="power", min="0", max="1", summary="false", modifier="false")]
    pub power: Option<DecimalDt>,
    /// Contact lens back curvature
    #[fhir(name="backCurve", min="0", max="1", summary="false", modifier="false")]
    pub back_curve: Option<DecimalDt>,
    /// Contact lens diameter
    #[fhir(name="diameter", min="0", max="1", summary="false", modifier="false")]
    pub diameter: Option<DecimalDt>,
    /// Lens wear duration
    #[fhir(name="duration", min="0", max="1", summary="false", modifier="false")]
    pub duration: Option<Quantity>,
    /// Color required
    #[fhir(name="color", min="0", max="1", summary="false", modifier="false")]
    pub color: Option<StringDt>,
    /// Brand required
    #[fhir(name="brand", min="0", max="1", summary="false", modifier="false")]
    pub brand: Option<StringDt>,
    /// Notes for coatings
    #[fhir(name="note", min="0", max="*", summary="false", modifier="false")]
    pub note: Option<Vec<Annotation>>,
}

#[derive(BackboneElement, Debug, Clone, Default)]
pub struct VisionPrescriptionLensSpecificationPrismBackboneElement {
    /// Unique id for inter-element referencing
    #[fhir(name="id", min="0", max="1", summary="false", modifier="false")]
    pub id: Option<String>,
    /// Additional content defined by implementations
    #[fhir(name="extension", min="0", max="*", summary="false", modifier="false")]
    pub extension: Option<Vec<Extension>>,
    /// Extensions that cannot be ignored even if unrecognized
    #[fhir(name="modifierExtension", min="0", max="*", summary="true", modifier="true")]
    pub modifier_extension: Option<Vec<Extension>>,
    /// Amount of adjustment
    #[fhir(name="amount", min="1", max="1", summary="false", modifier="false")]
    pub amount: Option<DecimalDt>,
    /// up | down | in | out
    #[fhir(name="base", min="1", max="1", summary="false", modifier="false")]
    pub base: Option<CodeDt>,
}


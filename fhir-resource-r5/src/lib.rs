mod resource;

use fhir_rs::prelude::*;
pub use resource::*;

macro_rules! any_resources {
    (
        $(
            $resource:ident,
        )+
    ) => {
        #[derive(Debug, Clone)]
        pub enum AnyResource {
        $(
            $resource($resource),
        )+
        }

        impl Executor for AnyResource {
            fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
                match self {
                    $(
                    AnyResource::$resource(resource) => resource.path(paths),
                    )+
                    
                }
            }

            fn as_collection(&self) -> Collection {
                match self {
                    $(
                    AnyResource::$resource(resource) => Collection(vec![Box::new(resource.clone())]),
                    )+
                    
                }
            }
        }
        
        impl Serialize for AnyResource {
            fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
                match self {
                    $(
                    AnyResource::$resource(resource) => { serializer.serialize_any("resource", resource) }
                    )+
                }
            }
        }
    }
}

any_resources!{
    Account,
    ActivityDefinition,
    ActorDefinition,
    AdministrableProductDefinition,
    AdverseEvent,
    AllergyIntolerance,
    Appointment,
    AppointmentResponse,
    ArtifactAssessment,
    AuditEvent,
    Basic,
    Binary,
    BiologicallyDerivedProduct,
    BiologicallyDerivedProductDispense,
    BodyStructure,
    Bundle,
    CapabilityStatement,
    CarePlan,
    CareTeam,
    ChargeItem,
    ChargeItemDefinition,
    Citation,
    Claim,
    ClaimResponse,
    ClinicalImpression,
    ClinicalUseDefinition,
    CodeSystem,
    Communication,
    CommunicationRequest,
    CompartmentDefinition,
    Composition,
    ConceptMap,
    Condition,
    ConditionDefinition,
    Consent,
    Contract,
    Coverage,
    CoverageEligibilityRequest,
    CoverageEligibilityResponse,
    DetectedIssue,
    Device,
    DeviceAssociation,
    DeviceDefinition,
    DeviceDispense,
    DeviceMetric,
    DeviceRequest,
    DeviceUsage,
    DiagnosticReport,
    DocumentReference,
    Encounter,
    EncounterHistory,
    Endpoint,
    EnrollmentRequest,
    EnrollmentResponse,
    EpisodeOfCare,
    EventDefinition,
    Evidence,
    EvidenceReport,
    EvidenceVariable,
    ExampleScenario,
    ExplanationOfBenefit,
    FamilyMemberHistory,
    Flag,
    FormularyItem,
    GenomicStudy,
    Goal,
    GraphDefinition,
    Group,
    GuidanceResponse,
    HealthcareService,
    ImagingSelection,
    ImagingStudy,
    Immunization,
    ImmunizationEvaluation,
    ImmunizationRecommendation,
    ImplementationGuide,
    Ingredient,
    InsurancePlan,
    InventoryItem,
    InventoryReport,
    Invoice,
    Library,
    Linkage,
    List,
    Location,
    ManufacturedItemDefinition,
    Measure,
    MeasureReport,
    Medication,
    MedicationAdministration,
    MedicationDispense,
    MedicationKnowledge,
    MedicationRequest,
    MedicationStatement,
    MedicinalProductDefinition,
    MessageDefinition,
    MessageHeader,
    MolecularSequence,
    NamingSystem,
    NutritionIntake,
    NutritionOrder,
    NutritionProduct,
    Observation,
    ObservationDefinition,
    OperationDefinition,
    OperationOutcome,
    Organization,
    OrganizationAffiliation,
    PackagedProductDefinition,
    Parameters,
    Patient,
    PaymentNotice,
    PaymentReconciliation,
    Permission,
    Person,
    PlanDefinition,
    Practitioner,
    PractitionerRole,
    Procedure,
    Provenance,
    Questionnaire,
    QuestionnaireResponse,
    RegulatedAuthorization,
    RelatedPerson,
    RequestOrchestration,
    Requirements,
    ResearchStudy,
    ResearchSubject,
    RiskAssessment,
    Schedule,
    SearchParameter,
    ServiceRequest,
    Slot,
    Specimen,
    SpecimenDefinition,
    StructureDefinition,
    StructureMap,
    Subscription,
    SubscriptionStatus,
    SubscriptionTopic,
    Substance,
    SubstanceDefinition,
    SubstanceNucleicAcid,
    SubstancePolymer,
    SubstanceProtein,
    SubstanceReferenceInformation,
    SubstanceSourceMaterial,
    SupplyDelivery,
    SupplyRequest,
    Task,
    TerminologyCapabilities,
    TestPlan,
    TestReport,
    TestScript,
    Transport,
    ValueSet,
    VerificationResult,
    VisionPrescription,
}

impl<'de> Deserialize<'de> for AnyResource {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {

        pub struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = AnyResource;
        }

        deserializer.deserialize_enum(AnyVisitor)
    }
}
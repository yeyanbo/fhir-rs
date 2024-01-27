mod resource;

use fhir_rs::prelude::*;
pub use resource::*;

#[derive(Debug, Clone)]
pub enum AnyResource {
    Account(Account),
    ActivityDefinition(ActivityDefinition),
    ActorDefinition(ActorDefinition),
    AdministrableProductDefinition(AdministrableProductDefinition),
    AdverseEvent(AdverseEvent),
    AllergyIntolerance(AllergyIntolerance),
    Appointment(Appointment),
    AppointmentResponse(AppointmentResponse),
    ArtifactAssessment(ArtifactAssessment),
    AuditEvent(AuditEvent),
    Basic(Basic),
    Binary(Binary),
    BiologicallyDerivedProduct(BiologicallyDerivedProduct),
    BiologicallyDerivedProductDispense(BiologicallyDerivedProductDispense),
    BodyStructure(BodyStructure),
    Bundle(Bundle),
    CapabilityStatement(CapabilityStatement),
    CarePlan(CarePlan),
    CareTeam(CareTeam),
    ChargeItem(ChargeItem),
    ChargeItemDefinition(ChargeItemDefinition),
    Citation(Citation),
    Claim(Claim),
    ClaimResponse(ClaimResponse),
    ClinicalImpression(ClinicalImpression),
    ClinicalUseDefinition(ClinicalUseDefinition),
    CodeSystem(CodeSystem),
    Communication(Communication),
    CommunicationRequest(CommunicationRequest),
    CompartmentDefinition(CompartmentDefinition),
    Composition(Composition),
    ConceptMap(ConceptMap),
    Condition(Condition),
    ConditionDefinition(ConditionDefinition),
    Consent(Consent),
    Contract(Contract),
    Coverage(Coverage),
    CoverageEligibilityRequest(CoverageEligibilityRequest),
    CoverageEligibilityResponse(CoverageEligibilityResponse),
    DetectedIssue(DetectedIssue),
    Device(Device),
    DeviceAssociation(DeviceAssociation),
    DeviceDefinition(DeviceDefinition),
    DeviceDispense(DeviceDispense),
    DeviceMetric(DeviceMetric),
    DeviceRequest(DeviceRequest),
    DeviceUsage(DeviceUsage),
    DiagnosticReport(DiagnosticReport),
    DocumentReference(DocumentReference),
    Encounter(Encounter),
    EncounterHistory(EncounterHistory),
    Endpoint(Endpoint),
    EnrollmentRequest(EnrollmentRequest),
    EnrollmentResponse(EnrollmentResponse),
    EpisodeOfCare(EpisodeOfCare),
    EventDefinition(EventDefinition),
    Evidence(Evidence),
    EvidenceReport(EvidenceReport),
    EvidenceVariable(EvidenceVariable),
    ExampleScenario(ExampleScenario),
    ExplanationOfBenefit(ExplanationOfBenefit),
    FamilyMemberHistory(FamilyMemberHistory),
    Flag(Flag),
    FormularyItem(FormularyItem),
    GenomicStudy(GenomicStudy),
    Goal(Goal),
    GraphDefinition(GraphDefinition),
    Group(Group),
    GuidanceResponse(GuidanceResponse),
    HealthcareService(HealthcareService),
    ImagingSelection(ImagingSelection),
    ImagingStudy(ImagingStudy),
    Immunization(Immunization),
    ImmunizationEvaluation(ImmunizationEvaluation),
    ImmunizationRecommendation(ImmunizationRecommendation),
    ImplementationGuide(ImplementationGuide),
    Ingredient(Ingredient),
    InsurancePlan(InsurancePlan),
    InventoryItem(InventoryItem),
    InventoryReport(InventoryReport),
    Invoice(Invoice),
    Library(Library),
    Linkage(Linkage),
    List(List),
    Location(Location),
    ManufacturedItemDefinition(ManufacturedItemDefinition),
    Measure(Measure),
    MeasureReport(MeasureReport),
    Medication(Medication),
    MedicationAdministration(MedicationAdministration),
    MedicationDispense(MedicationDispense),
    MedicationKnowledge(MedicationKnowledge),
    MedicationRequest(MedicationRequest),
    MedicationStatement(MedicationStatement),
    MedicinalProductDefinition(MedicinalProductDefinition),
    MessageDefinition(MessageDefinition),
    MessageHeader(MessageHeader),
    MolecularSequence(MolecularSequence),
    NamingSystem(NamingSystem),
    NutritionIntake(NutritionIntake),
    NutritionOrder(NutritionOrder),
    NutritionProduct(NutritionProduct),
    Observation(Observation),
    ObservationDefinition(ObservationDefinition),
    OperationDefinition(OperationDefinition),
    OperationOutcome(OperationOutcome),
    Organization(Organization),
    OrganizationAffiliation(OrganizationAffiliation),
    PackagedProductDefinition(PackagedProductDefinition),
    Parameters(Parameters),
    Patient(Patient),
    PaymentNotice(PaymentNotice),
    PaymentReconciliation(PaymentReconciliation),
    Permission(Permission),
    Person(Person),
    PlanDefinition(PlanDefinition),
    Practitioner(Practitioner),
    PractitionerRole(PractitionerRole),
    Procedure(Procedure),
    Provenance(Provenance),
    Questionnaire(Questionnaire),
    QuestionnaireResponse(QuestionnaireResponse),
    RegulatedAuthorization(RegulatedAuthorization),
    RelatedPerson(RelatedPerson),
    RequestOrchestration(RequestOrchestration),
    Requirements(Requirements),
    ResearchStudy(ResearchStudy),
    ResearchSubject(ResearchSubject),
    RiskAssessment(RiskAssessment),
    Schedule(Schedule),
    SearchParameter(SearchParameter),
    ServiceRequest(ServiceRequest),
    Slot(Slot),
    Specimen(Specimen),
    SpecimenDefinition(SpecimenDefinition),
    StructureDefinition(StructureDefinition),
    StructureMap(StructureMap),
    Subscription(Subscription),
    SubscriptionStatus(SubscriptionStatus),
    SubscriptionTopic(SubscriptionTopic),
    Substance(Substance),
    SubstanceDefinition(SubstanceDefinition),
    SubstanceNucleicAcid(SubstanceNucleicAcid),
    SubstancePolymer(SubstancePolymer),
    SubstanceProtein(SubstanceProtein),
    SubstanceReferenceInformation(SubstanceReferenceInformation),
    SubstanceSourceMaterial(SubstanceSourceMaterial),
    SupplyDelivery(SupplyDelivery),
    SupplyRequest(SupplyRequest),
    Task(Task),
    TerminologyCapabilities(TerminologyCapabilities),
    TestPlan(TestPlan),
    TestReport(TestReport),
    TestScript(TestScript),
    Transport(Transport),
    ValueSet(ValueSet),
    VerificationResult(VerificationResult),
    VisionPrescription(VisionPrescription),
}

impl Serialize for AnyResource {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        match self {
            AnyResource::Account(resource) => { serializer.serialize_any("resource", resource) }
            AnyResource::ActivityDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ActorDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::AdministrableProductDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::AdverseEvent(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::AllergyIntolerance(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Appointment(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::AppointmentResponse(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ArtifactAssessment(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::AuditEvent(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Basic(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Binary(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::BiologicallyDerivedProduct(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::BiologicallyDerivedProductDispense(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::BodyStructure(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Bundle(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CapabilityStatement(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CarePlan(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CareTeam(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ChargeItem(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ChargeItemDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Citation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Claim(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ClaimResponse(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ClinicalImpression(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ClinicalUseDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CodeSystem(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Communication(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CommunicationRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CompartmentDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Composition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ConceptMap(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Condition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ConditionDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Consent(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Contract(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Coverage(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CoverageEligibilityRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::CoverageEligibilityResponse(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DetectedIssue(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Device(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DeviceAssociation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DeviceDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DeviceDispense(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DeviceMetric(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DeviceRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DeviceUsage(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DiagnosticReport(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::DocumentReference(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Encounter(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EncounterHistory(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Endpoint(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EnrollmentRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EnrollmentResponse(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EpisodeOfCare(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EventDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Evidence(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EvidenceReport(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::EvidenceVariable(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ExampleScenario(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ExplanationOfBenefit(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::FamilyMemberHistory(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Flag(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::FormularyItem(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::GenomicStudy(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Goal(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::GraphDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Group(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::GuidanceResponse(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::HealthcareService(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ImagingSelection(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ImagingStudy(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Immunization(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ImmunizationEvaluation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ImmunizationRecommendation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ImplementationGuide(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Ingredient(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::InsurancePlan(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::InventoryItem(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::InventoryReport(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Invoice(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Library(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Linkage(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::List(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Location(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ManufacturedItemDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Measure(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MeasureReport(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Medication(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MedicationAdministration(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MedicationDispense(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MedicationKnowledge(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MedicationRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MedicationStatement(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MedicinalProductDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MessageDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MessageHeader(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::MolecularSequence(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::NamingSystem(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::NutritionIntake(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::NutritionOrder(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::NutritionProduct(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Observation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ObservationDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::OperationDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::OperationOutcome(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Organization(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::OrganizationAffiliation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::PackagedProductDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Parameters(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Patient(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::PaymentNotice(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::PaymentReconciliation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Permission(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Person(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::PlanDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Practitioner(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::PractitionerRole(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Procedure(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Provenance(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Questionnaire(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::QuestionnaireResponse(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::RegulatedAuthorization(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::RelatedPerson(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::RequestOrchestration(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Requirements(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ResearchStudy(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ResearchSubject(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::RiskAssessment(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Schedule(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SearchParameter(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ServiceRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Slot(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Specimen(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SpecimenDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::StructureDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::StructureMap(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Subscription(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubscriptionStatus(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubscriptionTopic(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Substance(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubstanceDefinition(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubstanceNucleicAcid(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubstancePolymer(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubstanceProtein(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubstanceReferenceInformation(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SubstanceSourceMaterial(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SupplyDelivery(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::SupplyRequest(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Task(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::TerminologyCapabilities(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::TestPlan(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::TestReport(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::TestScript(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::Transport(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::ValueSet(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::VerificationResult(resource) => {serializer.serialize_any("resource", resource)}
            AnyResource::VisionPrescription(resource) => {serializer.serialize_any("resource", resource)}
        }
    }
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
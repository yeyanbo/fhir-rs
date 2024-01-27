mod resource;

use fhir_rs::prelude::*;
pub use resource::*;

#[derive(Debug, Clone)]
pub enum Resource {
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

impl Serialize for Resource {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        match self {
            Resource::Account(resource) => { serializer.serialize_any("resource", resource) }
            Resource::ActivityDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ActorDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::AdministrableProductDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::AdverseEvent(resource) => {serializer.serialize_any("resource", resource)}
            Resource::AllergyIntolerance(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Appointment(resource) => {serializer.serialize_any("resource", resource)}
            Resource::AppointmentResponse(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ArtifactAssessment(resource) => {serializer.serialize_any("resource", resource)}
            Resource::AuditEvent(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Basic(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Binary(resource) => {serializer.serialize_any("resource", resource)}
            Resource::BiologicallyDerivedProduct(resource) => {serializer.serialize_any("resource", resource)}
            Resource::BiologicallyDerivedProductDispense(resource) => {serializer.serialize_any("resource", resource)}
            Resource::BodyStructure(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Bundle(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CapabilityStatement(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CarePlan(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CareTeam(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ChargeItem(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ChargeItemDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Citation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Claim(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ClaimResponse(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ClinicalImpression(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ClinicalUseDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CodeSystem(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Communication(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CommunicationRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CompartmentDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Composition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ConceptMap(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Condition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ConditionDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Consent(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Contract(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Coverage(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CoverageEligibilityRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::CoverageEligibilityResponse(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DetectedIssue(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Device(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DeviceAssociation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DeviceDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DeviceDispense(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DeviceMetric(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DeviceRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DeviceUsage(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DiagnosticReport(resource) => {serializer.serialize_any("resource", resource)}
            Resource::DocumentReference(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Encounter(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EncounterHistory(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Endpoint(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EnrollmentRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EnrollmentResponse(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EpisodeOfCare(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EventDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Evidence(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EvidenceReport(resource) => {serializer.serialize_any("resource", resource)}
            Resource::EvidenceVariable(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ExampleScenario(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ExplanationOfBenefit(resource) => {serializer.serialize_any("resource", resource)}
            Resource::FamilyMemberHistory(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Flag(resource) => {serializer.serialize_any("resource", resource)}
            Resource::FormularyItem(resource) => {serializer.serialize_any("resource", resource)}
            Resource::GenomicStudy(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Goal(resource) => {serializer.serialize_any("resource", resource)}
            Resource::GraphDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Group(resource) => {serializer.serialize_any("resource", resource)}
            Resource::GuidanceResponse(resource) => {serializer.serialize_any("resource", resource)}
            Resource::HealthcareService(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ImagingSelection(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ImagingStudy(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Immunization(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ImmunizationEvaluation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ImmunizationRecommendation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ImplementationGuide(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Ingredient(resource) => {serializer.serialize_any("resource", resource)}
            Resource::InsurancePlan(resource) => {serializer.serialize_any("resource", resource)}
            Resource::InventoryItem(resource) => {serializer.serialize_any("resource", resource)}
            Resource::InventoryReport(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Invoice(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Library(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Linkage(resource) => {serializer.serialize_any("resource", resource)}
            Resource::List(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Location(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ManufacturedItemDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Measure(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MeasureReport(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Medication(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MedicationAdministration(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MedicationDispense(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MedicationKnowledge(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MedicationRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MedicationStatement(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MedicinalProductDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MessageDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MessageHeader(resource) => {serializer.serialize_any("resource", resource)}
            Resource::MolecularSequence(resource) => {serializer.serialize_any("resource", resource)}
            Resource::NamingSystem(resource) => {serializer.serialize_any("resource", resource)}
            Resource::NutritionIntake(resource) => {serializer.serialize_any("resource", resource)}
            Resource::NutritionOrder(resource) => {serializer.serialize_any("resource", resource)}
            Resource::NutritionProduct(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Observation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ObservationDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::OperationDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::OperationOutcome(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Organization(resource) => {serializer.serialize_any("resource", resource)}
            Resource::OrganizationAffiliation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::PackagedProductDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Parameters(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Patient(resource) => {serializer.serialize_any("resource", resource)}
            Resource::PaymentNotice(resource) => {serializer.serialize_any("resource", resource)}
            Resource::PaymentReconciliation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Permission(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Person(resource) => {serializer.serialize_any("resource", resource)}
            Resource::PlanDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Practitioner(resource) => {serializer.serialize_any("resource", resource)}
            Resource::PractitionerRole(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Procedure(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Provenance(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Questionnaire(resource) => {serializer.serialize_any("resource", resource)}
            Resource::QuestionnaireResponse(resource) => {serializer.serialize_any("resource", resource)}
            Resource::RegulatedAuthorization(resource) => {serializer.serialize_any("resource", resource)}
            Resource::RelatedPerson(resource) => {serializer.serialize_any("resource", resource)}
            Resource::RequestOrchestration(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Requirements(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ResearchStudy(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ResearchSubject(resource) => {serializer.serialize_any("resource", resource)}
            Resource::RiskAssessment(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Schedule(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SearchParameter(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ServiceRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Slot(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Specimen(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SpecimenDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::StructureDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::StructureMap(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Subscription(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubscriptionStatus(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubscriptionTopic(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Substance(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubstanceDefinition(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubstanceNucleicAcid(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubstancePolymer(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubstanceProtein(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubstanceReferenceInformation(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SubstanceSourceMaterial(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SupplyDelivery(resource) => {serializer.serialize_any("resource", resource)}
            Resource::SupplyRequest(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Task(resource) => {serializer.serialize_any("resource", resource)}
            Resource::TerminologyCapabilities(resource) => {serializer.serialize_any("resource", resource)}
            Resource::TestPlan(resource) => {serializer.serialize_any("resource", resource)}
            Resource::TestReport(resource) => {serializer.serialize_any("resource", resource)}
            Resource::TestScript(resource) => {serializer.serialize_any("resource", resource)}
            Resource::Transport(resource) => {serializer.serialize_any("resource", resource)}
            Resource::ValueSet(resource) => {serializer.serialize_any("resource", resource)}
            Resource::VerificationResult(resource) => {serializer.serialize_any("resource", resource)}
            Resource::VisionPrescription(resource) => {serializer.serialize_any("resource", resource)}
        }
    }
}

impl<'de> Deserialize<'de> for Resource {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {

        pub struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = Resource;
        }

        deserializer.deserialize_enum(AnyVisitor)
    }
}
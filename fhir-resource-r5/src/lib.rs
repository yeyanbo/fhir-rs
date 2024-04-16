mod resource;

use fhir_rs::prelude::*;
pub use resource::*;

// impl PathPart for Patient {
//     fn path(&self, paths: &mut fhir_rs::fhirpath::FhirPaths) -> Result<Collection> {
//         match paths.next() {
//             Some(func) => {
//                 match func.definition.function_name() {
//                     FunctionName::Element => {
//                         match func.params {
//                             FunctionParam::String(name) => {
//                                 match name.as_str() {
//                                     "identifier" => {
//                                         self.identifier.path(paths)
//                                     },
//                                     "name" => {
//                                         self.name.path(paths)
//                                     },
//                                     "gender" => {
//                                         self.gender.path(paths)
//                                     },
//                                     other => Err(FhirError::Message(format!("无效的路径名12:[{}]", other)))
//                                 }
//                             },
//                             _ => unreachable!(),
//                         }
//                     },
//                     _ => Err(FhirError::Message(format!("Patient: 无效的函数名:{:?}", &func))),
//                 }
//             },
//             None => Ok(self.convert()),
//         }
//     }
    
//     fn convert(&self) -> Collection {
//         Collection(vec![Box::new(self.clone())])
//     }
// }

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


impl Executor for AnyResource {
    fn path(&self, paths: &mut FhirPaths) -> Result<Collection> {
        match self {
            AnyResource::Account(resource) => resource.path(paths),
            AnyResource::ActivityDefinition(resource) => resource.path(paths),
            AnyResource::ActorDefinition(resource) => resource.path(paths),
            AnyResource::AdministrableProductDefinition(resource) => resource.path(paths),
            AnyResource::AdverseEvent(resource) => resource.path(paths),
            AnyResource::AllergyIntolerance(resource) => resource.path(paths),
            AnyResource::Appointment(resource) => resource.path(paths),
            AnyResource::AppointmentResponse(resource) => resource.path(paths),
            AnyResource::ArtifactAssessment(resource) => resource.path(paths),
            AnyResource::AuditEvent(resource) => resource.path(paths),
            AnyResource::Basic(resource) => resource.path(paths),
            AnyResource::Binary(resource) => resource.path(paths),
            AnyResource::BiologicallyDerivedProduct(resource) => resource.path(paths),
            AnyResource::BiologicallyDerivedProductDispense(resource) => resource.path(paths),
            AnyResource::BodyStructure(resource) => resource.path(paths),
            AnyResource::Bundle(resource) => resource.path(paths),
            AnyResource::CapabilityStatement(resource) => resource.path(paths),
            AnyResource::CarePlan(resource) => resource.path(paths),
            AnyResource::CareTeam(resource) => resource.path(paths),
            AnyResource::ChargeItem(resource) => resource.path(paths),
            AnyResource::ChargeItemDefinition(resource) => resource.path(paths),
            AnyResource::Citation(resource) => resource.path(paths),
            AnyResource::Claim(resource) => resource.path(paths),
            AnyResource::ClaimResponse(resource) => resource.path(paths),
            AnyResource::ClinicalImpression(resource) => resource.path(paths),
            AnyResource::ClinicalUseDefinition(resource) => resource.path(paths),
            AnyResource::CodeSystem(resource) => resource.path(paths),
            AnyResource::Communication(resource) => resource.path(paths),
            AnyResource::CommunicationRequest(resource) => resource.path(paths),
            AnyResource::CompartmentDefinition(resource) => resource.path(paths),
            AnyResource::Composition(resource) => resource.path(paths),
            AnyResource::ConceptMap(resource) => resource.path(paths),
            AnyResource::Condition(resource) => resource.path(paths),
            AnyResource::ConditionDefinition(resource) => resource.path(paths),
            AnyResource::Consent(resource) => resource.path(paths),
            AnyResource::Contract(resource) => resource.path(paths),
            AnyResource::Coverage(resource) => resource.path(paths),
            AnyResource::CoverageEligibilityRequest(resource) => resource.path(paths),
            AnyResource::CoverageEligibilityResponse(resource) => resource.path(paths),
            AnyResource::DetectedIssue(resource) => resource.path(paths),
            AnyResource::Device(resource) => resource.path(paths),
            AnyResource::DeviceAssociation(resource) => resource.path(paths),
            AnyResource::DeviceDefinition(resource) => resource.path(paths),
            AnyResource::DeviceDispense(resource) => resource.path(paths),
            AnyResource::DeviceMetric(resource) => resource.path(paths),
            AnyResource::DeviceRequest(resource) => resource.path(paths),
            AnyResource::DeviceUsage(resource) => resource.path(paths),
            AnyResource::DiagnosticReport(resource) => resource.path(paths),
            AnyResource::DocumentReference(resource) => resource.path(paths),
            AnyResource::Encounter(resource) => resource.path(paths),
            AnyResource::EncounterHistory(resource) => resource.path(paths),
            AnyResource::Endpoint(resource) => resource.path(paths),
            AnyResource::EnrollmentRequest(resource) => resource.path(paths),
            AnyResource::EnrollmentResponse(resource) => resource.path(paths),
            AnyResource::EpisodeOfCare(resource) => resource.path(paths),
            AnyResource::EventDefinition(resource) => resource.path(paths),
            AnyResource::Evidence(resource) => resource.path(paths),
            AnyResource::EvidenceReport(resource) => resource.path(paths),
            AnyResource::EvidenceVariable(resource) => resource.path(paths),
            AnyResource::ExampleScenario(resource) => resource.path(paths),
            AnyResource::ExplanationOfBenefit(resource) => resource.path(paths),
            AnyResource::FamilyMemberHistory(resource) => resource.path(paths),
            AnyResource::Flag(resource) => resource.path(paths),
            AnyResource::FormularyItem(resource) => resource.path(paths),
            AnyResource::GenomicStudy(resource) => resource.path(paths),
            AnyResource::Goal(resource) => resource.path(paths),
            AnyResource::GraphDefinition(resource) => resource.path(paths),
            AnyResource::Group(resource) => resource.path(paths),
            AnyResource::GuidanceResponse(resource) => resource.path(paths),
            AnyResource::HealthcareService(resource) => resource.path(paths),
            AnyResource::ImagingSelection(resource) => resource.path(paths),
            AnyResource::ImagingStudy(resource) => resource.path(paths),
            AnyResource::Immunization(resource) => resource.path(paths),
            AnyResource::ImmunizationEvaluation(resource) => resource.path(paths),
            AnyResource::ImmunizationRecommendation(resource) => resource.path(paths),
            AnyResource::ImplementationGuide(resource) => resource.path(paths),
            AnyResource::Ingredient(resource) => resource.path(paths),
            AnyResource::InsurancePlan(resource) => resource.path(paths),
            AnyResource::InventoryItem(resource) => resource.path(paths),
            AnyResource::InventoryReport(resource) => resource.path(paths),
            AnyResource::Invoice(resource) => resource.path(paths),
            AnyResource::Library(resource) => resource.path(paths),
            AnyResource::Linkage(resource) => resource.path(paths),
            AnyResource::List(resource) => resource.path(paths),
            AnyResource::Location(resource) => resource.path(paths),
            AnyResource::ManufacturedItemDefinition(resource) => resource.path(paths),
            AnyResource::Measure(resource) => resource.path(paths),
            AnyResource::MeasureReport(resource) => resource.path(paths),
            AnyResource::Medication(resource) => resource.path(paths),
            AnyResource::MedicationAdministration(resource) => resource.path(paths),
            AnyResource::MedicationDispense(resource) => resource.path(paths),
            AnyResource::MedicationKnowledge(resource) => resource.path(paths),
            AnyResource::MedicationRequest(resource) => resource.path(paths),
            AnyResource::MedicationStatement(resource) => resource.path(paths),
            AnyResource::MedicinalProductDefinition(resource) => resource.path(paths),
            AnyResource::MessageDefinition(resource) => resource.path(paths),
            AnyResource::MessageHeader(resource) => resource.path(paths),
            AnyResource::MolecularSequence(resource) => resource.path(paths),
            AnyResource::NamingSystem(resource) => resource.path(paths),
            AnyResource::NutritionIntake(resource) => resource.path(paths),
            AnyResource::NutritionOrder(resource) => resource.path(paths),
            AnyResource::NutritionProduct(resource) => resource.path(paths),
            AnyResource::Observation(resource) => resource.path(paths),
            AnyResource::ObservationDefinition(resource) => resource.path(paths),
            AnyResource::OperationDefinition(resource) => resource.path(paths),
            AnyResource::OperationOutcome(resource) => resource.path(paths),
            AnyResource::Organization(resource) => resource.path(paths),
            AnyResource::OrganizationAffiliation(resource) => resource.path(paths),
            AnyResource::PackagedProductDefinition(resource) => resource.path(paths),
            AnyResource::Parameters(resource) => resource.path(paths),
            AnyResource::Patient(resource) => resource.path(paths),
            AnyResource::PaymentNotice(resource) => resource.path(paths),
            AnyResource::PaymentReconciliation(resource) => resource.path(paths),
            AnyResource::Permission(resource) => resource.path(paths),
            AnyResource::Person(resource) => resource.path(paths),
            AnyResource::PlanDefinition(resource) => resource.path(paths),
            AnyResource::Practitioner(resource) => resource.path(paths),
            AnyResource::PractitionerRole(resource) => resource.path(paths),
            AnyResource::Procedure(resource) => resource.path(paths),
            AnyResource::Provenance(resource) => resource.path(paths),
            AnyResource::Questionnaire(resource) => resource.path(paths),
            AnyResource::QuestionnaireResponse(resource) => resource.path(paths),
            AnyResource::RegulatedAuthorization(resource) => resource.path(paths),
            AnyResource::RelatedPerson(resource) => resource.path(paths),
            AnyResource::RequestOrchestration(resource) => resource.path(paths),
            AnyResource::Requirements(resource) => resource.path(paths),
            AnyResource::ResearchStudy(resource) => resource.path(paths),
            AnyResource::ResearchSubject(resource) => resource.path(paths),
            AnyResource::RiskAssessment(resource) => resource.path(paths),
            AnyResource::Schedule(resource) => resource.path(paths),
            AnyResource::SearchParameter(resource) => resource.path(paths),
            AnyResource::ServiceRequest(resource) => resource.path(paths),
            AnyResource::Slot(resource) => resource.path(paths),
            AnyResource::Specimen(resource) => resource.path(paths),
            AnyResource::SpecimenDefinition(resource) => resource.path(paths),
            AnyResource::StructureDefinition(resource) => resource.path(paths),
            AnyResource::StructureMap(resource) => resource.path(paths),
            AnyResource::Subscription(resource) => resource.path(paths),
            AnyResource::SubscriptionStatus(resource) => resource.path(paths),
            AnyResource::SubscriptionTopic(resource) => resource.path(paths),
            AnyResource::Substance(resource) => resource.path(paths),
            AnyResource::SubstanceDefinition(resource) => resource.path(paths),
            AnyResource::SubstanceNucleicAcid(resource) => resource.path(paths),
            AnyResource::SubstancePolymer(resource) => resource.path(paths),
            AnyResource::SubstanceProtein(resource) => resource.path(paths),
            AnyResource::SubstanceReferenceInformation(resource) => resource.path(paths),
            AnyResource::SubstanceSourceMaterial(resource) => resource.path(paths),
            AnyResource::SupplyDelivery(resource) => resource.path(paths),
            AnyResource::SupplyRequest(resource) => resource.path(paths),
            AnyResource::Task(resource) => resource.path(paths),
            AnyResource::TerminologyCapabilities(resource) => resource.path(paths),
            AnyResource::TestPlan(resource) => resource.path(paths),
            AnyResource::TestReport(resource) => resource.path(paths),
            AnyResource::TestScript(resource) => resource.path(paths),
            AnyResource::Transport(resource) => resource.path(paths),
            AnyResource::ValueSet(resource) => resource.path(paths),
            AnyResource::VerificationResult(resource) => resource.path(paths),
            AnyResource::VisionPrescription(resource) => resource.path(paths),
        }
    }

    fn as_collection(&self) -> Collection {
        match self {
            AnyResource::Account(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ActivityDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ActorDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::AdministrableProductDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::AdverseEvent(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::AllergyIntolerance(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Appointment(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::AppointmentResponse(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ArtifactAssessment(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::AuditEvent(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Basic(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Binary(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::BiologicallyDerivedProduct(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::BiologicallyDerivedProductDispense(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::BodyStructure(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Bundle(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CapabilityStatement(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CarePlan(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CareTeam(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ChargeItem(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ChargeItemDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Citation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Claim(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ClaimResponse(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ClinicalImpression(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ClinicalUseDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CodeSystem(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Communication(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CommunicationRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CompartmentDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Composition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ConceptMap(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Condition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ConditionDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Consent(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Contract(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Coverage(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CoverageEligibilityRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::CoverageEligibilityResponse(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DetectedIssue(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Device(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DeviceAssociation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DeviceDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DeviceDispense(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DeviceMetric(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DeviceRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DeviceUsage(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DiagnosticReport(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::DocumentReference(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Encounter(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EncounterHistory(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Endpoint(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EnrollmentRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EnrollmentResponse(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EpisodeOfCare(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EventDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Evidence(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EvidenceReport(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::EvidenceVariable(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ExampleScenario(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ExplanationOfBenefit(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::FamilyMemberHistory(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Flag(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::FormularyItem(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::GenomicStudy(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Goal(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::GraphDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Group(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::GuidanceResponse(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::HealthcareService(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ImagingSelection(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ImagingStudy(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Immunization(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ImmunizationEvaluation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ImmunizationRecommendation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ImplementationGuide(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Ingredient(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::InsurancePlan(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::InventoryItem(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::InventoryReport(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Invoice(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Library(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Linkage(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::List(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Location(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ManufacturedItemDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Measure(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MeasureReport(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Medication(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MedicationAdministration(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MedicationDispense(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MedicationKnowledge(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MedicationRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MedicationStatement(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MedicinalProductDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MessageDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MessageHeader(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::MolecularSequence(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::NamingSystem(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::NutritionIntake(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::NutritionOrder(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::NutritionProduct(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Observation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ObservationDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::OperationDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::OperationOutcome(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Organization(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::OrganizationAffiliation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::PackagedProductDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Parameters(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Patient(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::PaymentNotice(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::PaymentReconciliation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Permission(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Person(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::PlanDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Practitioner(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::PractitionerRole(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Procedure(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Provenance(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Questionnaire(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::QuestionnaireResponse(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::RegulatedAuthorization(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::RelatedPerson(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::RequestOrchestration(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Requirements(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ResearchStudy(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ResearchSubject(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::RiskAssessment(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Schedule(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SearchParameter(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ServiceRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Slot(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Specimen(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SpecimenDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::StructureDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::StructureMap(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Subscription(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubscriptionStatus(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubscriptionTopic(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Substance(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubstanceDefinition(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubstanceNucleicAcid(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubstancePolymer(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubstanceProtein(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubstanceReferenceInformation(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SubstanceSourceMaterial(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SupplyDelivery(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::SupplyRequest(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Task(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::TerminologyCapabilities(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::TestPlan(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::TestReport(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::TestScript(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::Transport(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::ValueSet(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::VerificationResult(resource) => Collection(vec![Box::new(resource.clone())]),
            AnyResource::VisionPrescription(resource) => Collection(vec![Box::new(resource.clone())]),
        }
    }
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
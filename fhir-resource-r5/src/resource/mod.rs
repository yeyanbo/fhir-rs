mod account;
pub use account::*;
mod activity_definition;
pub use activity_definition::*;
mod actor_definition;
pub use actor_definition::*;
mod administrable_product_definition;
pub use administrable_product_definition::*;
mod adverse_event;
pub use adverse_event::*;
mod allergy_intolerance;
pub use allergy_intolerance::*;
mod appointment;
pub use appointment::*;
mod appointment_response;
pub use appointment_response::*;
mod artifact_assessment;
pub use artifact_assessment::*;
mod audit_event;
pub use audit_event::*;
mod basic;
pub use basic::*;
mod binary;
pub use binary::*;
mod biologically_derived_product;
pub use biologically_derived_product::*;
mod biologically_derived_product_dispense;
pub use biologically_derived_product_dispense::*;
mod body_structure;
pub use body_structure::*;
mod bundle;
pub use bundle::*;
mod capability_statement;
pub use capability_statement::*;
mod care_plan;
pub use care_plan::*;
mod care_team;
pub use care_team::*;
mod charge_item;
pub use charge_item::*;
mod charge_item_definition;
pub use charge_item_definition::*;
mod citation;
pub use citation::*;
mod claim;
pub use claim::*;
mod claim_response;
pub use claim_response::*;
mod clinical_impression;
pub use clinical_impression::*;
mod clinical_use_definition;
pub use clinical_use_definition::*;
mod code_system;
pub use code_system::*;
mod communication;
pub use communication::*;
mod communication_request;
pub use communication_request::*;
mod compartment_definition;
pub use compartment_definition::*;
mod composition;
pub use composition::*;
mod concept_map;
pub use concept_map::*;
mod condition;
pub use condition::*;
mod condition_definition;
pub use condition_definition::*;
mod consent;
pub use consent::*;
mod contract;
pub use contract::*;
mod coverage;
pub use coverage::*;
mod coverage_eligibility_request;
pub use coverage_eligibility_request::*;
mod coverage_eligibility_response;
pub use coverage_eligibility_response::*;
mod detected_issue;
pub use detected_issue::*;
mod device;
pub use device::*;
mod device_association;
pub use device_association::*;
mod device_definition;
pub use device_definition::*;
mod device_dispense;
pub use device_dispense::*;
mod device_metric;
pub use device_metric::*;
mod device_request;
pub use device_request::*;
mod device_usage;
pub use device_usage::*;
mod diagnostic_report;
pub use diagnostic_report::*;
mod document_reference;
pub use document_reference::*;
mod encounter;
pub use encounter::*;
mod encounter_history;
pub use encounter_history::*;
mod endpoint;
pub use endpoint::*;
mod enrollment_request;
pub use enrollment_request::*;
mod enrollment_response;
pub use enrollment_response::*;
mod episode_of_care;
pub use episode_of_care::*;
mod event_definition;
pub use event_definition::*;
mod evidence;
pub use evidence::*;
mod evidence_report;
pub use evidence_report::*;
mod evidence_variable;
pub use evidence_variable::*;
mod example_scenario;
pub use example_scenario::*;
mod explanation_of_benefit;
pub use explanation_of_benefit::*;
mod family_member_history;
pub use family_member_history::*;
mod flag;
pub use flag::*;
mod formulary_item;
pub use formulary_item::*;
mod genomic_study;
pub use genomic_study::*;
mod goal;
pub use goal::*;
mod graph_definition;
pub use graph_definition::*;
mod group;
pub use group::*;
mod guidance_response;
pub use guidance_response::*;
mod healthcare_service;
pub use healthcare_service::*;
mod imaging_selection;
pub use imaging_selection::*;
mod imaging_study;
pub use imaging_study::*;
mod immunization;
pub use immunization::*;
mod immunization_evaluation;
pub use immunization_evaluation::*;
mod immunization_recommendation;
pub use immunization_recommendation::*;
mod implementation_guide;
pub use implementation_guide::*;
mod ingredient;
pub use ingredient::*;
mod insurance_plan;
pub use insurance_plan::*;
mod inventory_item;
pub use inventory_item::*;
mod inventory_report;
pub use inventory_report::*;
mod invoice;
pub use invoice::*;
mod library;
pub use library::*;
mod linkage;
pub use linkage::*;
mod list;
pub use list::*;
mod location;
pub use location::*;
mod manufactured_item_definition;
pub use manufactured_item_definition::*;
mod measure;
pub use measure::*;
mod measure_report;
pub use measure_report::*;
mod medication;
pub use medication::*;
mod medication_administration;
pub use medication_administration::*;
mod medication_dispense;
pub use medication_dispense::*;
mod medication_knowledge;
pub use medication_knowledge::*;
mod medication_request;
pub use medication_request::*;
mod medication_statement;
pub use medication_statement::*;
mod medicinal_product_definition;
pub use medicinal_product_definition::*;
mod message_definition;
pub use message_definition::*;
mod message_header;
pub use message_header::*;
mod molecular_sequence;
pub use molecular_sequence::*;
mod naming_system;
pub use naming_system::*;
mod nutrition_intake;
pub use nutrition_intake::*;
mod nutrition_order;
pub use nutrition_order::*;
mod nutrition_product;
pub use nutrition_product::*;
mod observation;
pub use observation::*;
mod observation_definition;
pub use observation_definition::*;
mod operation_definition;
pub use operation_definition::*;
mod operation_outcome;
pub use operation_outcome::*;
mod organization;
pub use organization::*;
mod organization_affiliation;
pub use organization_affiliation::*;
mod packaged_product_definition;
pub use packaged_product_definition::*;
mod parameters;
pub use parameters::*;
mod patient;
pub use patient::*;
mod payment_notice;
pub use payment_notice::*;
mod payment_reconciliation;
pub use payment_reconciliation::*;
mod permission;
pub use permission::*;
mod person;
pub use person::*;
mod plan_definition;
pub use plan_definition::*;
mod practitioner;
pub use practitioner::*;
mod practitioner_role;
pub use practitioner_role::*;
mod procedure;
pub use procedure::*;
mod provenance;
pub use provenance::*;
mod questionnaire;
pub use questionnaire::*;
mod questionnaire_response;
pub use questionnaire_response::*;
mod regulated_authorization;
pub use regulated_authorization::*;
mod related_person;
pub use related_person::*;
mod request_orchestration;
pub use request_orchestration::*;
mod requirements;
pub use requirements::*;
mod research_study;
pub use research_study::*;
mod research_subject;
pub use research_subject::*;
mod risk_assessment;
pub use risk_assessment::*;
mod schedule;
pub use schedule::*;
mod search_parameter;
pub use search_parameter::*;
mod service_request;
pub use service_request::*;
mod slot;
pub use slot::*;
mod specimen;
pub use specimen::*;
mod specimen_definition;
pub use specimen_definition::*;
mod structure_definition;
pub use structure_definition::*;
mod structure_map;
pub use structure_map::*;
mod subscription;
pub use subscription::*;
mod subscription_status;
pub use subscription_status::*;
mod subscription_topic;
pub use subscription_topic::*;
mod substance;
pub use substance::*;
mod substance_definition;
pub use substance_definition::*;
mod substance_nucleic_acid;
pub use substance_nucleic_acid::*;
mod substance_polymer;
pub use substance_polymer::*;
mod substance_protein;
pub use substance_protein::*;
mod substance_reference_information;
pub use substance_reference_information::*;
mod substance_source_material;
pub use substance_source_material::*;
mod supply_delivery;
pub use supply_delivery::*;
mod supply_request;
pub use supply_request::*;
mod task;
pub use task::*;
mod terminology_capabilities;
pub use terminology_capabilities::*;
mod test_plan;
pub use test_plan::*;
mod test_report;
pub use test_report::*;
mod test_script;
pub use test_script::*;
mod transport;
pub use transport::*;
mod value_set;
pub use value_set::*;
mod verification_result;
pub use verification_result::*;
mod vision_prescription;
pub use vision_prescription::*;

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

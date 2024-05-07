use std::cell::{Cell, RefCell};
use std::collections::{HashMap, HashSet};
use crate::prelude::*;

#[derive(Debug)]
pub enum SlicingType {
    TYP,
    VAL,
    OTH,
}

impl From<String> for SlicingType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "type" => Self::TYP,
            "value" => Self::VAL,
            _ => Self::OTH,
        }
    }
}

#[derive(Debug)]
pub struct Slicing {
    pub typ: SlicingType,
    pub path: String,
}

#[derive(Debug)]
pub struct Validator {
    pub root: ElementDefinition,
    pub elements: Vec<ElementDefinition>,
    pub current: Cell<usize>,
    pub slicing: RefCell<HashMap<String, Slicing>>,
    pub empty_collection: RefCell<HashSet<String>>,
}

impl Validator {

    pub fn new(profile:StructureDefinition) -> Self {
        match profile.snapshot {
            Some(snapshot) => {
                match snapshot.element {
                    Some(mut elements) => {
                        let root = elements.remove(0);

                        Self {
                            root,
                            elements,
                            current: Cell::new(0),
                            slicing: RefCell::new(HashMap::new()),
                            empty_collection : RefCell::new(HashSet::new()),
                        }
                    },
                    None => unreachable!(),
                }
            },
            None => unreachable!(),
        }
    }

    /// 校验整个资源的入口函数
    /// 异常情况表示是Profile的问题，资源的问题则输出到OperationOutcome中
    pub fn validate<R: Resource + Executor>(&mut self, resource: &R) -> Result<ValidateResult> {
        let mut validate_result = ValidateResult::new();

        while let Some(element) = self.next() {
            let rss: Vec<ValidateResultItem> = self.validate_element(resource, element)?;
            validate_result.add_result_item(rss);
        }

        Ok(validate_result)
    }

    pub fn validate_any_resource(&mut self, resource: &AnyResource) -> Result<ValidateResult> {
        match resource {
            AnyResource::Account(resource) => self.validate(resource),
            AnyResource::ActivityDefinition(resource) => self.validate(resource),
            AnyResource::ActorDefinition(resource) => self.validate(resource),
            AnyResource::AdministrableProductDefinition(resource) => self.validate(resource),
            AnyResource::AdverseEvent(resource) => self.validate(resource),
            AnyResource::AllergyIntolerance(resource) => self.validate(resource),
            AnyResource::Appointment(resource) => self.validate(resource),
            AnyResource::AppointmentResponse(resource) => self.validate(resource),
            AnyResource::ArtifactAssessment(resource) => self.validate(resource),
            AnyResource::AuditEvent(resource) => self.validate(resource),
            AnyResource::Basic(resource) => self.validate(resource),
            AnyResource::Binary(resource) => self.validate(resource),
            AnyResource::BiologicallyDerivedProduct(resource) => self.validate(resource),
            AnyResource::BiologicallyDerivedProductDispense(resource) => self.validate(resource),
            AnyResource::BodyStructure(resource) => self.validate(resource),
            AnyResource::Bundle(resource) => self.validate(resource),
            AnyResource::CapabilityStatement(resource) => self.validate(resource),
            AnyResource::CarePlan(resource) => self.validate(resource),
            AnyResource::CareTeam(resource) => self.validate(resource),
            AnyResource::ChargeItem(resource) => self.validate(resource),
            AnyResource::ChargeItemDefinition(resource) => self.validate(resource),
            AnyResource::Citation(resource) => self.validate(resource),
            AnyResource::Claim(resource) => self.validate(resource),
            AnyResource::ClaimResponse(resource) => self.validate(resource),
            AnyResource::ClinicalImpression(resource) => self.validate(resource),
            AnyResource::ClinicalUseDefinition(resource) => self.validate(resource),
            AnyResource::CodeSystem(resource) => self.validate(resource),
            AnyResource::Communication(resource) => self.validate(resource),
            AnyResource::CommunicationRequest(resource) => self.validate(resource),
            AnyResource::CompartmentDefinition(resource) => self.validate(resource),
            AnyResource::Composition(resource) => self.validate(resource),
            AnyResource::ConceptMap(resource) => self.validate(resource),
            AnyResource::Condition(resource) => self.validate(resource),
            AnyResource::ConditionDefinition(resource) => self.validate(resource),
            AnyResource::Consent(resource) => self.validate(resource),
            AnyResource::Contract(resource) => self.validate(resource),
            AnyResource::Coverage(resource) => self.validate(resource),
            AnyResource::CoverageEligibilityRequest(resource) => self.validate(resource),
            AnyResource::CoverageEligibilityResponse(resource) => self.validate(resource),
            AnyResource::DetectedIssue(resource) => self.validate(resource),
            AnyResource::Device(resource) => self.validate(resource),
            AnyResource::DeviceAssociation(resource) => self.validate(resource),
            AnyResource::DeviceDefinition(resource) => self.validate(resource),
            AnyResource::DeviceDispense(resource) => self.validate(resource),
            AnyResource::DeviceMetric(resource) => self.validate(resource),
            AnyResource::DeviceRequest(resource) => self.validate(resource),
            AnyResource::DeviceUsage(resource) => self.validate(resource),
            AnyResource::DiagnosticReport(resource) => self.validate(resource),
            AnyResource::DocumentReference(resource) => self.validate(resource),
            AnyResource::Encounter(resource) => self.validate(resource),
            AnyResource::EncounterHistory(resource) => self.validate(resource),
            AnyResource::Endpoint(resource) => self.validate(resource),
            AnyResource::EnrollmentRequest(resource) => self.validate(resource),
            AnyResource::EnrollmentResponse(resource) => self.validate(resource),
            AnyResource::EpisodeOfCare(resource) => self.validate(resource),
            AnyResource::EventDefinition(resource) => self.validate(resource),
            AnyResource::Evidence(resource) => self.validate(resource),
            AnyResource::EvidenceReport(resource) => self.validate(resource),
            AnyResource::EvidenceVariable(resource) => self.validate(resource),
            AnyResource::ExampleScenario(resource) => self.validate(resource),
            AnyResource::ExplanationOfBenefit(resource) => self.validate(resource),
            AnyResource::FamilyMemberHistory(resource) => self.validate(resource),
            AnyResource::Flag(resource) => self.validate(resource),
            AnyResource::FormularyItem(resource) => self.validate(resource),
            AnyResource::GenomicStudy(resource) => self.validate(resource),
            AnyResource::Goal(resource) => self.validate(resource),
            AnyResource::GraphDefinition(resource) => self.validate(resource),
            AnyResource::Group(resource) => self.validate(resource),
            AnyResource::GuidanceResponse(resource) => self.validate(resource),
            AnyResource::HealthcareService(resource) => self.validate(resource),
            AnyResource::ImagingSelection(resource) => self.validate(resource),
            AnyResource::ImagingStudy(resource) => self.validate(resource),
            AnyResource::Immunization(resource) => self.validate(resource),
            AnyResource::ImmunizationEvaluation(resource) => self.validate(resource),
            AnyResource::ImmunizationRecommendation(resource) => self.validate(resource),
            AnyResource::ImplementationGuide(resource) => self.validate(resource),
            AnyResource::Ingredient(resource) => self.validate(resource),
            AnyResource::InsurancePlan(resource) => self.validate(resource),
            AnyResource::InventoryItem(resource) => self.validate(resource),
            AnyResource::InventoryReport(resource) => self.validate(resource),
            AnyResource::Invoice(resource) => self.validate(resource),
            AnyResource::Library(resource) => self.validate(resource),
            AnyResource::Linkage(resource) => self.validate(resource),
            AnyResource::List(resource) => self.validate(resource),
            AnyResource::Location(resource) => self.validate(resource),
            AnyResource::ManufacturedItemDefinition(resource) => self.validate(resource),
            AnyResource::Measure(resource) => self.validate(resource),
            AnyResource::MeasureReport(resource) => self.validate(resource),
            AnyResource::Medication(resource) => self.validate(resource),
            AnyResource::MedicationAdministration(resource) => self.validate(resource),
            AnyResource::MedicationDispense(resource) => self.validate(resource),
            AnyResource::MedicationKnowledge(resource) => self.validate(resource),
            AnyResource::MedicationRequest(resource) => self.validate(resource),
            AnyResource::MedicationStatement(resource) => self.validate(resource),
            AnyResource::MedicinalProductDefinition(resource) => self.validate(resource),
            AnyResource::MessageDefinition(resource) => self.validate(resource),
            AnyResource::MessageHeader(resource) => self.validate(resource),
            AnyResource::MolecularSequence(resource) => self.validate(resource),
            AnyResource::NamingSystem(resource) => self.validate(resource),
            AnyResource::NutritionIntake(resource) => self.validate(resource),
            AnyResource::NutritionOrder(resource) => self.validate(resource),
            AnyResource::NutritionProduct(resource) => self.validate(resource),
            AnyResource::Observation(resource) => self.validate(resource),
            AnyResource::ObservationDefinition(resource) => self.validate(resource),
            AnyResource::OperationDefinition(resource) => self.validate(resource),
            AnyResource::OperationOutcome(resource) => self.validate(resource),
            AnyResource::Organization(resource) => self.validate(resource),
            AnyResource::OrganizationAffiliation(resource) => self.validate(resource),
            AnyResource::PackagedProductDefinition(resource) => self.validate(resource),
            AnyResource::Parameters(resource) => self.validate(resource),
            AnyResource::Patient(resource) => self.validate(resource),
            AnyResource::PaymentNotice(resource) => self.validate(resource),
            AnyResource::PaymentReconciliation(resource) => self.validate(resource),
            AnyResource::Permission(resource) => self.validate(resource),
            AnyResource::Person(resource) => self.validate(resource),
            AnyResource::PlanDefinition(resource) => self.validate(resource),
            AnyResource::Practitioner(resource) => self.validate(resource),
            AnyResource::PractitionerRole(resource) => self.validate(resource),
            AnyResource::Procedure(resource) => self.validate(resource),
            AnyResource::Provenance(resource) => self.validate(resource),
            AnyResource::Questionnaire(resource) => self.validate(resource),
            AnyResource::QuestionnaireResponse(resource) => self.validate(resource),
            AnyResource::RegulatedAuthorization(resource) => self.validate(resource),
            AnyResource::RelatedPerson(resource) => self.validate(resource),
            AnyResource::RequestOrchestration(resource) => self.validate(resource),
            AnyResource::Requirements(resource) => self.validate(resource),
            AnyResource::ResearchStudy(resource) => self.validate(resource),
            AnyResource::ResearchSubject(resource) => self.validate(resource),
            AnyResource::RiskAssessment(resource) => self.validate(resource),
            AnyResource::Schedule(resource) => self.validate(resource),
            AnyResource::SearchParameter(resource) => self.validate(resource),
            AnyResource::ServiceRequest(resource) => self.validate(resource),
            AnyResource::Slot(resource) => self.validate(resource),
            AnyResource::Specimen(resource) => self.validate(resource),
            AnyResource::SpecimenDefinition(resource) => self.validate(resource),
            AnyResource::StructureDefinition(resource) => self.validate(resource),
            AnyResource::StructureMap(resource) => self.validate(resource),
            AnyResource::Subscription(resource) => self.validate(resource),
            AnyResource::SubscriptionStatus(resource) => self.validate(resource),
            AnyResource::SubscriptionTopic(resource) => self.validate(resource),
            AnyResource::Substance(resource) => self.validate(resource),
            AnyResource::SubstanceDefinition(resource) => self.validate(resource),
            AnyResource::SubstanceNucleicAcid(resource) => self.validate(resource),
            AnyResource::SubstancePolymer(resource) => self.validate(resource),
            AnyResource::SubstanceProtein(resource) => self.validate(resource),
            AnyResource::SubstanceReferenceInformation(resource) => self.validate(resource),
            AnyResource::SubstanceSourceMaterial(resource) => self.validate(resource),
            AnyResource::SupplyDelivery(resource) => self.validate(resource),
            AnyResource::SupplyRequest(resource) => self.validate(resource),
            AnyResource::Task(resource) => self.validate(resource),
            AnyResource::TerminologyCapabilities(resource) => self.validate(resource),
            AnyResource::TestPlan(resource) => self.validate(resource),
            AnyResource::TestReport(resource) => self.validate(resource),
            AnyResource::TestScript(resource) => self.validate(resource),
            AnyResource::Transport(resource) => self.validate(resource),
            AnyResource::ValueSet(resource) => self.validate(resource),
            AnyResource::VerificationResult(resource) => self.validate(resource),
            AnyResource::VisionPrescription(resource) => self.validate(resource),
        }
    }

    fn validate_element<R: Resource + Executor>(&self, resource: &R, element: &ElementDefinition) -> Result<Vec<ValidateResultItem>> {
        if self.is_slice_element(&element) {
            self.validate_slice_element(resource, element)
        } else {
            self.validate_non_slice_element(resource, element)
        }    
    }

    fn validate_non_slice_element(&self, resource: &dyn Executor, element: &ElementDefinition) -> Result<Vec<ValidateResultItem>> {
        let mut rss = vec![];

        match &element.path {
            Some(path) => {
                let path = path.value.clone().unwrap();
                println!("path => {}", &path);

                let mut empty_collection = self.empty_collection.borrow_mut();
                for col in &*empty_collection {
                    if path.starts_with(col) {return Ok(rss)}
                }

                let expr = Expr::parse(path.clone())?;
                let collection = expr.eval(resource)?;

                // 最小值约束
                if let Some(min) = &element.min {
                    let min = min.value.unwrap();    
                    if collection.count() < min {
                        rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &path, format!("低于期望的最小值[{}]", min)))
                    } else {
                        rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &path, format!("符合期望的最小值[{}]", min)))
                    }
                }

                // 最大值约束
                if let Some(max) = &element.max {
                    let max = max.clone().value.unwrap();
                    let max = match max.as_str() {
                        "*" => usize::max_value(),
                        other => {
                            match other.parse() {
                                Ok(number) => number,
                                Err(_) => return Err(FhirError::Message(format!("最大值不是有效的数值[{}]", other))),
                            }
                        }
                    };

                    if collection.count() > max {
                        rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &path, format!("大于期望的最大值[{}]", max)))
                    } else {
                        rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &path, format!("符合期望的最大值[{}]", max)))
                    }
                }

                // 如果存在约束，则执行约束
                // 目前存在几种约束：
                // 1. dom-x: 这种约束针对整个资源
                // 2. ele-x: 这种约束针对的是对应的元素
                if let Some(constraints) = &element.constraint {
                    for constraint in constraints {
                        if let Some(key) = &constraint.key {
                            let id = key.clone().value.unwrap();
                            match id.as_str() {
                                "ele-1" => {
                                    if self.constraint_ele_1(&collection) {
                                        rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &path, "违反约束[ele-1], 元素不能为空。".to_string()));
                                    } else {
                                        rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &path, "符合约束[ele-1]要求。".to_string()));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // 如果存在Slicing元素，其中定义了切片的规则（过滤条件）
                if let Some(slicing) = &element.slicing {
                    if let Some(discriminators) = &slicing.discriminator {
                        let discriminator = &discriminators[0];
                        let typ = discriminator.type_.clone().unwrap().value.unwrap();
                        let filter_path = discriminator.path.clone().unwrap().value.unwrap();
                        
                        self.push_slicing(path.clone(), Slicing{typ: typ.into(), path: filter_path});
                    }
                }

                // binding
                // 缺少术语系统支持，暂时无法实现对binding的验证
                if let Some(_binding) = &element.binding {
                    rss.push(ValidateResultItem::new(ValidateStatus::Skip, &path, &path, "系统暂时不支持值域验证。".to_string()));
                }

                if collection.count() == 0 {
                    empty_collection.insert(path);
                }

                Ok(rss)
            },
            None => Err(FhirError::error("StructureDefinition.element中未指定path元素")),
        }
    }

    fn validate_slice_element(&self, resource: &dyn Executor, element: &ElementDefinition) -> Result<Vec<ValidateResultItem>> {
        let mut rss = vec![];

        match &element.path {
            Some(path) => {
                let path = path.value.clone().unwrap();
                println!("path => {}", &path);

                // 如果存在SliceName,则要首先找到过滤条件，然后才能对过滤后的Collection进行验证
                match &element.slice_name {
                    Some(slice_name) => {
                        // 根据SliceName查找过滤条件对应的值。（过滤条件在Slicing数组中)
                        let slice_name = slice_name.clone().value.unwrap();
                        let (key, value) = self.lookup_filter(&path, &slice_name)?;

                        let path_exec = match value {
                            AnyType::String(val) => format!("{}.where({} = '{}')", &path, &key, &val),
                            AnyType::Uri(val) => format!("{}.where({} = '{}')", &path, &key, &val),
                            AnyType::Coding(val) => {
                                format!("{path}.where({key}.system = '{}' and {key}.code = '{}')", &val.system.unwrap(), &val.code.unwrap())
                            },
                            AnyType::CodeableConcept(val) => {
                                if let Some(vec) = val.coding {
                                    let coding = &vec[0];
                                    format!("{path}.where({key}.coding.system = '{}' and {key}.coding.code = '{}')", &coding.system.clone().unwrap(), &coding.code.clone().unwrap())
                                } else if let Some(text) = val.text{
                                    format!("{path}.where({key} = '{}')", &text)
                                } else {
                                    return Err(FhirError::Message(format!("+dfd fdfsdfs")))
                                }
                            },
                            _ => unimplemented!(),
                        };

                        info!("Slicing filter path => {}", &path_exec);
                        let expr = Expr::parse(path_exec.clone())?;
                        let collection = expr.eval(resource)?;

                        // 最小值约束
                        if let Some(min) = &element.min {
                            let min = min.value.unwrap();
                            if collection.count() < min {
                                rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &path_exec, format!("低于期望的最小值[{}]", min)))
                            } else {
                                rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &path_exec, format!("符合期望的最小值[{}]", min)))
                            }
                        }

                        // 最大值约束
                        if let Some(max) = &element.max {
                            let max = max.clone().value.unwrap();
                            let max = match max.as_str() {
                                "*" => usize::max_value(),
                                other => {
                                    match other.parse() {
                                        Ok(number) => number,
                                        Err(_) => return Err(FhirError::Message(format!("最大值不是有效的数值[{}]", other))),
                                    }
                                }
                            };

                            if collection.count() > max {
                                rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &path_exec, format!("不符合期望的最大值[{}]", max)))
                            } else {
                                rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &path_exec, format!("符合期望的最大值[{}]", max)))
                            }
                        }

                        let rs = self.validate_slicing_element_with_collection(resource, &path, &slice_name, &path_exec)?;
                        rss.extend(rs);
                    },
                    // 所有的切片元素，都应该由递归函数处理，理论上不会到达这里
                    None => unreachable!(),
                }

                Ok(rss)
            }
            None => Err(FhirError::error("")),
        }
    }

    fn validate_slicing_element_with_collection(&self, resource: &dyn Executor, path: &String, slice_name: &String, path_exec: &String) -> Result<Vec<ValidateResultItem>> {
        let mut rss = vec![];

        while let Some(element) = self.next() {
            let root = format!("{}:{}", &path, &slice_name);

            match &element.id {
                Some(id) => {
                    if !id.starts_with(&root) {
                        break;
                    }

                    let other = id.replace(&root, &path_exec);
                    println!("slice path => {}", &other);

                    let expr = Expr::parse(other.clone())?;
                    let resp = expr.eval(resource)?;

                    // 最小值约束
                    if let Some(min) = &element.min {
                        let min = min.value.unwrap();
                        if resp.count() < min {
                            rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &other, format!("低于期望的最小值[{}]", min)))
                        } else {
                            rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &other, format!("符合期望的最小值[{}]", min)))
                        }
                    }

                    // 最大值约束
                    if let Some(max) = &element.max {
                        let max = max.clone().value.unwrap();
                        let max = match max.as_str() {
                            "*" => usize::max_value(),
                            other => {
                                match other.parse() {
                                    Ok(number) => number,
                                    Err(_) => return Err(FhirError::Message(format!("最大值不是有效的数值[{}]", other))),
                                }
                            }
                        };

                        if resp.count() > max {
                            rss.push(ValidateResultItem::new(ValidateStatus::Error, &path, &other, format!("大于期望的最大值[{}]", max)))
                        } else {
                            rss.push(ValidateResultItem::new(ValidateStatus::Success, &path, &other, format!("符合期望的最大值[{}]", max)))
                        }
                    }
                },
                None => unreachable!(),
            }
        }
        Ok(rss)
    }

    /// 是否违反了约束ele-1
    fn constraint_ele_1(&self, collection: &Collection) -> bool {
        collection.first().is_empty()
    }

    fn is_slice_element(&self, element: &ElementDefinition) -> bool {
        match &element.id {
            Some(id) => {
                id.contains(":")
            },
            None => unreachable!(),
        }
    }

    pub fn next(&self) -> Option<&ElementDefinition> {
        let current = self.current.get();
        if current < self.elements.len() {
            let rs = self.elements.get(current);
            self.current.set(current + 1);
            rs
        } else {
            None
        }
    }

    fn lookup_filter(&self, path: &String, slice_name: &String) -> Result<(String, AnyType)> {
        let slicings = self.slicing.borrow();
        let root = format!("{}:{}", &path, &slice_name);

        match slicings.get(&path.clone()) {
            Some(slicing) => {
                match slicing.typ {
                    SlicingType::TYP => todo!(),
                    SlicingType::VAL => {
                        match self.lookup_by_path(&root, &slicing.path) {
                            Some(element) => {
                                match &element.pattern {
                                    Some(pattern) => Ok((slicing.path.clone(), pattern.clone())),
                                    None => return Err(FhirError::error("在定义中没有找到限定值pattern元素")),
                                }
                            },
                            None => return Err(FhirError::error("找不到SliceName的限定条件")),
                        }
                    },
                    SlicingType::OTH => todo!(),
                }
            },
            None => return Err(FhirError::Message(format!("未找到切片[{}]需要的Slicing信息", &slice_name))),
        }
    }

    /// 处理Slice时，查找Slice的过滤条件
    /// Encounter.identifier:BID.system
    /// 其中： 
    /// root = Encounter.identifier:BID
    /// path = system(来自于slicing.discriminator.path(by value))
    pub fn lookup_by_path(&self, root: &String, path: &String) -> Option<&ElementDefinition> {
        let mut current = self.current.get();
        while let Some(element) = self.elements.get(current) {
            if let Some(id) = &element.id {
                debug!("look up at {}: id = {}", current, &id);
                if id.starts_with(root) {
                    let lookup_id = format!("{root}.{path}");
                    if id == &lookup_id {return Some(element)} else {current += 1}
                } else {
                    break
                }
            }
        }

        None
    }

    fn push_slicing(&self, path: String, slicing: Slicing) {
        let mut slicings = self.slicing.borrow_mut();
        slicings.insert(path, slicing);
    }
}

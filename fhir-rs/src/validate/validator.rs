use super::result::{ValidateResult, ValidateResultItem};
use crate::prelude::*;

#[derive(Debug)]
pub struct Validator {
    profile: StructureDefinition,
    slices: Vec<String>,
}

impl Validator {

    fn new(profile:StructureDefinition) -> Self {
        Self {
            profile,
            slices: vec![],
        }
    }

    fn validate<R: Resource>(&self, resource: &R) -> Result<OperationOutcome> {
        let mut validate_result = ValidateResult::new();

        if let Some(snapshot) = &self.profile.snapshot {
            if let Some(list) = &snapshot.element {
                for element in list {
                    validate_result.add_result_item(self.validate_element(resource, element )?);
                }  
            }
        }

        Ok(validate_result.into())
    }

    fn validate_element<R: Resource>(&self, resource: &R, element: &ElementDefinition) -> Result<Vec<ValidateResultItem>> {
        let mut rss = vec![];

        match &element.path {
            Some(path) => {
                let path = path.value.clone().unwrap();
                let collection = resource.path(path.clone())?;

                if let Some(min) = &element.min {
                    let min = min.value.unwrap();    
                    if collection.count() < min {
                        rss.push(ValidateResultItem::new("error", &path, format!("低于期望的最小值[{}]", min)))
                    } 
                }

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
                        rss.push(ValidateResultItem::new("error", &path, format!("大于期望的最大值[{}]", max)))
                    }
                }

                if let Some(constraints) = &element.constraint {
                    for constraint in constraints {
                        if let Some(key) = &constraint.key {
                            let id = key.clone().value.unwrap();
                            match id.as_str() {
                                "ele-1" => {
                                    if self.constraint_ele_1(&collection) {
                                        rss.push(ValidateResultItem::new("error", &path, format!("违反了约束[ele-1], 元素不能为空。")));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }

                // type 能够解析到实例，就说明type没有问题

                // binding
                // 缺少术语系统支持，暂时无法实现对binding的验证

                Ok(rss)
            },
            None => Err(FhirError::error("")),
        }
    }

    /// 是否违反了约束ele-1
    fn constraint_ele_1(&self, collection: &Collection) -> bool {
        match collection.0.get(0) {
            Some(val) => val.is_empty(),
            None => false,
        }
    }
}

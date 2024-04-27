use std::cell::Cell;
use std::collections::HashMap;

use super::result::{ValidateResult, ValidateResultItem};
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
pub struct Slice {
    pub typ: String,
    pub root: String,
    pub min: Option<usize>,
    pub max: Option<usize>,
}

#[derive(Debug, Default)]
pub struct Context {
    pub slicing: HashMap<String, Slicing>,
    pub slices: HashMap<String, Slice>,
}

#[derive(Debug)]
pub struct Validator {
    pub root: ElementDefinition,
    pub elements: Vec<ElementDefinition>,
    pub current: Cell<usize>,
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
                        }
                    },
                    None => unreachable!(),
                }
            },
            None => unreachable!(),
        }
    }

    pub fn validate<R: Resource + Executor>(&mut self, resource: &R) -> Result<OperationOutcome> {
        let mut validate_result = ValidateResult::new();
        let mut context: Context = Context::default();

        while let Some(element) = self.next() {
            let rss: Vec<ValidateResultItem> = self.validate_element(resource, element, &mut context)?;
            validate_result.add_result_item(rss);
        }

        Ok(validate_result.into())
    }

    fn validate_element<R: Resource + Executor>(&self, resource: &R, element: &ElementDefinition, context: &mut Context) -> Result<Vec<ValidateResultItem>> {
        if self.is_slice_element(&element) {
            self.validate_slice_element(resource, element, context)
        } else {
            self.validate_non_slice_element(resource, element, context)
        }    
    }

    fn validate_non_slice_element<R: Resource + Executor>(&self, resource: &R, element: &ElementDefinition, context: &mut Context) -> Result<Vec<ValidateResultItem>> {
        let mut rss = vec![];

        match &element.path {
            Some(path) => {
                let path = path.value.clone().unwrap();
                let mut expr = PathExpression::parse(path.clone())?;
                let collection = resource.path(&mut expr)?;

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

                if let Some(slicing) = &element.slicing {
                    if let Some(discriminators) = &slicing.discriminator {
                        let discriminator = &discriminators[0];
                        let typ = discriminator.type_.clone().unwrap().value.unwrap();
                        let filter_path = discriminator.path.clone().unwrap().value.unwrap();
                        
                        context.slicing.insert(path.clone(), Slicing{typ: typ.into(), path: filter_path});
                    }
                }

                // binding
                // 缺少术语系统支持，暂时无法实现对binding的验证

                Ok(rss)
            },
            None => Err(FhirError::error("StructureDefinition.element中未指定path元素")),
        }
    }

    fn validate_slice_element<R: Resource + Executor>(&self, resource: &R, element: &ElementDefinition, context: &mut Context) -> Result<Vec<ValidateResultItem>> {
        let mut rss = vec![];

        match &element.path {
            Some(path) => {
                let path = path.value.clone().unwrap();

                match &element.slice_name {
                    Some(slice_name) => {
                        let slice_name = slice_name.clone().value.unwrap();
                        let root = format!("{}:{}", &path, slice_name);

                        match context.slicing.get(&path) {
                            Some(slicing) => {
                                match slicing.typ {
                                    SlicingType::TYP => todo!(),
                                    SlicingType::VAL => {
                                        match self.lookup_by_path(&root, &slicing.path) {
                                            Some(element) => {
                                                match &element.pattern {
                                                    Some(pattern) => {

                                                    },
                                                    None => return Err(FhirError::error("在定义中没有找到限定值pattern元素")),
                                                }
                                            },
                                            None => return Err(FhirError::error("找不到SliceName的限定条件")),
                                        }
                                    },
                                    SlicingType::OTH => todo!(),
                                }
                            },
                            None => return Err(FhirError::Message(format!("未找到切片[{}]指定的Slicing信息", &slice_name))),
                        }
 

                    },
                    // 所有的切片元素，都应该由递归函数处理，理论上不会到达这里
                    None => unreachable!(),
                }          
                let mut expr = PathExpression::parse(path)?;
                let collection = resource.path(&mut expr)?;

                if let Some(min) = &element.min {
                    let min = min.value.unwrap();     
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
                }



                // type 能够解析到实例，就说明type没有问题

                // binding
                // 缺少术语系统支持，暂时无法实现对binding的验证

                Ok(rss)
            }
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

    /// 处理Slice时，查找Slice的过滤条件
    /// Encounter.identifier:BID.system
    /// 其中： 
    /// root = Encounter.identifier:BID
    /// path = system(来自于slicing.discriminator.path(by value))
    pub fn lookup_by_path(&self, root: &String, path: &String) -> Option<&ElementDefinition> {
        let mut current = self.current.get();
        while let Some(element) = self.elements.get(current) {
            if let Some(id) = &element.id {
                println!("look up at {}: id = {}", current, &id);
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
}

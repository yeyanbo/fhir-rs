use super::*;

#[derive(Debug)]
pub struct InvalidFunction(String);

impl From<InvalidFunction> for FhirError {
    fn from(value: InvalidFunction) -> Self {
        FhirError::Message(value.0)
    }
}

#[derive(Debug, Clone)]
pub enum FunctionParam {
    None,
    String(String),
    Integer(isize),
    Expression,
    Collection,
    Vec(Vec<FunctionParam>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionResponse {
    Collection,
    Bool,
    Integer,
    String,
    DateTime,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub definition: FunctionDefinition,
    pub params: FunctionParam,
}

impl Function {

    pub fn from_string(token: String, params: FunctionParam) -> std::result::Result<Self, InvalidFunction> {
        let definition = token.try_into()?; 
        Ok(Function{definition, params})
    }

    pub fn from_definition(definition: FunctionDefinition, params: FunctionParam) -> Self {
        Function{definition, params}
    }

    pub fn create_self_element() -> Self {
        Self::from_definition(FunctionDefinition::ELEMENT, FunctionParam::String("Self".to_string()))
    }

    pub fn match_resource_type_name(&self, type_name: &String) -> bool {
        let self_name = "Self".to_string();

        match self.definition.function_name() {
            FunctionName::Element => {
                match &self.params {
                    FunctionParam::String(name) if (type_name == name) | (type_name == &self_name) => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_resource_type_element(&self) -> bool {
        match self.definition.function_name() {
            FunctionName::Element => {
                match &self.params {
                    FunctionParam::String(name) if Self::is_first_char_upper_case(name) => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    fn is_first_char_upper_case(input: &String) -> bool {
        input.chars().next().map_or(false, |c| c.is_uppercase())
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition(FunctionName, u8, FunctionResponse);

impl FunctionDefinition {

    /// 返回函数的返回值类型
    /// 可以在语法解析阶段判断表达式最后的返回值类型
    pub fn function_name(&self) -> &FunctionName {
        &self.0
    }

    /// 返回函数的返回值类型
    /// 可以在语法解析阶段判断表达式最后的返回值类型
    pub fn response(&self) -> &FunctionResponse {
        &self.2
    }

    pub const ELEMENT: FunctionDefinition = FunctionDefinition(FunctionName::Element, 1, FunctionResponse::Collection);
    pub const CHILD: FunctionDefinition = FunctionDefinition(FunctionName::Child, 1, FunctionResponse::Collection);
    pub const EMPTY: FunctionDefinition = FunctionDefinition(FunctionName::Empty, 0, FunctionResponse::Bool);
    pub const COUNT: FunctionDefinition = FunctionDefinition(FunctionName::Count, 0, FunctionResponse::Integer);
    pub const ALLTRUE: FunctionDefinition = FunctionDefinition(FunctionName::AllTrue, 0, FunctionResponse::Bool);
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionName {
    Element,
    Child,
    Exist,
    Count,
    Empty,
    Single,
    Where,
    Other,
    AllTrue,
}

impl TryFrom<String> for FunctionDefinition {
    type Error = InvalidFunction;

    fn try_from(value: String) -> std::prelude::v1::Result<Self, Self::Error> {
        match value.as_str() {
            "child" => Ok(FunctionDefinition::CHILD),
            "element" => Ok(FunctionDefinition::ELEMENT),
            "empty" => Ok(FunctionDefinition::EMPTY),
            "count" => Ok(FunctionDefinition::COUNT),
            "allTrue" => Ok(FunctionDefinition::ALLTRUE),
            other => Err(InvalidFunction(format!("无效的或者不支持的函数名称:[{}]", other))),
        }
    }
}
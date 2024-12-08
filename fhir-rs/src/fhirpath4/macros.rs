
#[macro_export]
macro_rules! impl_executor_for_primitive {
    (
        $($ty: ident, )+
    ) => {
        $(
            impl Executor for $ty {
                fn element(&self, symbol: &String, index: &Option<usize>) -> Result<Collection> {
                    match symbol.as_str() {
                        "id" => Ok(self.id.to_collection(index)),
                        "extension" => Ok(self.extension.to_collection(index)),
                        "value" => Ok(self.value.to_collection(index)),
                        other => Err(FhirError::Message(format!("在类型{}中发现无效的路径[{}]", stringify!($ty), other)))
                    }
                }

                fn to_collection(&self, _index: &Option<usize>) -> Collection {
                    Collection::new_any(Box::new(self.clone()))
                }
            }

            impl Convert for $ty {
                fn to_integer(&self) -> Result<Integer> {
                    match &self.value {
                        None => Err(FhirError::error("该数据类型不能转换为数值")),
                        Some(value) => value.to_integer(),
                    }
                }

                fn to_decimal(&self) -> Result<Decimal> {
                    match &self.value {
                        None => Err(FhirError::error("该数据类型不能转换为数值")),
                        Some(value) => value.to_decimal(),
                    }
                }

                fn to_strings(&self) -> Result<String> {
                    match &self.value {
                        None => Err(FhirError::error("该数据类型不能转换为数值")),
                        Some(value) => value.to_strings(),
                    }
                }

                fn to_datetime(&self) -> Result<DateTime> {
                    match &self.value {
                        None => Err(FhirError::error("该数据类型不能转换为数值")),
                        Some(value) => value.to_datetime(),
                    }
                }

                fn to_boolean(&self) -> Result<Boolean> {
                    match &self.value {
                        None => Err(FhirError::error("该数据类型不能转换为数值")),
                        Some(value) => value.to_boolean(),
                    }
                }
            }

            impl Compare for $ty {
                fn eq(&self, right: &dyn Executor) -> Result<bool> {
                    match &self.value {
                        Some(value) => Compare::eq(value, right),
                        None => Err(FhirError::error("该类型的value取值为空")),
                    }
                }
            }
        )+
    };
}

#[macro_export]
macro_rules! impl_executor_for_anytype {
    (
        $($id: ident,)+
    ) => {
        impl Executor for AnyType {
            fn element(&self, symbol: &String, index: &Option<usize>) -> Result<Collection> {
                match self {
                    $(AnyType::$id(value) => value.element(symbol, index),)+
                }
            }

            fn to_collection(&self, index: &Option<usize>) -> Collection {
                match self {
                    $(AnyType::$id(value) => value.to_collection(index),)+
                }
            }
        }

        impl Convert for AnyType {}

        impl Compare for AnyType {}
    };
}

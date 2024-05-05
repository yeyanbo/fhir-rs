use crate::helper;

pub(crate) fn expand_derive_primitive(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let element_impl = impl_element(struct_name_ident)?;
    let primitive_impl = impl_primitive(struct_name_ident)?;
    let serialize_impl = impl_serialize(struct_name_ident)?;
    let deserialize_impl = impl_deserialize(struct_name_ident)?;
    let fhirpath_impl = impl_fhirpath(struct_name_ident)?;

    let ret = quote::quote!(
        #primitive_impl
        #element_impl
        #serialize_impl
        #deserialize_impl
        #fhirpath_impl
    );
    Ok(ret)
}

pub fn impl_element(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();
    
    let ret = quote::quote!(
        impl Element for #struct_name_ident {
            fn has_id(&self) -> bool {
                self.id.is_some()
            }
            fn id(&self) -> &Option<String> {
                &self.id
            }

            fn set_id<T: Into<String>>(mut self, id: T) -> Self {
                self.id = Some(id.into());
                self
            }

            fn has_extension(&self) -> bool {
                self.extension.is_some()
            }

            fn extension(&self) -> &Option<Vec<Extension>> {
                &self.extension
            }

            fn set_extension(mut self, ext: Vec<Extension>) -> Self {
                self.extension = Some(ext);
                self
            }

            fn add_extension(mut self, ext: Extension) -> Self {
                match self.extension {
                    Some(ref mut exts) => {
                        exts.push(ext);
                    },
                    None => {
                        self.extension = Some(vec![ext])
                    },
                }
                self
            }
        }

        impl Base for #struct_name_ident {
            fn is_empty(&self) -> bool {
                self.value.is_none() & self.extension.is_none()
            }

            fn type_name(&self) -> String {
                #struct_name_literal.to_string()
            }
        }
    );
    Ok(ret)
}

fn impl_primitive(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let interal = helper::primitive_internal(&struct_name_ident)?;

    let ret = quote::quote!(
        impl Primitive for #struct_name_ident {
            type T = #interal;

            fn new<A: Into<Self::T>>(v: A) -> Self {
                #struct_name_ident {
                    id: None,
                    extension: None,
                    value: Some(v.into()),
                }
            }

            fn has_value(&self) -> bool {
                self.value.is_some()
            }

            fn value(&self) -> &Option<Self::T> {
                &self.value
            }

            fn set_value(mut self, v: Self::T) -> Self {
                self.value = Some(v);
                self
            }

            fn combine(&mut self, other: Self) {
                if other.has_id() {
                    self.id = other.id.clone()
                }
                if other.has_extension() {
                    self.extension = other.extension.clone()
                }
                if other.has_value() {
                    self.value = other.value.clone()
                }
            }
        }

        impl Display for #struct_name_ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match &self.value{
                    Some(v) => v.fmt(f),
                    None => write!(f, ""),
                }
            }
        }

        impl FromStr for #struct_name_ident {
            type Err = FhirError;
        
            fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
                let val = #interal::from_str(s)?;
                Ok(Self {
                    id: None,
                    extension: None,
                    value: Some(val),
                })
            }
        }
    );
    Ok(ret)
}

fn impl_serialize(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {

    let ret = quote::quote!(
        impl Serialize for #struct_name_ident {
            fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
                let mut primitive  = serializer.serialize_primitive()?;
                primitive.serialize_id(&self.id)?;
                primitive.serialize_value(&self.value)?;
                primitive.serialize_extension(&self.extension)?;
                primitive.serialize_end()
            }
        }
    );
    Ok(ret)
}

fn impl_deserialize(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let interal = helper::primitive_internal(&struct_name_ident)?;
    let visitor = helper::visitor(&struct_name_ident)?;

    let ret = quote::quote!(
        impl<'de> Deserialize<'de> for #struct_name_ident {
            fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {

                pub struct #visitor;
                impl<'de> Visitor<'de> for #visitor {
                    type Value = #struct_name_ident;

                    fn visit_str(self, v: &str) -> Result<Self::Value> {
                        #struct_name_ident::from_str(v)
                    }

                    fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value> where M: MapAccess<'de> {
                        let mut id: Option<String> = None;
                        let mut extension: Option<Vec<Extension>> = None;
                        let mut value: Option<#interal> = None;

                        while let Some(key) = mapp.next_key()? {
                            match key.as_str() {
                                "id" => {
                                    id = Some(mapp.next_value()?);
                                },
                                "extension" => {
                                    extension = Some(mapp.next_value()?);
                                }
                                "value" => {
                                    value = Some(mapp.next_value()?);
                                }
                                _ => {return Err(FhirError::error("读到不存在的key了"));},
                            }
                        }

                        Ok(#struct_name_ident{
                            id,
                            extension,
                            value,
                        })
                    }
                }

                deserializer.deserialize_primitive("", #visitor)
            }
        }
    );
    Ok(ret)
}

fn impl_fhirpath(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();
    // let interal = helper::primitive_internal(&struct_name_ident)?;

    let ret = quote::quote!(
        impl Executor for #struct_name_ident {
            // fn exec(&self, comp: &PathComponent) -> Result<PathResponse> {
            //     println!("{}: Start Exec Fhirpath...", #struct_name_literal);
            //
            //     match comp {
            //         PathComponent::Path(path) => {
            //             match path.symbol.as_str() {
            //                 "id" => {
            //                     self.id.exec(&comp)
            //                 },
            //                 "extension" => {
            //                     self.extension.exec(&comp)
            //                 }
            //                 "value" => {
            //                     self.value.exec(&comp)
            //                 }
            //                 other => Err(FhirError::Message(format!("{}: 无效的路径名:[{}]", #struct_name_literal, other)))
            //             }
            //         },
            //         _ => Err(FhirError::Message(format!("{}: 无效的路径表达式:{:#?}", #struct_name_literal, &comp))),
            //     }
            // }
            fn element(&self, symbol: &String, index: &Option<usize>) -> Result<Collection> {
                 println!("{}: Element[{}]...", #struct_name_literal, &symbol);

                match symbol.as_str() {
                    "id" => Ok(self.id.to_collection(index)),
                    "extension" => Ok(self.extension.to_collection(index)),
                    "value" => Ok(self.value.to_collection(index)),
                    other => Err(FhirError::Message(format!("{}: 无效的路径名:[{}]", #struct_name_literal, other)))
                }
            }

            fn to_collection(&self, index: &Option<usize>) -> Collection {
                Collection::new_any(Box::new(self.clone()))
            }
        }

        impl Convert for #struct_name_ident {
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

        impl Compare for #struct_name_ident {
            fn eq(&self, right: &dyn Executor) -> Result<bool> {
                match &self.value {
                    Some(value) => Compare::eq(value, right),
                    None => Err(FhirError::error("该类型的value取值为空")),
                }
            }
        }
    );
    Ok(ret)
}
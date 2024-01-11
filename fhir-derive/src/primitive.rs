use crate::helper;

pub(crate) fn expand_derive_primitive(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let element_impl = impl_element(struct_name_ident)?;
    let primitive_impl = impl_primitive(struct_name_ident)?;
    let serialize_impl = impl_serialize(struct_name_ident)?;
    let deserialize_impl = impl_deserialize(struct_name_ident)?;

    let ret = quote::quote!(
        #element_impl
        #primitive_impl
        #serialize_impl
        #deserialize_impl
    );
    Ok(ret)
}

/// 为简单类型实现了Element特性
fn impl_element(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {

    let ret = quote::quote!(
        impl Element for #struct_name_ident {
            fn has_id(&self) -> bool {
                self.id.is_some()
            }
            fn id(&self) -> &Option<String> {
                &self.id
            }
            fn set_id(mut self, id: String) -> Self {
                self.id = Some(id);
                self
            }
            fn has_extension(&self) -> bool {
                self.extension.is_some()
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
            fn extension(&self) -> &Option<Vec<Extension>> {
                &self.extension
            }
            fn set_extension(mut self, ext: Vec<Extension>) -> Self {
                self.extension = Some(ext);
                self
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

            fn new(v: Self::T) -> Self {
                #struct_name_ident {
                    id: None,
                    extension: None,
                    value: Some(v),
                }
            }


            fn value(&self) -> &Option<Self::T> {
                &self.value
            }

            fn set_value(mut self, v: Self::T) -> Self {
                self.value = Some(v);
                self
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

                    fn visit_map<M>(self, mut map: M) -> Result<Self::Value> where M: MapAccess<'de> {
                        let mut id: Option<String> = None;
                        let mut extension: Option<Vec<Extension>> = None;
                        let mut value: Option<#interal> = None;

                        while let Some(key) = map.next_key()? {
                            match key.as_str() {
                                "id" => {
                                    id = Some(map.next_value()?);
                                },
                                "extension" => {
                                    extension = Some(map.next_value()?);
                                }
                                "value" => {
                                    value = Some(map.next_value()?);
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

                deserializer.deserialize_struct("", #visitor)
            }
        }
    );
    Ok(ret)
}
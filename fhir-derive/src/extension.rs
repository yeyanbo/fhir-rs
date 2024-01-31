use crate::helper;

pub(crate) fn expand_derive_extension(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let element_impl = helper::impl_element(struct_name_ident)?;
    let serialize_impl = impl_serialize(struct_name_ident)?;
    let deserialize_impl = impl_deserialize(struct_name_ident)?;

    let ret = quote::quote!(
        #element_impl
        #serialize_impl
        #deserialize_impl
    );
    Ok(ret)
}

fn impl_serialize(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let ret = quote::quote!(
        impl Serialize for #struct_name_ident {
            fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
                let mut extension  = serializer.serialize_extension()?;
                extension.serialize_id(&self.id)?;
                extension.serialize_url(&self.url)?;
                extension.serialize_extension(&self.extension)?;
                extension.serialize_value(&self.value)?;
                extension.serialize_end()
            }
        }
    );
    Ok(ret)
}

fn impl_deserialize(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let ret = quote::quote!(
        impl<'de> Deserialize<'de> for #struct_name_ident {
            fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
                pub struct ExtensionVisitor;
                impl<'de> Visitor<'de> for ExtensionVisitor {
                    type Value = Extension;
        
                    fn visit_map<V>(self, mut map: V) -> Result<Extension>
                        where
                            V: MapAccess<'de>,
                    {
                        let mut id: Option<String> = None;
                        let mut url : Option<String> = None;
                        let mut extension: Option<Vec<Extension>> = None;
                        let mut value: Option<Any> = None;
        
                        while let Some(key) = map.next_key()? {
                            match key.as_str() {
                                "id" => {
                                    id = Some(map.next_value()?);
                                    tracing::debug!("读取到值: {:?}", &id);
                                },
                                "url" => {
                                    url = Some(map.next_value()?);
                                    tracing::debug!("读取到值: {:?}", &url);
                                }
                                "extension" => {
                                    extension = Some(map.next_value()?);
                                    tracing::debug!("读取到值: {:?}", &url);
                                }
                                "valueString" => {
                                    let temp: StringDt = map.next_value()?;
                                    value = Some(Any::String(temp));
                                    tracing::debug!("读取到值: {:?}", &value);
                                }
                                "valuePositiveInt" => {
                                    let temp: PositiveIntDt = map.next_value()?;
                                    value = Some(Any::PositiveInt(temp));
                                    tracing::debug!("读取到值: {:?}", &value);
                                }
                                _ => {return Err(FhirError::error("Extension读到不存在的key了"));},
                            }
                        }
        
                        Ok(Extension {
                            id,
                            url,
                            extension,
                            value,
                        })
                    }
                }
        
                deserializer.deserialize_struct("Extension", ExtensionVisitor)
            }
        }
    );
    Ok(ret)
}

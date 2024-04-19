use crate::helper;
use crate::helper::{Field};

pub(crate) fn expand_derive_backbone(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let fields = helper::get_struct_fields(&st)?;

    let element_impl = impl_element(struct_name_ident, &fields)?;
    let serialize_impl = impl_serialize(struct_name_ident, &fields)?;
    let deserialize_impl = impl_deserialize(struct_name_ident, &fields)?;
    let fhirpath_impl = impl_fhirpath(struct_name_ident, &fields)?;
    
    let ret = quote::quote!(
        #element_impl
        #serialize_impl
        #deserialize_impl
        #fhirpath_impl
    );
    Ok(ret)
}

pub(crate) fn impl_element(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let ret = quote::quote!(

        impl Base for #struct_name_ident {
        }
    );
    Ok(ret)
}

fn impl_serialize(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let fields = helper::impl_serialize_fields(struct_fields)?;

    let ret = quote::quote!(
        impl Serialize for #struct_name_ident {
            fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
                let mut serialize_struct = serializer.serialize_struct("")?;
                serialize_struct.serialize_id(&self.id)?;
                serialize_struct.serialize_extension(&self.extension)?;
                #( #fields )*
                serialize_struct.serialize_end()
            }
        }
    );
    Ok(ret)
}

fn impl_deserialize(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let visitor = helper::visitor(struct_name_ident)?;

    let fields = helper::impl_deserialize_fields(struct_fields)?;
    let defs = helper::impl_deserialize_define(struct_fields)?;
    let maps = helper::impl_deserialize_map(struct_fields)?;

    let ret = quote::quote!(
        impl<'de> Deserialize<'de> for #struct_name_ident {
            fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
                pub struct #visitor;
                impl<'de> Visitor<'de> for #visitor {
                    type Value = #struct_name_ident;

                    fn visit_map<M>(self, mut map: M) -> Result<Self::Value> where M: MapAccess<'de> {
                        #( #defs )*

                        while let Some(keys) = map.next_key()? {
                            match keys.as_str() {
                                #( #maps )*
                                _ => {return Err(FhirError::error_string(format!("读到不存在的键:{}", keys)));},
                            }
                        }

                        Ok(#struct_name_ident {
                            #( #fields )*
                        })
                    }
                }

                deserializer.deserialize_struct("", #visitor)
            }
        }
    );
    Ok(ret)
}

pub fn impl_fhirpath_map(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut maps = Vec::with_capacity(32);

    struct_fields.iter()
        .for_each(|field| {
            let ident = field.name.clone();
            let ident_literal = field.original.clone();
            maps.push(quote::quote!( #ident_literal => { self.#ident.exec(&func, paths) }, ));
        });

    Ok(maps)
}

pub fn impl_fhirpath(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {

    let maps = impl_fhirpath_map(struct_fields)?;

    let ret = quote::quote!(
        impl Executor for #struct_name_ident {
            fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
                match func.definition.function_name() {
                    FunctionName::Element => {
                        match &func.params {
                            FunctionParam::String(name) => {
                                match name.as_str() {
                                    #( #maps )*
                                    other => Err(FhirError::Message(format!("无效的路径名:[{}]", other)))
                                }
                            },
                            _ => unreachable!(),
                        }
                    },
                    _ => Err(FhirError::Message(format!("Patient: 无效的函数名:{:?}", &func))),
                }
            } 

            fn as_collection(&self) -> Collection {
                Collection(vec![Box::new(self.clone())])
            }
        }
    );
    Ok(ret)
}


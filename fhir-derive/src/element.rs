use crate::helper;
use crate::helper::Field;

pub(crate) fn expand_derive_element(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
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

pub(crate) fn impl_element(struct_name_ident: &syn::Ident, _struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
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
            fn type_name(&self) -> String {
                #struct_name_literal.to_string()
            }
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

                    fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value> where M: MapAccess<'de> {
                        #( #defs )*

                        while let Some(keys) = mapp.next_key()? {
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
            maps.push(quote::quote!( #ident_literal => Ok(self.#ident.to_collection(index)), ));
        });

    Ok(maps)
}

pub fn impl_fhirpath(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();
    let maps = impl_fhirpath_map(struct_fields)?;

    let ret = quote::quote!(
        impl Executor for #struct_name_ident {
            // fn exec(&self, comp: &PathComponent) -> Result<PathResponse> {
            //
            //     match comp {
            //         PathComponent::Path(path) => {
            //             match path.symbol.as_str() {
            //                 #( #maps )*
            //                 other => Err(FhirError::Message(format!("{}: 无效的路径名:[{}]", #struct_name_literal, other)))
            //             }
            //         },
            //         _ => Err(FhirError::Message(format!("#struct_name_ident: 无效的函数名:{:?}", &comp))),
            //     }
            // }

            fn element(&self, symbol: &String, index: &Option<usize>) -> Result<Collection> {
                 println!("{}: Element[{}]...", #struct_name_literal, &symbol);

                match symbol.as_str() {
                    #( #maps )*
                    other => Err(FhirError::Message(format!("{}: 无效的路径名:[{}]", #struct_name_literal, other)))
                }
            }

            fn to_collection(&self, index: &Option<usize>) -> Collection {
                Collection::new_any(Box::new(self.clone()))
            }
        }

        impl Convert for #struct_name_ident {}
        impl Compare for #struct_name_ident {}
    );
    Ok(ret)
}


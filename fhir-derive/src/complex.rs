use quote::format_ident;
use crate::helper::{self, Field};

pub(crate) fn expand_derive_complex(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let fields = helper::get_struct_fields(&st)?;
    let element_impl = helper::impl_element(struct_name_ident)?;
    let complex_impl = impl_complex(struct_name_ident, &fields)?;
    let serialize_impl = impl_serialize(struct_name_ident, &fields)?;
    let deserialize_impl = impl_deserialize(struct_name_ident, &fields)?;

    let ret = quote::quote!(
        #element_impl
        #complex_impl
        #serialize_impl
        #deserialize_impl
    );
    Ok(ret)
}

fn impl_complex(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let fns = impl_complex_fields(struct_fields)?;

    let ret = quote::quote!(
        impl #struct_name_ident {
            #( #fns )*
        }
    );
    Ok(ret)
}

pub(crate) fn impl_complex_fields(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .skip(2)
        .for_each(|f| {
            let ident = &f.name;
            let typ = &f.ty;

            // 赋值类型为Option的内部类型
            let set_func = format_ident!("set_{}",  ident);
            let value_type = helper::option_inner(typ).unwrap();

            if helper::is_primitive(value_type) {
                fields.push(quote::quote!(
                    pub fn #set_func<T: Into<#value_type>>(mut self, v: T) -> Self {
                        self.#ident = Some(v.into());
                        self
                    }
                ));
            } else {
                fields.push(quote::quote!(
                    pub fn #set_func(mut self, v: #value_type) -> Self {
                        self.#ident = Some(v);
                        self
                    }
                ));
            };

            // 如果类型是Vec，添加形如add_xxxx的函数，参数为Vec的内部类型
            if let Some(v_typ) = helper::vector_inner(value_type) {

                let add_func = format_ident!("add_{}",  ident);
                if helper::is_primitive(v_typ) {
                    fields.push(quote::quote!(
                        pub fn #add_func<T: Into<#v_typ>>(mut self, v: T) -> Self {
                            match self.#ident {
                                None => {
                                    let mut vec = Vec::new();
                                    vec.push(v.into());
                                    self.#ident = Some(vec);
                                }
                                Some(ref mut vec) => {
                                    vec.push(v.into());
                                }
                            }
                            self
                        }
                    ));
                } else {
                    fields.push(quote::quote!(
                        pub fn #add_func(mut self, v: #v_typ) -> Self {
                            match self.#ident {
                                None => {
                                    let mut vec = Vec::new();
                                    vec.push(v);
                                    self.#ident = Some(vec);
                                }
                                Some(ref mut vec) => {
                                    vec.push(v);
                                }
                            }
                            self
                        }
                    ));
                }
            }
        });

    Ok(fields)
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

                        while let Some(key) = map.next_key()? {
                            match key.as_str() {
                                #( #maps )*
                                _ => {return Err(FhirError::error("读到不存在的键"));},
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

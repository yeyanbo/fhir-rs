use crate::helper;
use crate::helper::{Field};

pub(crate) fn expand_derive_backbone(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let fields = helper::get_struct_fields(&st)?;

    let serialize_impl = impl_serialize(struct_name_ident, &fields)?;
    let deserialize_impl = impl_deserialize(struct_name_ident, &fields)?;

    let ret = quote::quote!(
        #serialize_impl
        #deserialize_impl
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




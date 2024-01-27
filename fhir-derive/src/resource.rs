use crate::helper;
use crate::helper::StructFields;

pub(crate) fn expand_derive_resource(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let fields = helper::get_struct_fields(&st)?;
    let serialize_impl = impl_serialize(struct_name_ident, fields)?;
    let deserialize_impl = impl_deserialize(struct_name_ident, fields)?;

    let ret = quote::quote!(
        #serialize_impl
        #deserialize_impl
    );
    Ok(ret)
}

fn impl_serialize(struct_name_ident: &syn::Ident, struct_fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();
    let fields = impl_serialize_fields(struct_fields)?;

    let ret = quote::quote!(
        impl Serialize for #struct_name_ident {
            fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
                let mut serialize_struct = serializer.serialize_resource( #struct_name_literal )?;
                serialize_struct.serialize_id(&self.id)?;
                #( #fields )*
                serialize_struct.serialize_end()
            }
        }
    );
    Ok(ret)
}

/// 在序列化函数中，用于生成所有字段（除id之外）
/// 在资源中id是需要特殊处理的，skip(1)就是跳过id
fn impl_serialize_fields(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .skip(1)
        .map(|f| { &f.ident })
        .for_each(|ident| {
            let ident_literal = ident.as_ref().unwrap().to_string();
            fields.push(quote::quote!(serialize_struct.serialize_field(#ident_literal, &self.#ident)?;));
        });

    Ok(fields)
}

fn impl_deserialize(struct_name_ident: &syn::Ident, struct_fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
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




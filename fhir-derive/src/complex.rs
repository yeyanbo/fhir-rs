// use syn::spanned::Spanned;
use crate::helper;

pub(crate) fn expand_derive_complex(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let fields = get_struct_fields(&st)?;

    let serialize_impl = impl_serialize(struct_name_ident, fields)?;
    let deserialize_impl = impl_deserialize(struct_name_ident, fields)?;

    let ret = quote::quote!(
        #serialize_impl
        #deserialize_impl
    );
    Ok(ret)
}

fn impl_serialize(struct_name_ident: &syn::Ident, struct_fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let fields = impl_serialize_fields(struct_fields, struct_name_ident.span())?;

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

fn impl_serialize_fields(struct_fields: &StructFields, span: proc_macro2::Span) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    let id = syn::Ident::new("id", span);
    let ext = syn::Ident::new("extension", span);

    struct_fields.iter()
        .map(|f| { &f.ident })
        .filter(|f| {
            let ident = f.as_ref().unwrap();
            ident.ne(&id) && ident.ne(&ext)
        })
        .for_each(|ident| {
            let ident_literal = ident.as_ref().unwrap().to_string();
            fields.push(quote::quote!(serialize_struct.serialize_field(#ident_literal, &self.#ident)?;));
        });

    Ok(fields)
}

fn impl_deserialize(struct_name_ident: &syn::Ident, struct_fields: &StructFields) -> syn::Result<proc_macro2::TokenStream> {
    let visitor = helper::visitor(struct_name_ident)?;

    let fields = impl_deserialize_fields(struct_fields)?;
    let defs = impl_deserialize_define(struct_fields)?;
    let maps = impl_deserialize_map(struct_fields)?;

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

fn impl_deserialize_define(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut defines = Vec::with_capacity(32);

    struct_fields.iter()
        // .map(|f| { &f.ident })
        .for_each(|f| {
            let ident = &f.ident;
            let typ = &f.ty;
            defines.push(quote::quote!( let mut #ident: #typ = None; ));
        });

    Ok(defines)
}

fn impl_deserialize_map(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut maps = Vec::with_capacity(32);

    struct_fields.iter()
        .map(|f| { &f.ident })
        .for_each(|ident| {
            let ident_literal = ident.as_ref().unwrap().to_string();
            maps.push(quote::quote!( #ident_literal => { #ident = Some(map.next_value()?);}, ));
        });

    Ok(maps)
}

fn impl_deserialize_fields(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .map(|f| { &f.ident })
        .for_each(|ident| {
            fields.push(quote::quote!(#ident,));
        });

    Ok(fields)
}

type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

fn get_struct_fields(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct{
                                 fields: syn::Fields::Named(syn::FieldsNamed{named, ..}),
                                 ..
                             }) = &st.data {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(&st, "Must define On Struct".to_string()))
}
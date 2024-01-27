// use syn::spanned::Spanned;

pub(crate) type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

pub(crate) fn get_struct_fields(st: &syn::DeriveInput) -> syn::Result<&StructFields> {
    if let syn::Data::Struct(syn::DataStruct{
                                 fields: syn::Fields::Named(syn::FieldsNamed{named, ..}),
                                 ..
                             }) = &st.data {
        return Ok(named);
    }
    Err(syn::Error::new_spanned(&st, "Must define On Struct".to_string()))
}

pub(crate) fn visitor(struct_name_ident: &syn::Ident) -> syn::Result<syn::Ident> {
    let visitor_literal = format!("{}Visitor", &struct_name_ident.to_string());
    Ok(syn::Ident::new(&visitor_literal, struct_name_ident.span()))
}

pub(crate) fn primitive_internal(struct_name_ident: &syn::Ident) -> syn::Result<syn::Ident> {
    let internal_literal = &struct_name_ident.to_string().replace("Dt", "");
    Ok(syn::Ident::new(&internal_literal, struct_name_ident.span()))
}

#[warn(dead_code)]
pub(crate) fn option_inner(field_type: &syn::Type) -> Option<&syn::Type> {

    if let syn::Type::Path(syn::TypePath { path: syn::Path { ref segments, .. }, ..}) = field_type
    {
        if let Some(seg) = segments.last() {
            if seg.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(
                    syn::AngleBracketedGenericArguments{ref args, ..}
                ) = seg.arguments {
                    if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                        return Some(inner_type)
                    }
                }
            }
        }
    }
    None
}

/// 在序列化函数中，用于生成所有字段（除id、extension之外）
/// TODO 待优化：如果能够将id、extension的处理方法与其它通用字段保持一致，可以大大降低处理逻辑的复杂性
pub(crate) fn impl_serialize_fields(struct_fields: &StructFields, span: proc_macro2::Span) -> syn::Result<Vec<proc_macro2::TokenStream>> {
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


pub(crate) fn impl_deserialize_define(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
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

///
pub(crate) fn impl_deserialize_map(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut maps = Vec::with_capacity(32);

    struct_fields.iter()
        .map(|f| { &f.ident })
        .for_each(|ident| {
            let ident_literal = ident.as_ref().unwrap().to_string();
            maps.push(quote::quote!( #ident_literal => { #ident = Some(map.next_value()?);}, ));
        });

    Ok(maps)
}

/// 在反序列化函数中，生成最后的数据结构体内的所有字段
pub(crate) fn impl_deserialize_fields(struct_fields: &StructFields) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .map(|f| { &f.ident })
        .for_each(|ident| {
            fields.push(quote::quote!(#ident,));
        });

    Ok(fields)
}
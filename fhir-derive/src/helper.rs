// use syn::spanned::Spanned;

pub(crate) fn visitor(struct_name_ident: &syn::Ident) -> syn::Result<syn::Ident> {
    let visitor_literal = format!("{}Visitor", &struct_name_ident.to_string());
    Ok(syn::Ident::new(&visitor_literal, struct_name_ident.span()))
}

pub(crate) fn primitive_internal(struct_name_ident: &syn::Ident) -> syn::Result<syn::Ident> {
    let internal_literal = &struct_name_ident.to_string().replace("Dt", "");
    Ok(syn::Ident::new(&internal_literal, struct_name_ident.span()))
}

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
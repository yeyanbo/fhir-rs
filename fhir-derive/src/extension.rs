use crate::helper;

pub(crate) fn expand_derive_extension(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let element_impl = helper::impl_element(struct_name_ident)?;

    let ret = quote::quote!(
        #element_impl
    );
    Ok(ret)
}

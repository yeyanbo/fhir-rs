use crate::helper;

pub(crate) fn expand_derive_primitive(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let primitive_impl = impl_primitive(struct_name_ident)?;

    let ret = quote::quote!(
        #primitive_impl
    );
    Ok(ret)
}

fn impl_primitive(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let interal = helper::primitive_internal(&struct_name_ident)?;

    let ret = quote::quote!(
        impl Primitive for #struct_name_ident {
            type T = #interal;

            fn new<A: Into<Self::T>>(v: A) -> Self {
                #struct_name_ident {
                    id: None,
                    extension: None,
                    value: Some(v.into()),
                }
            }

            fn has_value(&self) -> bool {
                self.value.is_some()
            }

            fn value(&self) -> &Option<Self::T> {
                &self.value
            }

            fn set_value(mut self, v: Self::T) -> Self {
                self.value = Some(v);
                self
            }

            fn combine(&mut self, other: Self) {
                if other.id.is_some() {
                    self.id = other.id.clone()
                }
                if other.extension.is_some() {
                    self.extension = other.extension.clone()
                }
                if other.has_value() {
                    self.value = other.value.clone()
                }
            }
        }

        impl Display for #struct_name_ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match &self.value{
                    Some(v) => v.fmt(f),
                    None => write!(f, ""),
                }
            }
        }
    );
    Ok(ret)
}
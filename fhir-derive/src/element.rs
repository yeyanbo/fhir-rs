
pub(crate) fn expand_derive_element(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let base_impl = impl_base(struct_name_ident)?;
    let datatype_impl = impl_datatype(struct_name_ident)?;

    let ret = quote::quote!(
        #base_impl
        #datatype_impl
    );
    Ok(ret)
}

/// 为所有类型实现通用的DataType接口
pub(crate) fn impl_datatype(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {

    let ret = quote::quote!(

        impl DataType for #struct_name_ident {}

        impl Element for #struct_name_ident {

            fn id(&self) -> Option<&String> {
                self.id.as_ref()
            }

            fn set_id<T: Into<String>>(mut self, id: T) -> Self {
                self.id = Some(id.into());
                self
            }

            fn extensions(&self) -> Option<&Vec<Extension>> {
                self.extension.as_ref()
            }

            fn set_extensions(mut self, ext: Vec<Extension>) -> Self {
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
    );
    Ok(ret)
}

/// 为所有类型实现通用的Base接口
pub(crate) fn impl_base(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();

    let ret = quote::quote!(
        impl Base for #struct_name_ident {
            fn type_name(&self) -> &str {
                #struct_name_literal
            }
        }
    );
    Ok(ret)
}
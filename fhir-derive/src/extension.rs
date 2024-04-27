
pub(crate) fn expand_derive_extension(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let element_impl = impl_element(struct_name_ident)?;

    let ret = quote::quote!(
        #element_impl
    );
    Ok(ret)
}

pub fn impl_element(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
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
            fn is_empty(&self) -> bool {
                self.url.is_none() & self.value.is_none() 
            }

            fn type_name(&self) -> String {
                #struct_name_literal.to_string()
            }
        }
    );
    Ok(ret)
}


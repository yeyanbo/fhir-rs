mod primitive;
mod complex;
mod helper;

use proc_macro::TokenStream;
use syn;
// use quote;
// use syn::spanned::Spanned;

#[proc_macro_derive(Complex, attributes(fhir))]
pub fn derive_complex(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    complex::expand_derive_complex(&st)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Primitive, attributes(fhir))]
pub fn derive_primitive(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    primitive::expand_derive_primitive(&st)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Resource, attributes(fhir))]
pub fn derive_resource(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    // eprintln!("{:#?}", st);
    TokenStream::new()
}

#[proc_macro_derive(BackboneElement, attributes(fhir))]
pub fn derive_backbone_element(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    TokenStream::new()
}

#[proc_macro_derive(Element, attributes(fhir))]
pub fn derive_element(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    TokenStream::new()
}

#[proc_macro_derive(Extension, attributes(fhir))]
pub fn derive_extension(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    TokenStream::new()
}
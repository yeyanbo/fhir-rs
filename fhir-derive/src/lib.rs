mod primitive;
mod complex;
mod helper;
mod resource;
mod backbone;
mod element;

use proc_macro::TokenStream;

#[proc_macro_derive(Element, attributes(fhir))]
pub fn derive_element(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    element::expand_derive_element(&st)
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

#[proc_macro_derive(Complex, attributes(fhir))]
pub fn derive_complex(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    complex::expand_derive_complex(&st)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(Resource, attributes(fhir))]
pub fn derive_resource(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    resource::expand_derive_resource(&st)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

#[proc_macro_derive(BackboneElement, attributes(fhir))]
pub fn derive_backbone_element(item: TokenStream) -> TokenStream {
    let st = syn::parse_macro_input!(item as syn::DeriveInput);

    backbone::expand_derive_backbone(&st)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
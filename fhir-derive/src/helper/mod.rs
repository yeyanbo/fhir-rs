use syn::{Ident, LitBool, LitStr};

// pub(crate) type StructFields = syn::punctuated::Punctuated<syn::Field, syn::Token!(,)>;

pub(crate) struct Field {
    pub name: syn::Ident,
    pub ty: syn::Type,
    pub original: String,
    pub min: String,
    pub max: String,
    pub summary: bool,
    pub modifier: bool,
    pub choice: String,
}

impl Field {
    pub fn new(name: syn::Ident, ty: syn::Type) -> Self {
        Field {
            name,
            ty,
            original: "".to_string(),
            min: "".to_string(),
            max: "".to_string(),
            summary: false,
            modifier: false,
            choice: "".to_string(),
        }
    }
}

impl From<&syn::Field> for Field {
    fn from(value: &syn::Field) -> Self {
        let mut field = Field::new(value.ident.clone().unwrap(), value.ty.clone());
        for attr in &value.attrs {
            if !attr.path().is_ident("fhir") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                // eprintln!("{:#?}", meta.path);
                if meta.path.is_ident("name") {
                    let value = meta.value()?;
                    let str: LitStr = value.parse()?;
                    field.original = str.value();
                } else if meta.path.is_ident("min") {
                    let value = meta.value()?;
                    let str: LitStr = value.parse()?;
                    field.min = str.value();
                } else if meta.path.is_ident("max") {
                    let value = meta.value()?;
                    let str: LitStr = value.parse()?;
                    field.max = str.value();
                }
                else if meta.path.is_ident("summary") {
                    let value = meta.value()?;
                    let str: LitBool = value.parse()?;
                    field.summary = str.value();
                } else if meta.path.is_ident("modifier") {
                    let value = meta.value()?;
                    let str: LitBool = value.parse()?;
                    field.modifier = str.value();
                } else if meta.path.is_ident("choice") {
                    let value = meta.value()?;
                    let str: LitStr = value.parse()?;
                    field.choice = str.value();
                }
                Ok(())
            }).expect("Field From Trait Impl Failure");
        }
        field
    }
}

pub(crate) fn get_struct_fields(st: &syn::DeriveInput) -> syn::Result<Vec<Field>> {
    if let syn::Data::Struct(syn::DataStruct{
                                 fields: syn::Fields::Named(syn::FieldsNamed{named, ..}),
                                 ..
                             }) = &st.data {
        let mut vec = Vec::new();

        named.iter()
            .for_each(|f| {
                vec.push(f.into());
            });
        return Ok(vec);
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

pub(crate) fn vector_inner(field_type: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Path(syn::TypePath { path: syn::Path { ref segments, .. }, ..}) = field_type
    {
        if let Some(seg) = segments.last() {
            if seg.ident == "Vec" {
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

pub(crate) fn is_primitive(field_type: &syn::Type) -> bool {
    if let syn::Type::Path(syn::TypePath { path: syn::Path { ref segments, .. }, ..}) = field_type
    {
        if let Some(seg) = segments.last() {
            if seg.ident.to_string().ends_with("Dt") {
                return true;
            }
        }
    }
    false
}

pub(crate) fn base_resource(st: &syn::DeriveInput) -> Option<String> {
    let mut rst : Option<String> = None;
    let attrs = &st.attrs;
    if let Some(attr) = attrs.first() {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("base") {
                let value = meta.value()?;
                let str: LitStr = value.parse()?;
                rst = Some(str.value());
            }
            Ok(())
        }).expect("TODO: panic message");
    }
    rst
}

// pub(crate) fn field_attr(field: &syn::Field, attr_key: &str) -> Option<String> {
//     let mut rst : Option<String> = None;
//     let attrs = &field.attrs;
//     if let Some(attr) = attrs.first() {
//         attr.parse_nested_meta(|meta| {
//             if meta.path.is_ident("base") {
//                 let value = meta.value()?;
//                 let str: LitStr = value.parse()?;
//                 rst = Some(str.value());
//             }
//             Ok(())
//         }).expect("TODO: panic message");
//     }
//     rst
// }

/// 在序列化函数中，用于生成所有字段（除id、extension之外）
/// TODO 待优化：如果能够将id、extension的处理方法与其它通用字段保持一致，可以大大降低处理逻辑的复杂性
pub(crate) fn impl_serialize_fields(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .skip(2)
        .for_each(|f| {
            let name = &f.name;
            let name_literal = &f.original;
            fields.push(quote::quote!(
                serialize_struct.serialize_field(#name_literal, &self.#name)?;
            ));
        });

    Ok(fields)
}

pub(crate) fn impl_deserialize_define(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut defines = Vec::with_capacity(32);

    struct_fields.iter()
        .for_each(|f| {
            let ident = &f.name;
            let typ = &f.ty;
            defines.push(quote::quote!( let mut #ident: #typ = None; ));
        });

    Ok(defines)
}

///
pub(crate) fn impl_deserialize_map(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut maps = Vec::with_capacity(32);

    struct_fields.iter()
        .for_each(|field| {
            let ident = field.name.clone();
            let ident_literal = field.original.clone();

            if field.choice.len() > 0 {
                let mut any_type_maps = Vec::with_capacity(32);

                field.choice.split("|")
                    .for_each(|ty| {
                        let dd = syn::Ident::new(ty, ident.span().clone());
                        any_type_maps.push(quote::quote!(#ty => Some(AnyType::#dd(mapp.next_value()?)),));
                    });

                maps.push(quote::quote!( 
                    k_value if k_value.starts_with(#ident_literal) => { 
                        let ttt = k_value.replace(#ident_literal, "");
                        #ident = match ttt.as_str() {
                            #( #any_type_maps )*
                            _ => return Err(FhirError::Message(format!("{}不在可选类型范围之内", &k_value))),
                        }
                    }, )
                );
            } else {
                maps.push(quote::quote!( #ident_literal => { #ident = Some(mapp.next_value()?);}, ));
            }
        });

    Ok(maps)
}

/// 在反序列化函数中，生成最后的数据结构体内的所有字段
pub(crate) fn impl_deserialize_fields(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .map(|f| { &f.name })
        .for_each(|ident| {
            fields.push(quote::quote!(#ident,));
        });

    Ok(fields)
}
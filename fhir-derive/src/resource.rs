use proc_macro2::TokenStream;
use quote::format_ident;
use crate::helper::{self, Field};

pub(crate) fn expand_derive_resource(st: &syn::DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_ident = &st.ident;

    let fields = helper::get_struct_fields(&st)?;
    let resource_trait_impl = impl_resource_trait(struct_name_ident)?;

    let mut domain_resource_trait_impl = TokenStream::new();
    if let Some(base) = helper::base_resource(st) {
        if base == "DomainResource" {
            domain_resource_trait_impl = impl_domain_resource_trait(struct_name_ident)?;
        }
    }

    let resource_impl = impl_resource(struct_name_ident, &fields)?;
    let serialize_impl = impl_serialize(struct_name_ident, &fields)?;
    let deserialize_impl = impl_deserialize(struct_name_ident, &fields)?;
    let fhirpath_impl = impl_fhirpath(struct_name_ident, &fields)?;
    
    let ret = quote::quote!(
        #resource_trait_impl
        #domain_resource_trait_impl
        #resource_impl
        #serialize_impl
        #deserialize_impl
        #fhirpath_impl
    );
    Ok(ret)
}

fn impl_resource_trait(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();

    let ret = quote::quote!(

        impl Resource for #struct_name_ident {

            fn resource_name(&self) -> String {
                #struct_name_literal.to_string()
            }

            fn id(&self) -> &Option<String> {
                &self.id
            }

            fn set_id<T: Into<Id>>(mut self, id: T) -> Self {
                self.id = Some(id.into());
                self
            }

            fn meta(&self) -> &Option<Meta> {
                &self.meta
            }

            fn set_meta(mut self, meta: Meta) -> Self {
                self.meta = Some(meta);
                self
            }

            fn assert(&self, path: String) -> Result<bool> {
                let mut paths = FhirPaths::parse(path)?;

                match paths.response() {
                    Some(response) => {
                        if response != &FunctionResponse::Bool {
                            return Err(FhirError::error("该表达式不是一个有效的路径表达式，最后返回值不是Boolean"));
                        }
                    },
                    None => return Err(FhirError::error("该表达式不是一个有效的路径表达式，最后返回值不是Boolean")),
                }

                if let Some(first) = paths.next() {
                    if !first.match_resource_type_name(&self.resource_name()) {
                        return Err(FhirError::Message(format!("路径中首个组成与当前资源类型[{}]不符", self.resource_name())))
                    }
                }

                let mut vv = self.as_collection2();
                while let Some(func) = paths.next() {
                    vv = vv.exec(&func, &mut paths)?;
                    println!("Response: {:?}", &vv)
                }

                match vv {
                    PathResponse::Bool(value) => Ok(value),
                    _ => unreachable!()
                }
            }

            fn path(&self, path: String) -> Result<Collection> {
                let mut paths = FhirPaths::parse(path)?;

                match paths.response() {
                    Some(response) => {
                        if response != &FunctionResponse::Collection {
                            return Err(FhirError::error("该表达式不是一个有效的路径表达式，最后返回值不是Collection"));
                        }
                    },
                    None => return Err(FhirError::error("该表达式不是一个有效的路径表达式，最后返回值不是Collection")),
                }

                if let Some(first) = paths.next() {
                    if !first.match_resource_type_name(&self.resource_name()) {
                        return Err(FhirError::Message(format!("路径中首个组成与当前资源类型[{}]不符", self.resource_name())))
                    }
                }

                let mut vv = self.as_collection2();
                while let Some(func) = paths.next() {
                    vv = vv.exec(&func, &mut paths)?;
                    println!("Response: {:?}", &vv)
                }

                match vv {
                    PathResponse::Collection(collection) => Ok(collection),
                    _ => unreachable!()
                }
            }
        }

        impl Base for #struct_name_ident {}
    );
    Ok(ret)
}

fn impl_domain_resource_trait(struct_name_ident: &syn::Ident) -> syn::Result<proc_macro2::TokenStream> {

    let ret = quote::quote!(
        impl DomainResource for #struct_name_ident {
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

            fn modifier_extension(&self) -> &Option<Vec<Extension>> {
                &self.modifier_extension
            }

            fn set_modifier_extension(mut self, ext: Vec<Extension>) -> Self {
                self.modifier_extension = Some(ext);
                self
            }

            fn add_modifier_extension(mut self, ext: Extension) -> Self {
                match self.modifier_extension {
                    Some(ref mut exts) => {
                        exts.push(ext);
                    },
                    None => {
                        self.modifier_extension = Some(vec![ext])
                    },
                }
                self
            }
        }
    );
    Ok(ret)
}

fn impl_resource(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let fns = impl_resource_fields(struct_fields)?;

    let ret = quote::quote!(
        impl #struct_name_ident {
            #( #fns )*
        }
    );
    Ok(ret)
}

/// 为资源的每一个字段，增加一个赋值函数。
/// 如果字段类型为Primitive类型，则赋值函数的参数为impl Into<T>
/// 如果字段类型为Vec类型，则分配两个赋值函数：set_xxx 和 add_xxx
fn impl_resource_fields(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .skip(8)
        .for_each(|f| {
            let ident = &f.name;
            let typ = &f.ty;

            // 赋值类型为Option的内部类型
            let set_func = format_ident!("set_{}",  &ident);
            let value_type = helper::option_inner(typ).unwrap();

            if helper::is_primitive(value_type) {
                fields.push(quote::quote!(
                    pub fn #set_func<T: Into<#value_type>>(mut self, v: T) -> Self {
                        self.#ident = Some(v.into());
                        self
                    }
                ));
            } else {
                fields.push(quote::quote!(
                    pub fn #set_func(mut self, v: #value_type) -> Self {
                        self.#ident = Some(v);
                        self
                    }
                ));
            };

            // 如果类型是Vec，添加形如add_xxxx的函数，参数为Vec的内部类型
            if let Some(v_typ) = helper::vector_inner(value_type) {
                let add_func = format_ident!("add_{}", &ident);
                if helper::is_primitive(v_typ) {
                    fields.push(quote::quote!(
                        pub fn #add_func<T: Into<#v_typ>>(mut self, v: T) -> Self {
                            match self.#ident {
                                None => {
                                    let mut vec = Vec::new();
                                    vec.push(v.into());
                                    self.#ident = Some(vec);
                                }
                                Some(ref mut vec) => {
                                    vec.push(v.into());
                                }
                            }
                            self
                        }
                    ));
                } else {
                    fields.push(quote::quote!(
                        pub fn #add_func(mut self, v: #v_typ) -> Self {
                            match self.#ident {
                                None => {
                                    let mut vec = Vec::new();
                                    vec.push(v);
                                    self.#ident = Some(vec);
                                }
                                Some(ref mut vec) => {
                                    vec.push(v);
                                }
                            }
                            self
                        }
                    ));
                }
            }
        });

    Ok(fields)
}

fn impl_serialize(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let struct_name_literal = struct_name_ident.to_string();
    let fields = impl_serialize_fields(struct_fields)?;

    let ret = quote::quote!(
        impl Serialize for #struct_name_ident {
            fn serialize<Ser: Serializer>(&self, serializer: Ser) -> Result<()> {
                let mut serialize_struct = serializer.serialize_resource( #struct_name_literal )?;
                serialize_struct.serialize_id(&self.id)?;
                #( #fields )*
                serialize_struct.serialize_end()
            }
        }
    );
    Ok(ret)
}

/// 在序列化函数中，用于生成所有字段（除id之外）
/// 在资源中id是需要特殊处理的，skip(1)就是跳过id
fn impl_serialize_fields(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut fields = Vec::with_capacity(32);

    struct_fields.iter()
        .skip(1)
        .for_each(|f| {
            let name = &f.name;
            let name_literal = &f.original;
            fields.push(quote::quote!(
                serialize_struct.serialize_field(#name_literal, &self.#name)?;
            ));
        });

    Ok(fields)
}

fn impl_deserialize(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let visitor = helper::visitor(struct_name_ident)?;

    let fields = helper::impl_deserialize_fields(struct_fields)?;
    let defs = helper::impl_deserialize_define(struct_fields)?;
    let maps = helper::impl_deserialize_map(struct_fields)?;

    let ret = quote::quote!(
        impl<'de> Deserialize<'de> for #struct_name_ident {
            fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
                pub struct #visitor;
                impl<'de> Visitor<'de> for #visitor {
                    type Value = #struct_name_ident;

                    fn visit_map<M>(self, mut map: M) -> Result<Self::Value> where M: MapAccess<'de> {
                        #( #defs )*

                        while let Some(keys) = map.next_key()? {
                            match keys.as_str() {
                                #( #maps )*
                                _ => {return Err(FhirError::error_string(format!("读到不存在的键:{}", keys)));},
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

pub fn impl_fhirpath_map(struct_fields: &Vec<Field>) -> syn::Result<Vec<proc_macro2::TokenStream>> {
    let mut maps = Vec::with_capacity(32);

    struct_fields.iter()
        .for_each(|field| {
            let ident = field.name.clone();
            let ident_literal = field.original.clone();
            maps.push(quote::quote!( #ident_literal => { self.#ident.exec(&func, paths) }, ));
        });

    Ok(maps)
}

pub fn impl_fhirpath(struct_name_ident: &syn::Ident, struct_fields: &Vec<Field>) -> syn::Result<proc_macro2::TokenStream> {
    let maps = impl_fhirpath_map(struct_fields)?;

    let ret = quote::quote!(

        impl Executor for #struct_name_ident {
            fn exec(&self, func: &Function, paths: &mut FhirPaths) -> Result<PathResponse> {
                match func.definition.function_name() {
                    FunctionName::Element => {
                        match &func.params {
                            FunctionParam::String(name) => {
                                match name.as_str() {
                                    #( #maps )*
                                    other => Err(FhirError::Message(format!("无效的路径名:[{}]", other)))
                                }
                            },
                            _ => unreachable!(),
                        }
                    },
                    _ => Err(FhirError::Message(format!("Patient: 无效的函数名:{:?}", &func))),
                }
            } 

            fn as_collection(&self) -> Collection {
                Collection(vec![Box::new(self.clone())])
            }
        }
    );
    Ok(ret)
}



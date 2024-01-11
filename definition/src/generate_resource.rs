use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::slice::Iter;
use anyhow::anyhow;
use convert_case::{Case, Casing};
use xml::{EventReader, ParserConfig};
use xml::reader::XmlEvent;

#[derive(Clone, Debug, Default)]
struct Resource {
    name: Option<String>,
    fields: Vec<Field>,
}

#[derive(Clone, Debug, Default)]
struct Field {
    name: Option<String>,
    typ: Option<String>,
    min: Option<String>,
    max: Option<String>,
    short: Option<String>,
    modifier: Option<String>,
    summary: Option<String>,
    content_reference: Option<String>,
}

pub struct DefinitionParser<R: Read> {
    reader: EventReader<R>,
    names: Vec<String>,
    resources: Vec<Resource>,
}
impl<R: Read> DefinitionParser<R> {

    pub fn new(read: R) -> Self {
        let config = ParserConfig::new()
            .trim_whitespace(true)
            .ignore_comments(true);

        let reader = EventReader::new_with_config(read, config);
        DefinitionParser{
            reader,
            names: Vec::with_capacity(256),
            resources: Vec::with_capacity(256),
        }
    }

    pub fn work_through(&mut self) -> anyhow::Result<()> {
        let sd = "StructureDefinition".to_string();

        while let next = self.reader.next()? {
            match next {
                XmlEvent::StartElement { name, .. }
                if name.local_name.starts_with("StructureDefinition") => {
                    tracing::info!("find StructureDefinition");
                    self.struct_definition()?;
                },
                XmlEvent::EndDocument => {
                    break;
                }
                _ => {}
            }
        }

        tracing::debug!("处理资源完成,共有资源{}个", self.resources.len());

        Ok(())
    }

    fn struct_definition(&mut self) -> anyhow::Result<()> {

        while let next = self.reader.next()? {
            match next {
                XmlEvent::StartElement { name,  .. }
                if name.local_name.starts_with("snapshot") => {
                    let resource = self.snapshot()?;
                    tracing::debug!("处理完成一个资源:{:?}", &resource.name);

                    let rname = &resource.name;
                    if let Some(nm) = rname {
                        match nm.as_str() {
                            "Resource"|"CanonicalResource"|"DomainResource"|"MetadataResource" => {},
                            _ => {
                                self.names.push(resource.name.clone().unwrap());
                                self.resources.push(resource);
                            }
                        }
                    }
                },
                XmlEvent::EndElement { name}
                if name.local_name.starts_with("StructureDefinition") => {
                    break;
                },
                XmlEvent::EndDocument => {
                    break;
                },
                _ => {}
            }
        };

        Ok(())
    }

    fn snapshot(&mut self) -> anyhow::Result<Resource> {
        let mut resource = Resource::default();

        let element = "element".to_string();
        while let next = self.reader.next()? {
            match next {
                XmlEvent::StartElement { name, attributes, .. }
                if name.local_name == element => {
                    resource.fields.push(self.element()?);
                },
                XmlEvent::EndElement { name }
                if name.local_name == element => {
                    break;
                },
                _ => {
                    break;
                }
            }
        };

        resource.name = resource.fields[0].name.clone();

        Ok(resource)
    }

    fn element(&mut self) -> anyhow::Result<Field> {
        let mut field = Field::default();

        loop {
            match self.reader.next()? {
                XmlEvent::StartElement { name, attributes, .. } => {
                    match name.local_name.as_str() {
                        "path" => {
                            field.name = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        "short" => {
                            field.short = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        "min" =>  {
                            field.min = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        "max" =>  {
                            field.max = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        "type" => {
                            field.typ = Some(self.typed()?);
                        },
                        "isModifier" => {
                            field.modifier = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        "isSummary" =>  {
                            field.summary = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        "contentReference" =>  {
                            field.content_reference = Some(attributes[0].value.clone());
                            self.reader.skip()?;
                        },
                        _ => {
                            self.reader.skip()?;
                        }
                    }
                },
                _ => {
                    break;
                }
            }
        };

        Ok(field)
    }


    // <type>
    //   <extension url="http://hl7.org/fhir/StructureDefinition/structuredefinition-fhir-type">
    //     <valueUrl value="code"/>
    //   </extension>
    //   <extension url="http://hl7.org/fhir/StructureDefinition/regex">
    //     <valueString value="[^\s]+( [^\s]+)*"/>
    //   </extension>
    //   <code value="http://hl7.org/fhirpath/System.String"/>
    // </type>

    // <type>
    //   <code value="Extension"/>
    // </type>
    fn typed(&mut self) -> anyhow::Result<String> {

        let mut type1 = None;
        let mut type2 = None;

        while let next = self.reader.next()? {
            match next {
                XmlEvent::StartElement { name, attributes, .. }
                if name.local_name.starts_with("valueUrl") => {
                    let tt = attributes[0].value.clone();
                    type1 = Some(tt.to_case(Case::Pascal));
                },
                XmlEvent::StartElement { name, attributes, .. }
                if name.local_name.starts_with("code") => {
                    let typ = attributes[0].value.clone();
                    type2 = if is_lowercase(typ.as_str()) {
                        Some(format!("{}Dt",typ.to_case(Case::Pascal)))
                    } else {
                        Some(typ)
                    };
                }
                XmlEvent::EndElement { name }
                if name.local_name.starts_with("type") => {
                    break;
                }
                _ => {}
            }
        }

        Ok(type1.unwrap_or(type2.unwrap_or("None".to_string())))
    }

    pub fn write_mod<W: Write>(&mut self, mut writer: W) -> anyhow::Result<()> {

        self.names
            .iter()
            .for_each(|name| {
                writeln!(writer, "mod {};", name.to_case(Case::Snake)).expect("TODO: panic message");
                writeln!(writer, "pub use {}::*;", name.to_case(Case::Snake)).expect("TODO: panic message");
            });

        writeln!(writer, "").expect("TODO: panic message");
        writeln!(writer, "pub enum Resource {{").expect("TODO: panic message");
        self.names
            .iter()
            .for_each(|name| {
                writeln!(writer, "{0}({0}),", name).expect("TODO: panic message");
            });
        writeln!(writer, "}}").expect("TODO: panic message");

        writer.flush()?;
        tracing::debug!("写入mod结束");

        Ok(())
    }

    pub fn write_resources(&mut self, path: &PathBuf) -> anyhow::Result<()> {

        self.resources
            .iter()
            .for_each(|mut resource| {
                self.write_resource(path, resource).expect("TODO: Error Message");
            });

        Ok(())
    }

    fn write_resource(&self, path: &PathBuf, resource: &Resource) -> anyhow::Result<()> {
        let resource_name = resource.name.clone().unwrap();

        let file_name = resource_name.to_case(Case::Snake);
        let mut file = File::create(path.join(file_name).with_extension("rs"))?;

        tracing::debug!("Start Write Resource {}...", &resource_name);

        writeln!(file, "use fhir_rs::prelude::*;")?;
        writeln!(file, "use crate::Resource;")?;
        writeln!(file, "")?;

        let mut iter = resource.fields.iter();
        if let Some(field) = iter.next().take() {
            self.write_struct(file, field, &mut iter)?;
        };

        Ok(())
    }

    fn write_struct(&self, mut writer: File, struct_field: &Field, iter: &mut Iter<Field>) -> anyhow::Result<()> {

        let mut structs: HashMap<String, Vec<Field>> = HashMap::with_capacity(8);
        //
        let struct_field_name = struct_field.name.clone().unwrap();
        writeln!(writer, "#[derive(Resource, Debug, Clone, Default)]")?;
        writeln!(writer, "pub struct {} {{", &struct_field_name)?;

        while let Some(field) = iter.next().take() {
            let field_name = field.name.clone().unwrap();
            tracing::debug!("原始Field Name: {}", &field_name);

            let mut field_type = match (&field.typ, &field.content_reference) {
                (Some(ty), _) => {
                    ty.to_case(Case::Pascal)
                },
                (_, Some(cr)) => {
                    let tt = cr.replace("#", "").split(".")
                        .into_iter()
                        .map(|s| s.to_case(Case::Pascal))
                        .collect::<Vec<String>>()
                        .join("");
                    format!("{}BackboneElement", tt)
                },
                _ => {
                    return Err(anyhow!("ddddddd"));
                }
            };

            let replace_name = format!("{}.", &struct_field_name);
            let ffn = field_name
                .replacen(replace_name.as_str(), "", 1)
                .replace("[x]", "");
            tracing::debug!("替换后的Field Name: {}", &ffn);

            let split_ffn: Vec<&str> = ffn.split(".").collect();
            if split_ffn.len() > 1 {
                structs.get_mut(split_ffn[0]).unwrap().push(field.clone());
                continue;
            }

            if field_type == "BackboneElement" {
                structs.insert(ffn.clone(), Vec::with_capacity(64));
                field_type = format!("{}{}BackboneElement", &struct_field_name.to_case(Case::Pascal), &ffn.to_case(Case::Pascal));
            }

            if let Some(b) = &field.max {
                if b.as_str() == "*" {
                    field_type = format!("Vec<{}>", field_type);
                }
            }

            writeln!(writer, "    /// {}", &field.short.clone().unwrap_or("".to_string())).expect("TODO: panic message");
            writeln!(writer,
                     "    #[fhir(name=\"{}\", min=\"{}\", max=\"{}\", summary=\"{}\", modifier=\"{}\")]",
                     &ffn,
                     field.min.clone().unwrap_or("0".to_string()),
                     field.max.clone().unwrap_or("0".to_string()),
                     field.summary.clone().unwrap_or("".to_string()),
                     field.modifier.clone().unwrap_or("".to_string()))
                .expect("TODO: panic message");

            let next_ffn = match ffn.as_str() {
                "type"|"use"|"abstract"|"for"|"const" => {
                    format!("{}_", ffn.clone())
                },
                _ => {ffn.clone().to_case(Case::Snake)}
            };
            writeln!(writer, "    pub {}: Option<{}>,", next_ffn, field_type).expect("TODO: panic message");
        };

        writeln!(writer, "}}")?;
        writeln!(writer, "")?;

        for (key, vec) in structs {
            let parent = vec![struct_field_name.clone(), key];
            self.write_sub_struct(&mut writer, parent, vec)?;
        }

        Ok(())
    }

    fn write_sub_struct(&self, writer: &mut File, parent: Vec<String>, fields: Vec<Field>) -> anyhow::Result<()> {
        tracing::debug!("处理子类: {:?}", &parent);
        let mut structs: HashMap<String, Vec<Field>> = HashMap::with_capacity(8);

        let parent_name = parent
            .iter()
            .map(|s| s.to_case(Case::Pascal))
            .collect::<Vec<String>>()
            .join("");
        let struct_field_name = format!("{}BackboneElement", parent_name);
        writeln!(writer, "#[derive(BackboneElement, Debug, Clone, Default)]")?;
        writeln!(writer, "pub struct {} {{", &struct_field_name)?;

        for field in fields {
            let field_name = field.name.clone().unwrap();
            tracing::debug!("子类字段: {:?}", field_name);

            let mut field_type = match (&field.typ, &field.content_reference) {
                (Some(ty), _) => {
                    ty.to_case(Case::Pascal)
                },
                (_, Some(cr)) => {
                    let tt = cr.replace("#", "").split(".")
                        .into_iter()
                        .map(|s| s.to_case(Case::Pascal))
                        .collect::<Vec<String>>()
                        .join("");
                    format!("{}BackboneElement", tt)
                },
                _ => {
                    return Err(anyhow!("ddddddd"));
                }
            };

            let replace_name = parent.join(".");
            let replace_name = format!("{}.", replace_name);
            tracing::debug!("替换因子: {}", &replace_name);

            let ffn = field_name
                .replacen(replace_name.as_str(), "", 1)
                .replace("[x]", "");
            tracing::debug!("替换后的Field Name: {}", &ffn);

            let split_ffn: Vec<&str> = ffn.split(".").collect();
            if split_ffn.len() > 1 {
                structs.get_mut(split_ffn[0]).unwrap().push(field.clone());
                continue;
            }

            if field_type == "BackboneElement" {
                structs.insert(ffn.clone(), Vec::with_capacity(64));
                field_type = format!("{}{}BackboneElement", &parent_name.to_case(Case::Pascal), &ffn.to_case(Case::Pascal));
            }

            if let Some(b) = &field.max {
                if b.as_str() == "*" {
                    field_type = format!("Vec<{}>", field_type);
                }
            }

            writeln!(writer, "    /// {}", &field.short.clone().unwrap_or("".to_string())).expect("TODO: panic message");
            writeln!(writer,
                     "    #[fhir(name=\"{}\", min=\"{}\", max=\"{}\", summary=\"{}\", modifier=\"{}\")]",
                     &ffn,
                     field.min.clone().unwrap_or("0".to_string()),
                     field.max.clone().unwrap_or("0".to_string()),
                     field.summary.clone().unwrap_or("".to_string()),
                     field.modifier.clone().unwrap_or("".to_string()))
                .expect("TODO: panic message");

            let next_ffn = match ffn.as_str() {
                "type"|"use"|"abstract"|"for"|"const" => {
                    format!("{}_", ffn.clone())
                },
                _ => {ffn.clone().to_case(Case::Snake)}
            };

            writeln!(writer, "    pub {}: Option<{}>,", next_ffn, field_type).expect("TODO: panic message");
        }

        writeln!(writer, "}}")?;
        writeln!(writer, "")?;

        for (key, vec) in structs {
            let mut new_parent = Vec::new();
            new_parent.extend(parent.clone());
            new_parent.push(key);
            self.write_sub_struct(writer, new_parent, vec)?;
        }

        Ok(())
    }
}

fn is_lowercase(s: &str) -> bool {
    if let Some((first_char)) = s.chars().next() {
        first_char >= 'a' && first_char <= 'z'
    } else {
        false // 如果字符串为空则返回false
    }
}
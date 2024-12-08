use std::collections::VecDeque;
use std::io::{BufRead, Cursor};
use xml::{
    ParserConfig,
    reader::{EventReader, XmlEvent}
};

use super::*;

pub fn from_str<'a, T>(s: &'a str) -> Result<T>
    where
        T: Deserialize<'a>,
{
    from_reader(Cursor::new(s))
}

fn from_reader<'de, R, T>(read: R) -> Result<T>
    where
        R: BufRead,
        T: Deserialize<'de>,
{
    let mut deserializer = XmlDeserializer::from_reader(read)?;
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

#[derive(Debug)]
pub enum XmlNode {
    ObjectKey(String),
    Object{ name: String, events: VecDeque<XmlNode>},
    AttributeKey(String),
    Text(String),
    Value{key: String, value: String},
}

pub struct XmlDeserializer {
    resource: String,
    buffer: VecDeque<XmlNode>,
}

impl XmlDeserializer {
    pub fn from_reader<R: BufRead>(read: R) -> Result<Self> {
        let config = ParserConfig::new().trim_whitespace(true).ignore_comments(true);
        let mut reader = EventReader::new_with_config(read, config);

        let event = reader.next()?;
        match event {
            XmlEvent::StartDocument { encoding, ..} => {
                println!("encoding: {}", &encoding.to_ascii_lowercase());
                if &encoding.to_ascii_lowercase() != "utf-8" {
                    return Err(FhirError::Message(format!( "{} is not valid fhir Encoding", encoding)));
                }
            },
            _ => return Err(FhirError::error("Xml string is not valid XmlDocument")),
        }

        let event = reader.next()?;
        match event {
            XmlEvent::StartElement { name, .. } => {
                let buffer = Self::read_object(&mut reader, &name.local_name)?;
                Ok(Self{ resource: name.local_name, buffer})
            }
            _ => Err(FhirError::error("Xml element is not valid XmlDocument")),
        }
    }

    pub fn from_event(resource: String, buffer: VecDeque<XmlNode>) -> Self {
        Self{ resource, buffer }
    }

    fn read_object<R: BufRead>(reader: &mut EventReader<R>, element_name: &String) -> Result<VecDeque<XmlNode>> {
        let mut events = VecDeque::new();

        loop {
            let event = reader.next()?;
            match event {
                XmlEvent::StartElement { name, attributes, .. } => {
                    let mut nodes = Self::read_object(reader, &name.local_name)?;
                    events.push_back(XmlNode::ObjectKey(name.local_name.to_owned()));
                    for attribute in attributes {
                        nodes.push_back(XmlNode::AttributeKey(attribute.name.local_name.clone()));
                        nodes.push_back(XmlNode::Value{key: attribute.name.local_name, value: attribute.value});
                    }
                    events.push_back(XmlNode::Object{ name: name.local_name.to_owned(), events: nodes});
                },
                XmlEvent::CData(_) |
                XmlEvent::StartDocument { .. } |
                XmlEvent::ProcessingInstruction { .. } |
                XmlEvent::Whitespace { .. } |
                XmlEvent::Comment(_) => { /* skip */ }
                XmlEvent::Characters(s) => { events.push_back(XmlNode::Text(s)); }
                XmlEvent::EndElement { name, .. } => {
                    if element_name != &name.local_name {
                        return Err(FhirError::Message(format!("End tag </{}> didn't match the start tag <{}>", name.local_name, element_name))) ;
                    }
                    break
                },
                XmlEvent::EndDocument => unreachable!(),
            }
        }

        Ok(events)
    }

    fn next_key(&mut self) -> Option<String> {
        loop {
            match self.next() {
                None => return None,
                Some(node) => {
                    match node {
                        XmlNode::ObjectKey(key) => return Some(key),
                        XmlNode::AttributeKey(key) => return Some(key),
                        _ => continue,
                    }
                }
            }
        }
    }

    fn next(&mut self) -> Option<XmlNode> {
        self.buffer.pop_front()
    }

    fn events(&mut self) -> VecDeque<XmlNode> {
        self.buffer.drain(..).collect()
    }

    fn narrative(&self, name: String, events: VecDeque<XmlNode>, namespace: Option<String>) -> String {
        let mut current_sub_element = vec![];
        let mut current_attributes = vec![];

        for event in events {
            match event {
                XmlNode::Object { name, events } => {
                    let ss = self.narrative(name, events, None);
                    current_sub_element.push(ss);
                }
                XmlNode::Text(text) => current_sub_element.push(Self::xhtml_encode(text)),
                XmlNode::Value{key, value} => current_attributes.push(format!("{}=\"{}\"", key, value)),
                _ => {}
            }
        }

        format!("<{}{}{}>{}</{}>",
            name,
            match namespace {
                Some(ns) => format!(" xmlns=\"{}\"", ns),
                None => "".to_string(),
            },
            if current_attributes.is_empty() {String::new()} else {format!(" {}", current_attributes.join(" "))},
            current_sub_element.join(""),
            name
        )
    }

    fn array(&mut self) -> Result<VecDeque<XmlNode>> {
        let mut vec = VecDeque::new();
        let mut matched_name = None;

        while let Some(node) = self.next() {
           match &node {
               XmlNode::Object{ name, .. } => {
                   matched_name = Some(name.to_owned());
                   vec.push_back(node);
               },
               XmlNode::ObjectKey(key) => {
                    match &matched_name {
                        None => return Err(FhirError::Message(format!("{}", key))),
                        Some(name) => {
                            if name != key {
                                self.buffer.push_front(node);
                                break
                            }
                        }
                    }
               }
               XmlNode::AttributeKey(_) | XmlNode::Text(_) | XmlNode::Value{..} => {
                   self.buffer.push_front(node);
                   break
               },
           }
        }

        Ok(vec)
    }

    fn empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn xhtml_encode(input: String) -> String {
        input.chars().map(|c| match c {
            '&' => "&amp;".to_string(),
            '<' => "&lt;".to_string(),
            '>' => "&gt;".to_string(),
            '"' => "&quot;".to_string(),
            '\'' => "&#x27;".to_string(),
            '/' => "&#x2F;".to_string(),
            _ => c.to_string(),
        }).collect()
    }
}

#[test]
fn test_xml_deserializer() -> Result<()> {
    let script_str = include_str!("../../../examples/transform/testscript.xml");
    let s = Cursor::new(script_str);
    let de = XmlDeserializer::from_reader(s)?;

    let buffer = de.buffer.into_iter().collect::<Vec<_>>();

    println!("{:?}", buffer);
    Ok(())
}

impl<'de> Deserializer<'de> for &mut XmlDeserializer {
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        match self.next() {
            None => Err(FhirError::error("在尝试获取字符串时读到EOF")),
            Some(node) => {
                match node {
                    XmlNode::Value{value, ..} => visitor.visit_str(value.as_str()),
                    _ => Err(FhirError::error("在尝试获取字符串时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_number<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        match self.next() {
            None => Err(FhirError::error("在尝试获取数值时读到EOF")),
            Some(node) => {
                match node {
                    XmlNode::Value{value, ..} => visitor.visit_str(value.as_str()),
                    _ => Err(FhirError::error("在尝试获取数值时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        debug!("开始处理Map了");
        let buffer = self.events();
        visitor.visit_vec( XmlMapProcessor::from_event("".into(), buffer))
    }

    fn deserialize_vec<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        debug!("开始处理数组了");
        let buffer = self.array()?;
        visitor.visit_vec( XmlMapProcessor::from_event("".into(), buffer))
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        debug!("开始处理枚举类型");
        visitor.visit_enum(&self.resource.clone(), self)
    }

    fn deserialize_narrative<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        debug!("开始处理Narrative了");
        match self.next() {
            None => Err(FhirError::error("在尝试获取Narrative时读到EOF")),
            Some(node) => {
                match node {
                    XmlNode::Object{ name, events} => {
                        for event in &events {
                            println!("{:#?}", event);
                        }
                        let xhtml = self.narrative(name, events, Some(String::from("http://www.w3.org/1999/xhtml")));
                        visitor.visit_str(xhtml.as_str())
                    },
                    _ => Err(FhirError::error("在尝试获取Narrative时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        debug!("开始处理结构体");

        match self.next() {
            None => Err(FhirError::error("在尝试获取结构体时读到EOF")),
            Some(node) => {
                match node {
                    XmlNode::Object{ name, events} => {
                        visitor.visit_map(XmlMapProcessor::from_event(name, events))
                    },
                    _ => Err(FhirError::error("在尝试获取结构体时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_resource<V>(self, name: &str, visitor: V) -> Result<V::Value> where V: Visitor<'de>{
        debug!("开始处理资源");

        if name != &self.resource {
            return Err(FhirError::Message(format!("Resource Type not matched: {} and {}", name, &self.resource)));
        }

        visitor.visit_map(XmlMapProcessor::from_event(self.resource.to_owned(), self.events()))
    }

    fn deserialize_primitive<V>(self, name: &str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.deserialize_struct(name, visitor)
    }
}

pub struct XmlMapProcessor {
    de: XmlDeserializer,
}

impl XmlMapProcessor {
    fn from_event(resource: String, buffer: VecDeque<XmlNode>) -> Self {
        let de = XmlDeserializer::from_event(resource, buffer);
        XmlMapProcessor { de }
    }
}

impl<'de> MapAccess<'de> for XmlMapProcessor {
    fn next_key(&mut self) -> Result<Option<String>> {
        let value = self.de.next_key();
        tracing::debug!("读取到key: {:?}", &value);
        Ok(value)
    }

    fn next_value<De>(&mut self) -> Result<De> where De: Deserialize<'de> {
        De::deserialize(&mut self.de)
    }
}

impl<'de> VecAccess<'de> for XmlMapProcessor {
    fn next_element<T>(&mut self) -> Result<Option<T>> where T: Deserialize<'de> {
        if self.de.empty() {
            return Ok(None);
        }
        Ok(Some(T::deserialize(&mut self.de)?))
    }
}

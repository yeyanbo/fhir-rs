use std::collections::VecDeque;
use std::io::{BufRead, Cursor};

use xml::{
    ParserConfig,
    reader::{EventReader, XmlEvent},
    name::OwnedName,
    attribute::OwnedAttribute,
    namespace::Namespace,
};

use crate::prelude::*;


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
    let mut deserializer = XmlDeserializer::from_reader(read);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

pub struct XmlDeserializer<R: BufRead>{
    reader: EventReader<R>,
    buffer: VecDeque<XmlEvent>,
}

impl<'a, R> XmlDeserializer <R>
    where R: BufRead
{
    pub fn from_reader(read: R) -> Self {
        let config = ParserConfig::new()
            .trim_whitespace(true)
            .ignore_comments(true);
        
        XmlDeserializer{
            reader: EventReader::new_with_config(read, config),
            buffer: VecDeque::new(),
        }
    }
    
    pub fn next_key(&mut self) -> Result<Option<String>> {
        let event = self.next()?;
        match event {
            XmlEvent::StartElement { name, attributes, namespace } => {
                let key = name.local_name;

                self.push_element_start_event(key.clone(), namespace.clone());
                // 检测有没有属性值
                if let Some(kv) = self.read_attribute_from_element(&attributes) {
                    self.push_text_event(kv.0, kv.1, namespace.clone());
                }

                return Ok(Some(key));
            }
            XmlEvent::EndElement { name, .. } => {
                tracing::debug!("元素的Map结束: {:?}", name);
                return Ok(None)
            }
            _ => {
                return Err(FhirError::error(format!("获取到未预期的元素: {:?}", &event).as_str()));
            }
        };
    }

    pub fn next_element(&mut self) -> Result<Option<&XmlEvent>> {
        let event = self.next()?;
        match event {
            XmlEvent::StartElement { name, attributes, namespace } => {
                self.push_element_start_event(name.local_name, namespace.clone());

                // 检测有没有属性值
                if let Some(kv) = self.read_attribute_from_element(&attributes) {
                    self.push_text_event(kv.0, kv.1, namespace.clone());
                }

                return Ok(Some(self.peek()?));
            }
            _ => {
                self.buffer.push_back(event);
                return Ok(None)
            }
        };
    }

    /// 首先从缓冲区获取下一个元素，如果不存在，再通过reader获取
    /// 缓冲区为先进先出队列
    fn next(&mut self) -> Result<XmlEvent> {
        let event = match self.buffer.pop_front(){
            Some(event) => {Ok(event)}
            None => {
                self.read_useful_next_from_reader()
            }
        };

        tracing::debug!("POP: {:?}", &event);

        event
    }

    pub fn peek(&mut self) -> Result<&XmlEvent> {
        match self.buffer.front() {
            Some(_) => {}
            None => {
                let new = self.read_useful_next_from_reader()?;
                self.buffer.push_back(new);
            }
        };

        let event = self.buffer.front().unwrap();
        tracing::debug!("PEEK: {:?}", event);

        Ok(event)
    }

    /// 从reader中获取下一个有用的元素
    fn read_useful_next_from_reader(&mut self) -> Result<XmlEvent> {
        loop {
            match self.reader.next()? {
                XmlEvent::StartDocument { .. }
                | XmlEvent::ProcessingInstruction { .. }
                | XmlEvent::Whitespace { .. }
                | XmlEvent::Comment(_) => { /* skip */ }
                other => return Ok(other),
            }
        }
    }

    fn push_text_event(&mut self, key: String, value: String, namespace: Namespace) {
        let start = XmlEvent::StartElement {
            name: OwnedName::local(key.clone()),
            attributes: vec![],
            namespace,
        };
        let event = XmlEvent::Characters(value);

        let end = XmlEvent::EndElement {
            name: OwnedName::local(key),
        };

        tracing::debug!("PUSH: {:?}", &start);
        self.buffer.push_back(start);
        tracing::debug!("PUSH: {:?}", &event);
        self.buffer.push_back(event);
        tracing::debug!("PUSH: {:?}", &end);
        self.buffer.push_back(end);
    }

    fn push_element_start_event(&mut self, name: String, namespace: Namespace) {
        let event = XmlEvent::StartElement {
            name: OwnedName::local(name.clone()),
            attributes: vec![],
            namespace,
        };

        tracing::debug!("PUSH: {:?}", &event);

        self.buffer.push_front(event);
    }

    /// 从Element中获取属性值
    /// ## 规则
    /// 属性值只有一个，目前在规范中要么是value，要么是url（在Extension中）
    /// 
    /// ## 返回值
    /// 要么返回None，表示没有属性
    /// 要么返回一个Key-Value对
    fn read_attribute_from_element(&mut self, attributes: &Vec<OwnedAttribute>) -> Option<(String, String)> {
        match attributes.first() {
            None => {None}
            Some(attr) => {
                Some((attr.name.local_name.clone(), attr.value.clone()))
            }
        }
    }
}

impl<'de, 'a,  R: BufRead> Deserializer<'de> for &'a mut XmlDeserializer<R> {
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {

        self.next()?;
        let value = match self.next()? {
            XmlEvent::Characters(text) => {
                visitor.visit_str(text.as_str())?
            }
            _ => {
                return Err(FhirError::error("错误的字符串元素"));
            }
        };
        self.next()?;

        Ok(value)
    }

    fn deserialize_number<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        self.next()?;
        let value = match self.next()? {
            XmlEvent::Characters(text) => {
                visitor.visit_str(text.as_str())?
            }
            _ => {
                return Err(FhirError::error("错误的数值元素"));
            }
        };
        self.next()?;

        Ok(value)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        tracing::debug!("开始处理Map了");
        visitor.visit_map(XmlProcessor::new(self))
    }

    fn deserialize_vec<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        tracing::debug!("开始处理数组了");
        visitor.visit_vec( XmlVecProcessor::new(self))
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        tracing::debug!("开始处理枚举类型");
        loop {
            let root = self.peek()?;
            return match root {
                XmlEvent::StartElement { name, .. } => {
                    let value = visitor.visit_enum(name.local_name.clone().as_str(), self)?;
                    Ok(value)
                }
                _ => Err(FhirError::error("未读到XML的根元素"))
            }
        };
    }

    fn deserialize_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        tracing::debug!("开始处理结构体");
        loop{
            let event = self.next()?;
            return match event {
                XmlEvent::StartElement { .. } => {
                    let value = visitor.visit_map(XmlProcessor::new(self))?;
                    Ok(value)
                }
                _ => {
                    tracing::debug!("{:?}", &event);
                    Err(FhirError::error("未读到XML的根元素"))
                }
            };
        }
    }

    fn deserialize_primitive<V>(self, _name: &str, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        todo!()
    }
}

pub struct XmlProcessor<'a, R: BufRead> {
    de: &'a mut XmlDeserializer<R>,
}

impl<'a, R: BufRead> XmlProcessor<'a, R> {
    fn new(de: &'a mut XmlDeserializer<R>) -> Self {
        XmlProcessor { de }
    }
}

impl<'a, 'de, R: BufRead> MapAccess<'de> for XmlProcessor<'a, R> {
    fn next_key(&mut self) -> Result<Option<String>> {
        let value = self.de.next_key()?;
        tracing::debug!("读取到key: {:?}", &value);
        Ok(value)
    }

    fn next_value<De>(&mut self) -> Result<De> where De: Deserialize<'de> {
        De::deserialize(&mut *self.de)
    }
}

pub struct XmlVecProcessor<'a, R: BufRead> {
    de: &'a mut XmlDeserializer<R>,
    name: String,
}

impl<'a, R: BufRead> XmlVecProcessor<'a, R> {
    fn new(de: &'a mut XmlDeserializer<R>) -> Self {

        let event = de.peek().unwrap();
        let name = match event {
            XmlEvent::StartElement {name, .. } => {
                name.local_name.clone()
            }
            _ => { "Error".to_string()}
        };
        tracing::debug!("数组元素名称为:{:?}", &name);

        XmlVecProcessor {
            de,
            name
        }
    }
}

impl<'a, 'de, R:BufRead> VecAccess<'de> for XmlVecProcessor<'a, R> {
    fn next_element<T>(&mut self) -> Result<Option<T>> where T: Deserialize<'de> {

        let next = self.de.next_element()?;
        tracing::debug!("获取数组元素:{:?}", &next);

        match next {
            Some(XmlEvent::StartElement { name, .. })
                if name.local_name == self.name => {
                T::deserialize(&mut *self.de).map(Some)
            }
            Some(XmlEvent::StartElement { .. }) => {
                Ok(None)
            }
            Some(XmlEvent::EndElement { .. }) => {
                Ok(None)
            }
            None => {
                Ok(None)
            }
            _ => {
                Err(FhirError::error("======ddfdfdfsdfsdfssfs"))
            }
        }
    }
}

use std::collections::VecDeque;
use std::io::{BufRead, Cursor};
use json_event_parser::{JsonEvent, JsonReader};
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
    let mut deserializer = JsonDeserializer::from_reader(read);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

#[derive(Debug)]
pub enum NodeEvent {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
    StartArray,
    EndArray,
    StartObject,
    EndObject,
    ObjectKey(String),
    Eof,
}

pub struct JsonDeserializer<R: BufRead>{
    reader: JsonReader<R>,
    buffer: Vec<u8>,
    event_buffer: VecDeque<NodeEvent>,
}

impl<'a, R> JsonDeserializer<R>
    where R: BufRead
{
    pub fn from_reader(read: R) -> Self {
        JsonDeserializer{
            reader: JsonReader::from_reader(read),
            buffer: Vec::new(),
            event_buffer: VecDeque::new(),
        }
    }

    pub fn next_key(&mut self) -> Result<Option<String>> {
        let event = self.next()?;

        let value=match event {
            NodeEvent::ObjectKey(key) => Some(key),
            NodeEvent::EndObject => None,
            _ => {return Err(FhirError::error("在读取key时出现错误"));},
        };

        Ok(value)
    }

    fn next(&mut self) -> Result<NodeEvent> {
        let event = match self.event_buffer.pop_front(){
            Some(event) => {Ok(event)}
            None => {
                self.read_useful_next_from_reader()
            }
        };

        debug!("POP: {:?}", &event);
        event
    }

    pub fn lookup(&mut self) -> Result<&NodeEvent> {
        let node = self.read_useful_next_from_reader()?;
        self.event_buffer.push_back(node);

        let last = self.event_buffer.back().unwrap();
        Ok(last)
    }

    pub fn read_useful_next_from_reader(&mut self) -> Result<NodeEvent> {
        let event = self.reader.read_event(&mut self.buffer)?;
        let node = match event {
            JsonEvent::String(s) => NodeEvent::String(String::from(s)),
            JsonEvent::Number(s) => NodeEvent::Number(String::from(s)),
            JsonEvent::Boolean(b) => NodeEvent::Boolean(b),
            JsonEvent::Null => NodeEvent::Null,
            JsonEvent::StartArray => NodeEvent::StartArray,
            JsonEvent::EndArray => NodeEvent::EndArray,
            JsonEvent::StartObject => NodeEvent::StartObject,
            JsonEvent::EndObject => NodeEvent::EndObject,
            JsonEvent::ObjectKey(s) => NodeEvent::ObjectKey(String::from(s)),
            JsonEvent::Eof => NodeEvent::Eof,
        };
        Ok(node)
    }
}

impl<'a, 'de, R: BufRead> Deserializer<'de> for &'a mut JsonDeserializer<R> {
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let event = self.next()?;

        match event {
            NodeEvent::String(s) => {
                visitor.visit_str(s.as_str())
            },
            NodeEvent::EndArray => {
                Err(FhirError::EndArrayWhileParsingList)
            },
            _ => {Err(FhirError::error("在获取字符串值时出现错误"))},
        }
    }

    fn deserialize_number<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let event = self.next()?;

        match event {
            NodeEvent::Number(s) => {
                visitor.visit_str(s.as_str())
            },
            NodeEvent::StartObject => {
                visitor.visit_map(JsonProcessor::new(self))
            },
            _ => {Err(FhirError::error("在获取数值时出现错误"))},
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_vec<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        tracing::debug!("开始处理数组了");

        let event = self.next()?;
        let value = match event {
            NodeEvent::StartArray => {
                tracing::debug!("开始按Vec处理");
                visitor.visit_vec(JsonProcessor::new(self))?
            }
            _ => {
                return Err(FhirError::error("必须以数组字符开头"));
            }
        };
        Ok(value)
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let expected = String::from("resourceType");

        loop {
            let event = self.lookup()?;
            let value = match event {
                NodeEvent::ObjectKey(key) if *key == expected => {
                    let node = self.lookup()?;
                    match node {
                        NodeEvent::String(resource_type) => {
                            let value = visitor.visit_enum(resource_type.clone().as_str(), self)?;
                            return Ok(value)
                        }
                        _ => return Err(FhirError::error("[resourceType]元素类型应为String类型")),
                    }
                },
                NodeEvent::EndObject | NodeEvent::Eof => break,
                _ => {}
            };
        }
        Err(FhirError::error("未找到代表资源类型的[resourceType]元素"))
    }

    fn deserialize_primitive<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        tracing::debug!("开始解析结构体");
        let event = self.next()?;

        match event {
            NodeEvent::StartObject => {
                tracing::debug!("开始按Map处理");
                visitor.visit_map(JsonProcessor::new(self))
            },
            NodeEvent::String(s) => {
                visitor.visit_str(s.as_str())
            },
            NodeEvent::Number(s) => {
                visitor.visit_str(s.as_str())
            },
            NodeEvent::Boolean(b) => {
                visitor.visit_str(b.to_string().as_str())
            },
            NodeEvent::EndArray => Err(FhirError::EndArrayWhileParsingList),
            _ => Err(FhirError::error("必须以对象字符开头")),
        }
    }

    fn deserialize_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        tracing::debug!("开始解析结构体");
        let event = self.next()?;

        let value = match event {
            NodeEvent::StartObject => {
                tracing::debug!("开始按Map处理");
                visitor.visit_map(JsonProcessor::new(self))?
            },
            NodeEvent::EndArray => {
                return Err(FhirError::EndArrayWhileParsingList);
            },
            _ => {
                return Err(FhirError::error("必须以对象字符开头"));
            }
        };

        Ok(value)
    }
}

pub struct JsonProcessor<'a, R: BufRead> {
    de: &'a mut JsonDeserializer<R>,
}

impl<'a, R: BufRead> JsonProcessor<'a, R> {
    fn new(de: &'a mut JsonDeserializer<R>) -> Self {
        JsonProcessor { de}
    }
}

impl<'a, 'de, R: BufRead> MapAccess<'de> for JsonProcessor<'a, R> {
    fn next_key(&mut self) -> Result<Option<String>> {
        tracing::debug!("读取key");

        let value = self.de.next_key()?;
        tracing::debug!("读取到key: {:?}", &value);
        Ok(value)
    }

    fn next_value<De>(&mut self) -> Result<De> where De: Deserialize<'de> {
        De::deserialize(&mut *self.de)
    }
}

impl<'a, 'de, R:BufRead> VecAccess<'de> for JsonProcessor<'a, R> {
    fn next_element<T>(&mut self) -> Result<Option<T>> where T: Deserialize<'de> {
        Ok(Some(T::deserialize(&mut *self.de)?))
    }
}
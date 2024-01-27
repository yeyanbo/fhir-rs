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

pub struct JsonDeserializer<R: BufRead>{
    reader: JsonReader<R>,
    buffer: Vec<u8>,
}

impl<'a, R> JsonDeserializer<R>
    where R: BufRead
{
    pub fn from_reader(read: R) -> Self {
        JsonDeserializer{
            reader: JsonReader::from_reader(read),
            buffer: Vec::new(),
        }
    }

    pub fn next_key(&'a mut self) -> Result<Option<String>> {
        let event = self.reader.read_event(&mut self.buffer)?;

        let value=match event {
            JsonEvent::ObjectKey(key) => {
                Some(String::from(key))
            }
            JsonEvent::EndObject => None,
            _ => {return Err(FhirError::error("在读取key时出现错误"));},
        };

        Ok(value)
    }
}

impl<'a, 'de, R: BufRead> Deserializer<'de> for &'a mut JsonDeserializer<R> {
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let event = self.reader.read_event(&mut self.buffer)?;

        match event {
            JsonEvent::String(s) => {
                visitor.visit_str(s)
            },
            JsonEvent::EndArray => {
                Err(FhirError::EndArrayWhileParsingList)
            },
            _ => {Err(FhirError::error("在获取字符串值时出现错误"))},
        }
    }

    fn deserialize_number<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        let event = self.reader.read_event(&mut self.buffer)?;

        match event {
            JsonEvent::Number(s) => {
                visitor.visit_str(s)
            },
            JsonEvent::StartObject => {
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

        let event = self.reader.read_event(&mut self.buffer)?;

        let value = match event {
            JsonEvent::StartArray => {
                tracing::debug!("开始按Vec处理");
                visitor.visit_vec(JsonProcessor::new(self))?
            }
            _ => {
                return Err(FhirError::error("必须以数组字符开头"));
            }
        };
        Ok(value)
        // visitor.visit_vec()
    }

    fn deserialize_enum<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de> {
        tracing::debug!("到这里了");

        let event = self.reader.read_event(&mut self.buffer)?;

        let value = match event {
            JsonEvent::StartObject => {
                tracing::debug!("开始按Map处理");
                visitor.visit_map(JsonProcessor::new(self))?
            },
            JsonEvent::EndArray => {
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
        tracing::debug!("读取value");
        // let value = self.de.next_value()?;
        De::deserialize(&mut *self.de)
    }
}

impl<'a, 'de, R:BufRead> VecAccess<'de> for JsonProcessor<'a, R> {
    fn next_element<T>(&mut self) -> Result<Option<T>> where T: Deserialize<'de> {
        Ok(Some(T::deserialize(&mut *self.de)?))
    }
}
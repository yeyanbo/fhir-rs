use std::collections::VecDeque;
use std::io::{BufRead, Cursor};
use json_event_parser::{JsonEvent, JsonReader};
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
    let mut deserializer = RootJsonDeserializer::from_reader(read)?;
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}


#[derive(Debug)]
pub enum Node {
    String(String),
    Number(String),
    Boolean(bool),
    Null,
    Array(VecDeque<Node>),
    Object{resource: Option<String>, event: VecDeque<Node>},
    ObjectKey(String),
}

pub trait NodeBuffer {
    fn next_key(&mut self) -> Option<String>;
    fn next(&mut self) -> Option<Node>;
    fn events(&mut self) -> &mut VecDeque<Node>;
    fn empty(&self) -> bool;
}


pub struct RootNodeBuffer {
    pub buffer: VecDeque<Node>,
}
impl RootNodeBuffer {
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }
}

impl NodeBuffer for RootNodeBuffer {
    fn next_key(&mut self) -> Option<String> {
        loop {
            match self.next() {
                None => return None,
                Some(node) => {
                    match node {
                        Node::ObjectKey(key) => return Some(key),
                        _ => continue,
                    }
                }
            }
        }
    }

    fn next(&mut self) -> Option<Node> {
        self.buffer.pop_front()
    }

    fn events(&mut self) -> &mut VecDeque<Node> {
        &mut self.buffer
    }

    fn empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

pub struct ChildNodeBuffer<'parent> {
    buffer: &'parent mut VecDeque<Node>,
}

impl<'parent> ChildNodeBuffer<'parent> {
    pub fn new(buffer: &'parent mut VecDeque<Node>) -> Self {
        Self { buffer }
    }
}

impl<'parent> NodeBuffer for ChildNodeBuffer<'parent> {
    fn next_key(&mut self) -> Option<String> {
        loop {
            match self.next() {
                None => return None,
                Some(node) => {
                    match node {
                        Node::ObjectKey(key) => return Some(key),
                        _ => continue,
                    }
                }
            }
        }
    }

    fn next(&mut self) -> Option<Node> {
        self.buffer.pop_front()
    }

    fn events(&mut self) -> &mut VecDeque<Node> {
        self.buffer
    }

    fn empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

type RootJsonDeserializer = JsonDeserializer<RootNodeBuffer>;
type ChildJsonDeserializer<'parent> = JsonDeserializer<ChildNodeBuffer<'parent>>;

#[derive(Debug)]
pub struct JsonDeserializer<B: NodeBuffer> {
    resource: Option<String>,
    buffer: B,
}

impl RootJsonDeserializer {
    /// 全部加载到Buffer中
    /// 特殊处理了数组和对象
    pub fn from_reader<R:BufRead>(read: R) -> Result<Self> {
        let mut reader= JsonReader::from_reader(read);
        let mut buffer: Vec<u8> = Vec::new();

        let event = reader.read_event(&mut buffer)?;
        match event {
            JsonEvent::StartObject => {
                let (resource, buffer) = Self::read_object(&mut reader)?;
                Ok(Self{ resource, buffer: RootNodeBuffer{buffer} })
            },
            _ => Err(FhirError::error("JSON string is not a Object"))
        }
    }
}

impl<'parent> ChildJsonDeserializer<'parent> {
    pub fn from_event(resource: Option<String>, event: &'parent mut VecDeque<Node>) -> Self {
        Self{resource, buffer: ChildNodeBuffer::new(event)}
    }
}

impl<B: NodeBuffer> JsonDeserializer<B> {
    /// 获取下一个Key
    /// 返回None表示Buffer清空
    /// 为什么用loop？
    /// 当遍历到不可识别的key，要跳过该key对应的value值
    pub fn next_key(&mut self) -> Option<String> {
        self.buffer.next_key()
    }

    /// 获取下一个Node
    fn next(&mut self) -> Option<Node> {
        self.buffer.next()
    }
    
    fn read_array<R: BufRead>(reader: &mut JsonReader<R>) -> Result<VecDeque<Node>> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut events = VecDeque::new();

        loop {
            let event = reader.read_event(&mut buffer)?;
            match event {
                JsonEvent::String(s) => events.push_back(Node::String(String::from(s))),
                JsonEvent::Number(s) => events.push_back(Node::Number(String::from(s))),
                JsonEvent::Boolean(b) => events.push_back(Node::Boolean(b)),
                JsonEvent::Null => events.push_back(Node::Null),
                JsonEvent::ObjectKey(s) => events.push_back(Node::ObjectKey(String::from(s))),
                JsonEvent::StartArray => {
                    let array = Self::read_array(reader)?;
                    events.push_back(Node::Array(array));
                },
                JsonEvent::StartObject => {
                    let (resource, event) = Self::read_object(reader)?;
                    events.push_back(Node::Object{resource, event});
                },
                JsonEvent::EndArray => break ,
                JsonEvent::EndObject| JsonEvent::Eof => unreachable!(),
            };
        }
        
        Ok(events)
    }

    /// 对象可能是资源、复合类型和带扩展的基本类型
    /// 如果是资源，则resource值为资源类型
    /// 复合类型和带扩展的基本类型时，resource为None
    fn read_object<R: BufRead>(reader: &mut JsonReader<R>) -> Result<(Option<String>, VecDeque<Node>)> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut events = VecDeque::new();
        let mut resource = None;

        loop {
            let event = reader.read_event(&mut buffer)?;
            match event {
                JsonEvent::String(s) => events.push_back(Node::String(String::from(s))),
                JsonEvent::Number(s) => events.push_back(Node::Number(String::from(s))),
                JsonEvent::Boolean(b) => events.push_back(Node::Boolean(b)),
                JsonEvent::Null => events.push_back(Node::Null),
                JsonEvent::StartArray => {
                    let array = Self::read_array(reader)?;
                    events.push_back(Node::Array(array));
                },
                JsonEvent::StartObject => {
                    let (resource, event) = Self::read_object(reader)?;
                    events.push_back(Node::Object{resource, event});
                },
                JsonEvent::ObjectKey(s) if(s == "resourceType") => {
                    let temp_event = reader.read_event(&mut buffer)?;
                    match temp_event {
                        JsonEvent::String(s) => resource = Some(String::from(s)),
                        _ => {return Err(FhirError::error("读取资源类型失败"))}
                    }
                },
                JsonEvent::ObjectKey(s) => events.push_back(Node::ObjectKey(String::from(s))),
                JsonEvent::EndObject => break ,
                JsonEvent::EndArray | JsonEvent::Eof => unreachable!(),
            };
        }
        
        Ok((resource, events))
    }
}

impl<'de, B: NodeBuffer> Deserializer<'de> for &mut JsonDeserializer<B> {
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        match self.next() {
            None => Err(FhirError::error("在尝试获取字符串时读到EOF")),
            Some(node) => {
                match node {
                    Node::String(s) => visitor.visit_str(s.as_str()),
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
                    Node::Number(s) => visitor.visit_str(s.as_str()),
                    _ => Err(FhirError::error("在尝试获取数值时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        todo!()
    }

    fn deserialize_vec<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        match self.next() {
            None => Err(FhirError::error("在尝试获取数组时读到EOF")),
            Some(node) => {
                match node {
                    Node::Array(mut event) => {
                        visitor.visit_vec(JsonProcessor::from_event(None, &mut event))
                    },
                    _ => Err(FhirError::error("在尝试获取数组时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {

        match self.resource.clone() {
            Some(resource) => {
                println!("helllllo {}", &resource);
                visitor.visit_enum(&resource, self)
            }
            None => Err(FhirError::error("未找到代表资源类型的[resourceType]元素"))
        }
    }

    fn deserialize_narrative<V>(self, visitor: V) -> Result<V::Value> where V: Visitor<'de> {
        debug!("开始处理Narrative了");
        self.deserialize_str(visitor)
    }

    fn deserialize_struct<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        match self.next() {
            None => Err(FhirError::error("在尝试获取结构体时读到EOF")),
            Some(node) => {
                match node {
                    Node::Object{resource, mut event } => {
                        visitor.visit_map(JsonProcessor::from_event(resource, &mut event))
                    },
                    _ => Err(FhirError::error("在尝试获取结构体时读到其它数据类型")),
                }
            }
        }
    }

    fn deserialize_resource<V>(self, name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        if let Some(resource) = &self.resource{
            if name != resource {
                return Err(FhirError::Message(format!("Resource Type not matched: {} and {}", name, resource)));
            }
        }

        visitor.visit_map(JsonProcessor::from_event(self.resource.clone(), &mut self.buffer.events()))
    }

    fn deserialize_primitive<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>
    {
        match self.next() {
            None => Err(FhirError::error("在尝试获取简单类型数据时读到EOF")),
            Some(node) => {
                match node {
                    Node::String(s) => visitor.visit_str(s.as_str()),
                    Node::Number(s) => visitor.visit_str(s.as_str()),
                    Node::Boolean(b) => visitor.visit_str(b.to_string().as_str()),
                    Node::Object { resource, mut event } => {
                        visitor.visit_map(JsonProcessor::from_event(resource, &mut event))
                    }
                    _ => Err(FhirError::error("在尝试获取简单类型数据时读到其它数据类型")),
                }
            }
        }
    }
}

pub struct JsonProcessor<'parent> {
    de: ChildJsonDeserializer<'parent>,
}

impl<'parent> JsonProcessor<'parent> {
    fn from_event(resource: Option<String>, event: &'parent mut VecDeque<Node>) -> Self {
        let de = ChildJsonDeserializer::from_event(resource, event);
        Self { de }
    }
}

impl<'parent, 'de> MapAccess<'de> for JsonProcessor<'parent> {
    fn next_key(&mut self) -> Result<Option<String>> {
        let value = self.de.next_key();
        debug!("读取到key: {:?}", &value);
        Ok(value)
    }

    fn next_value<De>(&mut self) -> Result<De> where De: Deserialize<'de> {
        De::deserialize(&mut self.de)
    }
}

impl<'parent, 'de> VecAccess<'de> for JsonProcessor<'parent> {
    fn next_element<T>(&mut self) -> Result<Option<T>> where T: Deserialize<'de> {
        if self.de.buffer.empty() {
            return Ok(None);
        }
        Ok(Some(T::deserialize(&mut self.de)?))
    }
}
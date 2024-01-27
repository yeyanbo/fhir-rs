use std::io::Write;
use json_event_parser::{JsonEvent, JsonWriter};
use crate::prelude::*;

pub fn to_string<Ser: Serialize>(value: &Ser) -> Result<String> {
    let mut buffer = Vec::with_capacity(128);
    to_writer(&mut buffer, value)?;

    let string = String::from_utf8(buffer)?;
    Ok(string)
}

pub fn to_writer<W: Write, S: Serialize>(writer: W, value: &S) -> Result<()> {
    let mut ser = JsonSerializer::from_writer(writer);
    value.serialize(&mut ser)
}

pub struct JsonSerializer<W: Write> {
    pub writer: JsonWriter<W>,
    root: bool,
    tags: Vec<String>,
}

impl<W> JsonSerializer<W>
    where W: Write
{
    pub fn from_writer(writer: W) -> Self {
        JsonSerializer{
            writer: JsonWriter::from_writer(writer),
            root: true,
            tags: Vec::with_capacity(32),
        }
    }

    fn start_root(&mut self, name: &str) -> Result<()> {
        tracing::debug!("根对象开始");
        self.writer.write_event(JsonEvent::StartObject)?;
        self.writer.write_event(JsonEvent::ObjectKey("resourceType"))?;
        self.writer.write_event(JsonEvent::String(name))?;
        Ok(())
    }

    fn start_object(&mut self) -> Result<()> {
        tracing::debug!("对象开始");
        self.writer.write_event(JsonEvent::StartObject)?;
        Ok(())
    }

    fn open_element(&mut self, name: &str) -> Result<()> {
        self.tags.push(name.into());
        self.debug();
        Ok(())
    }

    fn end_object(&mut self) -> Result<()> {
        let event = JsonEvent::EndObject;
        tracing::debug!("对象结束");

        self.writer.write_event(event)?;
        Ok(())
    }

    fn build_element(&mut self) -> Result<()> {
        if let Some(tag) = self.tags.pop() {
            let event = JsonEvent::ObjectKey(tag.as_str());
            tracing::debug!("写入Element Key: {:?}", &event);
            self.debug();
            self.writer.write_event(event)?;
        };
        Ok(())
    }

    fn start_array(&mut self) -> Result<()> {
        let event = JsonEvent::StartArray;
        tracing::debug!("数组开始: {:?}", &event);

        self.writer.write_event(event)?;
        Ok(())
    }

    fn end_array(&mut self) -> Result<()> {
        let event = JsonEvent::EndArray;
        tracing::debug!("数组结束: {:?}", &event);

        self.writer.write_event(event)?;
        Ok(())
    }

    fn debug(&mut self) {
        tracing::warn!("===Debug===");
        tracing::warn!("tags: {:?}", self.tags);
    }
}

impl<'ser, W: Write> Serializer for &'ser mut JsonSerializer<W> {
    type SerializeResource = JsonCompositeProcessor<'ser, W>;
    type SerializeStruct = JsonCompositeProcessor<'ser, W>;
    type SerializeVec = JsonCompositeProcessor<'ser, W>;
    type SerializePrimitive = JsonPrimitiveProcessor<'ser, W>;
    type SerializeExtension = JsonCompositeProcessor<'ser, W>;

    fn serialize_any<T: Serialize>(self, name: &str, value: &T) -> Result<()> {
        self.open_element(name)?;
        // self.build_element()?;
        value.serialize(self)
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.writer.write_event(JsonEvent::String(value))?;
        Ok(())
    }

    fn serialize_string(self, value: String) -> Result<()> {
        self.serialize_str(value.as_str())
    }

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.writer.write_event(JsonEvent::Boolean(value))?;
        Ok(())
    }

    fn serialize_number(self, value: usize) -> Result<()> {
        self.writer.write_event(JsonEvent::Number(value.to_string().as_str()))?;
        Ok(())
    }

    fn serialize_integer(self, value: isize) -> Result<()> {
        self.writer.write_event(JsonEvent::Number(value.to_string().as_str()))?;
        Ok(())
    }

    fn serialize_integer64(self, value: i64) -> Result<()> {
        self.writer.write_event(JsonEvent::Number(value.to_string().as_str()))?;
        Ok(())
    }

    fn serialize_decimal(self, value: f64) -> Result<()> {
        self.writer.write_event(JsonEvent::Number(value.to_string().as_str()))?;
        Ok(())
    }

    /// 如果解析到一个None值，意味着key已经赋值，这时必须要将key移除
    fn serialize_none(self) -> Result<()> {
        self.tags.pop();
        self.debug();
        Ok(())
    }

    fn serialize_primitive(self) -> Result<Self::SerializePrimitive> {
        let tag = match self.tags.last() {
            None => {None}
            Some(t) => {
                let mut tag = t.clone();
                tag.insert(0, '_');
                Some(tag)
            }
        };

        Ok(JsonPrimitiveProcessor {
            ser: self,
            addition: false,
            tag
        })
    }

    fn serialize_vec(self, _len: Option<usize>) -> Result<Self::SerializeVec> {
        self.build_element()?;
        self.start_array()?;
        Ok(JsonCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_struct(self, name: &'static str) -> Result<Self::SerializeStruct> {
        self.build_element()?;
        self.start_object()?;

        Ok(JsonCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_extension(self) -> Result<Self::SerializeExtension> {
        self.start_object()?;
        Ok(JsonCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_resource(self, name: &'static str) -> Result<Self::SerializeResource> {
        self.start_root(name)?;
        Ok(JsonCompositeProcessor {
            ser: self,
        })
    }
}

pub struct JsonCompositeProcessor<'ser, W: Write> {
    ser: &'ser mut JsonSerializer<W>,
}

impl<'ser, W: Write> SerializeResource for JsonCompositeProcessor<'ser, W> {
    fn serialize_id(&mut self, value: &Option<Id>) -> Result<()> {
        if let Some(v) = value {
            self.ser.open_element("id")?;
            self.ser.build_element()?;
            v.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_field<T: Serialize>(&mut self, name: &'static str, value: &T) -> Result<()> {
        self.ser.open_element(name)?;
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_object()
    }
}

impl<'ser, W: Write> SerializeStruct for JsonCompositeProcessor<'ser, W> {

    //TODO 这里的处理逻辑是不是和serialize_field是一致的，看那看能不能简化下
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(v) = value {
            self.ser.open_element("id")?;
            self.ser.build_element()?;
            v.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    //TODO 这里的处理逻辑是不是和serialize_field是一致的，看那看能不能简化下
    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()> {
        if let Some(vec) = value {
            self.ser.open_element("extension")?;
            self.ser.build_element()?;
            vec.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_field<T: Serialize>(&mut self, name: &'static str, value: &T) -> Result<()> {
        self.ser.open_element(name)?;
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_object()
    }
}

pub struct JsonPrimitiveProcessor<'ser, W: Write> {
    ser: &'ser mut JsonSerializer<W>,
    addition: bool,
    tag: Option<String>,
}

impl<'ser, W: Write> SerializePrimitive for JsonPrimitiveProcessor<'ser, W> {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(v) = value {
            if !self.addition {
                self.addition = true;
                if let Some(tag) = self.tag.take() {
                    self.ser.open_element(tag.as_str())?;
                    self.ser.build_element()?;
                    self.ser.start_object()?;
                }
            }

            self.ser.open_element("id")?;
            self.ser.build_element()?;
            v.serialize(&mut *self.ser)?;
        };

        Ok(())
    }

    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()> {
        if let Some(ext) = value {
            if !self.addition {
                self.addition = true;
                if let Some(tag) = self.tag.take() {
                    self.ser.open_element(tag.as_str())?;
                    self.ser.build_element()?;
                    self.ser.start_object()?;
                }
            }

            self.ser.open_element("extension")?;
            self.ser.build_element()?;
            ext.serialize(&mut *self.ser)?;
        }

        Ok(())
    }

    fn serialize_value<T: Serialize>(&mut self, value: &Option<T>) -> Result<()> {
        match value {
            None => {
                self.ser.tags.pop();
            }
            Some(val) => {
                self.ser.build_element()?;
                val.serialize(&mut *self.ser)?;
            }
        }
        Ok(())
    }

    fn serialize_end(self) -> Result<()> {
        if self.addition {
            self.ser.end_object()?;
        } else {
            self.ser.tags.pop();
        }
        Ok(())
    }
}

impl<'ser, W: Write> SerializeVec for JsonCompositeProcessor<'ser, W> {
    fn serialize_element<T: Serialize>(&mut self, value: &T) -> Result<()> {
        tracing::debug!("处理数组的单个元素");
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_array()?;
        Ok(())
    }
}

impl<'ser, W: Write> SerializeExtension for JsonCompositeProcessor<'ser, W> {
    fn serialize_id(&mut self, _value: &Option<String>) -> Result<()> {
       Ok(())
    }

    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()> {
        if let Some(vec) = value {
            self.ser.open_element("extension")?;
            self.ser.build_element()?;
            vec.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_url(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(url) = value {
            self.ser.open_element("url")?;
            self.ser.build_element()?;
            url.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_object()
    }
}
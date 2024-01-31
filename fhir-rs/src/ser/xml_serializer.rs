use std::collections::HashMap;
use std::io::Write;
use xml::common::XmlVersion;
use xml::{EmitterConfig, EventWriter};
use xml::writer::{
    XmlEvent,
    events::StartElementBuilder
};
use crate::prelude::*;

pub fn to_string<Ser: Serialize>(value: &Ser) -> Result<String> {
    let mut buffer = Vec::with_capacity(128);
    // to_writer(&mut buffer, value)?;

    let mut ser = XmlSerializer::from_writer(&mut buffer, false);
    value.serialize(&mut ser)?;

    let string = String::from_utf8(buffer)?;
    Ok(string)
}

pub fn to_string_pretty<Ser: Serialize>(value: &Ser) -> Result<String> {
    let mut buffer = Vec::with_capacity(128);
    let mut ser = XmlSerializer::from_writer(&mut buffer, true);
    value.serialize(&mut ser)?;

    let string = String::from_utf8(buffer)?;
    Ok(string)
}

pub struct XmlSerializer<W: Write> {
    writer: EventWriter<W>,
    tags: Vec<String>,
    current_attr_key: Option<&'static str>,
    current_tag_attrs: Option<HashMap<&'static str, String>>,
}

impl<W: Write> XmlSerializer<W> {
    pub fn from_writer(writer: W, pretty: bool) -> Self {
        let writer = EmitterConfig::new()
            .perform_indent(pretty)
            .create_writer(writer);

        XmlSerializer{
            writer,
            tags: Vec::with_capacity(32),
            current_attr_key: None,
            current_tag_attrs: None,
        }
    }

    fn start_document(&mut self) -> Result<()> {
        self.writer.write(XmlEvent::StartDocument {
            version: XmlVersion::Version10,
            encoding: Some("UTF-8"),
            standalone: None,
        })?;
        Ok(())
    }

    fn start_root(&mut self, name: &str) -> Result<()> {
        let event = XmlEvent::start_element(name)
            .default_ns("http://hl7.org/fhir");

        self.writer.write::<StartElementBuilder>(event.into())?;
        Ok(())
    }

    fn open_element(&mut self, name: &str) -> Result<()> {
        self.tags.push(name.into());
        self.current_tag_attrs = Some(HashMap::with_capacity(4));
        Ok(())
    }

    fn reopen_element(&mut self) -> Result<()> {
        self.current_tag_attrs = Some(HashMap::with_capacity(4));
        Ok(())
    }

    fn set_current_attr_key(&mut self, name: &'static str) -> Result<()> {
        self.current_attr_key = Some(name.into());
        Ok(())
    }

    fn set_current_attr_value(&mut self, value:String) -> Result<()> {
        if let Some(key) = self.current_attr_key.take() {
            self.current_tag_attrs
                .as_mut()
                .unwrap()
                .insert(key, value);
        };
        Ok(())
    }

    fn build_start_element(&mut self) -> Result<()> {
        if let Some(tag) = self.tags.pop() {
            match self.current_tag_attrs.take() {
                None => {
                    let event = XmlEvent::start_element(tag.as_str());
                    let event: XmlEvent = event.into();
                    tracing::debug!("{:?}", &event);
                    self.writer.write(event)?;
                }
                Some(attrs) => {
                    let event = attrs.iter()
                        .fold(XmlEvent::start_element(tag.as_str()), |b, (&key, value)| {
                            b.attr(key, value)
                        });
                    let event: XmlEvent = event.into();
                    tracing::debug!("{:?}", &event);
                    self.writer.write(event)?;
                }
            };
        };
        Ok(())
    }

    fn end_element(&mut self) -> Result<()> {
        let event = XmlEvent::end_element();
        let event : XmlEvent = event.into();
        tracing::debug!("{:?}", &event);

        self.writer.write(event)?;
        Ok(())
    }

    fn debug(&mut self) {
        tracing::debug!("tags: {:?} attr_key: {:?}", self.tags, self.current_attr_key);
    }
}

impl<'ser, W: Write> Serializer for &'ser mut XmlSerializer<W> {
    type SerializeResource = XmlCompositeProcessor<'ser, W>;
    type SerializeStruct = XmlCompositeProcessor<'ser, W>;
    type SerializeVec = XmlCompositeProcessor<'ser, W>;
    type SerializePrimitive = XmlCompositeProcessor<'ser, W>;
    type SerializeExtension = XmlCompositeProcessor<'ser, W>;

    fn serialize_any<T: Serialize>(self, name: &str, value: &T) -> Result<()> {
        self.open_element(name)?;
        value.serialize(self)
    }

    fn serialize_str(self, value: &str)  -> Result<()> {
        self.set_current_attr_value(value.into())?;
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, value );
        Ok(())
    }

    fn serialize_string(self, value: String)  -> Result<()> {
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, &value );
        self.set_current_attr_value(value)?;
        Ok(())
    }

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.set_current_attr_value(value.to_string())?;
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, value );
        Ok(())
    }

    fn serialize_number(self, value: usize)  -> Result<()> {
        self.set_current_attr_value(value.to_string())?;
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, value );
        Ok(())
    }

    fn serialize_integer(self, value: isize)  -> Result<()> {
        self.set_current_attr_value(value.to_string())?;
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, value );
        Ok(())
    }

    fn serialize_integer64(self, value: i64) -> Result<()> {
        self.set_current_attr_value(value.to_string())?;
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, value );
        Ok(())
    }

    fn serialize_decimal(self, value: f64)  -> Result<()> {
        self.set_current_attr_value(value.to_string())?;
        tracing::debug!("tags: {:?} attr_key: {:?} value: {:?}", self.tags, self.current_attr_key, value );
        Ok(())
    }

    /// 如果解析到一个None值，意味着key已经赋值，这时必须要将key移除
    /// 这时出现两种情况:
    /// 1. 元素为空，这时要将最后push的值pop即可；
    /// 2. 属性为空（也就是primitive的value为空时），这时要将attr_key置为空
    ///
    /// 这里的策略是：
    /// * 判定attr_key是否为空
    ///     * 如果不为空，就置空
    ///     * 如果为空，就pop出tags的最后一个值
    fn serialize_none(self) -> Result<()> {
        self.debug();
        match self.current_attr_key {
            None => {self.tags.pop();}
            Some(_) => {self.current_attr_key.take();}
        }
        self.debug();
        Ok(())
    }

    fn serialize_primitive(self) -> Result<Self::SerializePrimitive> {

        Ok(XmlCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_vec(self, len: Option<usize>) -> Result<Self::SerializeVec> {
        let name = self.tags.pop().unwrap();
        if let Some(size) = len {
            for _ in 0..size {
                self.tags.push(name.clone())
            }
        }
        self.debug();
        Ok(XmlCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_struct(self, _name: &'static str) -> Result<Self::SerializeStruct> {
        Ok(XmlCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_extension(self) -> Result<Self::SerializeExtension> {
        Ok(XmlCompositeProcessor {
            ser: self,
        })
    }

    fn serialize_resource(self, name: &'static str) -> Result<Self::SerializeResource> {
        self.start_document()?;
        self.start_root(name)?;

        Ok(XmlCompositeProcessor {
            ser: self,
        })
    }
}

pub struct XmlCompositeProcessor<'ser, W: Write> {
    ser: &'ser mut XmlSerializer<W>,
}

impl<'ser, W: Write> SerializeResource for XmlCompositeProcessor<'ser, W> {
    fn serialize_id(&mut self, value: &Option<Id>) -> Result<()> {
        if let Some(id) = value {
            let iddt = IdDt::new(id);
            SerializeResource::serialize_field(self, "id", &iddt)?;
        }
        Ok(())
    }

    fn serialize_field<T: Serialize>(&mut self, name: &'static str, value: &T) -> Result<()> {
        self.ser.open_element(name)?;
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_element()
    }
}

impl<'ser, W: Write> SerializeStruct for XmlCompositeProcessor<'ser, W> {

    fn serialize_id(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(v) = value {
            self.ser.set_current_attr_key("id")?;
            v.serialize(&mut *self.ser)?;
        };
        self.ser.build_start_element()?;
        Ok(())
}

    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()> {
        self.ser.build_start_element()?;

        if let Some(ext) = value {
            self.ser.open_element("extension")?;
            ext.serialize(&mut *self.ser)?;
        }
        Ok(())
    }

    fn serialize_field<T: Serialize>(&mut self, name: &'static str, value: &T) -> Result<()> {
        self.ser.open_element(name)?;
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_element()
    }
}

impl<'ser, W: Write> SerializePrimitive for XmlCompositeProcessor<'ser, W> {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(v) = value {
            self.ser.set_current_attr_key("id")?;
            v.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()> {
        self.ser.build_start_element()?;

        if let Some(ext) = value {
            self.ser.open_element("extension")?;
            ext.serialize(&mut *self.ser)?;
        }
        Ok(())
    }

    fn serialize_value<T: Serialize>(&mut self, value: &Option<T>) -> Result<()> {
        if let Some(val) = value {
            self.ser.set_current_attr_key("value")?;
            val.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_element()
    }
}

impl<'ser, W: Write> SerializeVec for XmlCompositeProcessor<'ser, W> {
    fn serialize_element<T: Serialize>(&mut self, value: &T) -> Result<()> {
        tracing::debug!("处理数组的单个元素");
        self.ser.reopen_element()?;
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        tracing::debug!("数组处理结束");
        Ok(())
    }
}

impl<'ser, W: Write> SerializeExtension for XmlCompositeProcessor<'ser, W> {
    fn serialize_id(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(v) = value {
            self.ser.set_current_attr_key("id")?;
            v.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_extension(&mut self, value: &Option<Vec<Extension>>) -> Result<()> {
        self.ser.build_start_element()?;

        if let Some(ext) = value {
            self.ser.open_element("extension")?;
            ext.serialize(&mut *self.ser)?;
        }
        Ok(())
    }

    fn serialize_url(&mut self, value: &Option<String>) -> Result<()> {
        if let Some(v) = value {
            self.ser.set_current_attr_key("url")?;
            v.serialize(&mut *self.ser)?;
        };
        Ok(())
    }

    fn serialize_value<T: Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut *self.ser)
    }

    fn serialize_end(self) -> Result<()> {
        self.ser.end_element()
    }
}
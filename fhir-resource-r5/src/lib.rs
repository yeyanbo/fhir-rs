pub mod resource;

use fhir_rs::prelude::*;
use crate::resource::*;

#[derive(Debug)]
pub struct Patient {
    pub name: Option<StringDt>,
    pub age: Option<PositiveIntDt>,
    pub telecom: Option<Vec<StringDt>>,
    pub class: Option<Vec<Coding>>,
}

impl Serialize for Patient {
    fn serialize<Ser>(&self, serializer: Ser) -> Result<()> where Ser: Serializer {
        let mut resource  = serializer.serialize_struct("Patient")?;
        resource.serialize_field("name", &self.name)?;
        resource.serialize_field("age", &self.age)?;
        resource.serialize_field("telecom", &self.telecom)?;
        resource.serialize_field("class", &self.class)?;
        resource.serialize_end()
    }
}

impl<'de> Deserialize<'de> for Patient {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
        tracing::info!("开始反序列化患者信息");
        deserializer.deserialize_struct("Patient", PatientVisitor)
    }
}

struct PatientVisitor;

impl<'de> Visitor<'de> for PatientVisitor {
    type Value = Patient;

    fn visit_map<M>(self, mut map: M) -> Result<Self::Value>
        where
            M: MapAccess<'de>
    {
        tracing::debug!("进入到 Patient 的 visit-map 函数了");

        let mut name: Option<StringDt> = None;
        let mut age: Option<PositiveIntDt> = None;
        let mut age_ext: Option<PositiveIntDt> = None;
        let mut telecom: Option<Vec<StringDt>> = None;
        let mut class: Option<Vec<Coding>> = None;

        while let Some(key) = map.next_key()? {
            match key.as_bytes() {
                b"resourceType" => {
                    let typ: StringDt = map.next_value()?;
                    tracing::debug!("读取到值: {:?}", typ);
                },
                b"name" => {
                    name = Some(map.next_value()?);
                    tracing::debug!("读取到值: {:?}", &name);
                },
                b"age" => {
                    age = Some(map.next_value()?);
                    tracing::debug!("读取到值: {:?}", &age);
                }
                b"_age" => {
                    age_ext = Some(map.next_value()?);
                    tracing::debug!("读取到值: {:?}", &age_ext);
                }
                b"telecom" => {
                    telecom = Some(map.next_value()?);
                    tracing::debug!("读取到值: {:?}", &telecom);
                }
                b"class" => {
                    class = Some(map.next_value()?);
                    tracing::debug!("读取到值: {:?}", &class);
                },
                _ => {return Err(FhirError::error("Patient读到不存在的key了"));},
            }
        }

        let age = self.with_extension(age, age_ext);

        Ok(Patient{
            name,
            age,
            telecom,
            class,
        })
    }
}


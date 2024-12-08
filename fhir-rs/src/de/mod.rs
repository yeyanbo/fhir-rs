mod xml_deserializer;
mod json_deserializer;

use std::marker::PhantomData;
use std::str::FromStr;

pub use xml_deserializer::from_str as from_xml;
pub use json_deserializer::from_str as from_json;

use crate::prelude::*;

/// 资源和数据类型的反序列化特性
/// 所有的资源和数据类型（简单类型和复合类型）都应实现该特性
pub trait Deserialize<'de>: Sized {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de>;
}

pub trait Deserializer<'de>: Sized {

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>;

    fn deserialize_number<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>;

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
        where V: Visitor<'de>;

    fn deserialize_vec<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>;

    fn deserialize_enum<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>;

    fn deserialize_narrative<V>(self, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>;

    fn deserialize_struct<V>(self, name: &str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>;

    fn deserialize_resource<V>(self, name: &str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>;

    fn deserialize_primitive<V>(self, _name: &str, visitor: V) -> Result<V::Value>
        where
            V: Visitor<'de>;
}

pub trait MapAccess<'de> {

    fn next_key(&mut self) -> Result<Option<String>>;

    fn next_value<De>(&mut self) -> Result<De> where De: Deserialize<'de>;

    fn next_any_value<De>(&mut self, _key: &str) -> Result<De> where De: Deserialize<'de> {
        unreachable!()
    }
}

pub trait VecAccess<'de> {

    fn next_element<T>(&mut self) -> Result<Option<T>> where T: Deserialize<'de>;
}

pub trait Visitor<'de>: Sized {
    type Value;
    fn visit_str(self, _v: &str) -> Result<Self::Value>{
        Err(FhirError::un_implementation("visit_str"))
    }

    fn visit_bool(self, _v: bool) -> Result<Self::Value>{
        Err(FhirError::un_implementation("visit_bool"))
    }

    fn visit_map<M>(self, _map: M) -> Result<Self::Value>
        where
            M: MapAccess<'de>
    {
        Err(FhirError::un_implementation("visit_map"))
    }

    fn visit_vec<V>(self, _vec: V) -> Result<Self::Value>
        where
            V: VecAccess<'de>
    {
        Err(FhirError::un_implementation("visit_vec"))
    }

    fn visit_enum<De>(self, _name: &str, _deserializer: De) -> Result<Self::Value>
        where De: Deserializer<'de>
    {
        Err(FhirError::un_implementation("visit_enum"))
    }

    fn visit_key<De>(self, _deserializer: &De) -> Result<&'de str>
        where De: Deserializer<'de>
    {
        Err(FhirError::un_implementation("visit_key"))
    }

    /// 将一个没有扩展信息的值与一个有扩展信息的值进行合并
    ///
    /// ### 参数
    /// - `master`: 没有扩展信息的数据
    /// - `slave`:  有扩展信息的值
    /// ### 返回值
    fn with_extension<T>(self, master: Option<T>, slave: Option<T>) -> Option<T>
        where T: Element
    {
        match(master, slave) {
            (Some(v), None) => {Some(v)}
            (None, Some(v)) => {Some(v)}
            (None, None) => {None}
            (Some(a), Some(b)) => {
                Some(a.set_id(b.id().clone().unwrap()).set_extensions(b.extensions().unwrap().clone()))
            }
        }
    }
}

impl<'de, T> Deserialize<'de> for Box<T>
    where
        T: Deserialize<'de>,
{
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {
        Deserialize::deserialize(deserializer).map(Box::new)
    }
}


impl<'de, T> Deserialize<'de> for Vec<T>
    where
        T: Deserialize<'de>,
{
    fn deserialize<De>(deserializer: De) -> Result<Self>
        where De: Deserializer<'de>
    {
        let visitor = VecVisitor {
            marker: PhantomData,
        };
        deserializer.deserialize_vec( visitor)
    }
}
pub struct VecVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> Visitor<'de> for VecVisitor<T>
    where
        T: Deserialize<'de>,
{
    type Value = Vec<T>;

    fn visit_vec<V>(self, mut vec: V) -> Result<Self::Value> where V: VecAccess<'de> {
        let mut values = Vec::<T>::new();

        loop {
            match vec.next_element() {
                Ok(Some(value)) => {
                    values.push(value);
                },
                Ok(None) => {
                    break
                }
                Err(FhirError::EndArrayWhileParsingList) => {
                    break
                },
                Err(err) => {
                    return Err(err);
                }
            }
        }

        Ok(values)
    }
}


impl<'de> Deserialize<'de> for String {
    fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'de>,
    {
        struct StringVisitor;
        impl<'de> Visitor<'de> for StringVisitor {
            type Value = String;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(v.to_owned())
            }
        }
        
        deserializer.deserialize_str(StringVisitor)
    }
}

impl<'de> Deserialize<'de> for bool {
    fn deserialize<D>(deserializer: D) -> Result<Self> where D: Deserializer<'de> {
        struct BoolVisitor;
        impl<'de> Visitor<'de> for BoolVisitor {
            type Value = bool;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(bool::from_str(v)?)
            }
        }

        deserializer.deserialize_str(BoolVisitor)
    }
}

impl<'de> Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self> where D: Deserializer<'de> {
        struct DateVisitor;
        impl<'de> Visitor<'de> for DateVisitor {
            type Value = Date;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(Date::from_str(v)?)
            }
        }

        deserializer.deserialize_str(DateVisitor)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self> where D: Deserializer<'de> {
        struct TimeVisitor;
        impl<'de> Visitor<'de> for TimeVisitor {
            type Value = Time;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(Time::from_str(v)?)
            }
        }

        deserializer.deserialize_str(TimeVisitor)
    }
}

impl<'de> Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self> where D: Deserializer<'de> {
        struct DateTimeVisitor;
        impl<'de> Visitor<'de> for DateTimeVisitor {
            type Value = DateTime;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(DateTime::from_str(v)?)
            }
        }

        deserializer.deserialize_str(DateTimeVisitor)
    }
}

impl<'de> Deserialize<'de> for Instant {
    fn deserialize<D>(deserializer: D) -> Result<Self> where D: Deserializer<'de> {
        struct InstantVisitor;
        impl<'de> Visitor<'de> for InstantVisitor {
            type Value = Instant;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(Instant::from_str(v)?)
            }
        }

        deserializer.deserialize_str(InstantVisitor)
    }
}

impl<'de> Deserialize<'de> for usize {
    fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'de>,
    {
        struct PositiveIntVisitor;
        impl<'de> Visitor<'de> for PositiveIntVisitor {
            type Value = PositiveInt;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(PositiveInt::from_str(v)?)
            }
        }
        
        deserializer.deserialize_number(PositiveIntVisitor)
    }
}

impl<'de> Deserialize<'de> for isize {
    fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'de>,
    {
        struct IntegerVisitor;
        impl<'de> Visitor<'de> for IntegerVisitor {
            type Value = Integer;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(Integer::from_str(v)?)
            }
        }

        deserializer.deserialize_number(IntegerVisitor)
    }
}

impl<'de> Deserialize<'de> for f64 {
    fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'de>,
    {
        struct DecimalVisitor;
        impl<'de> Visitor<'de> for DecimalVisitor {
            type Value = Decimal;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(Decimal::from_str(v)?)
            }
        }

        deserializer.deserialize_number(DecimalVisitor)
    }
}

impl<'de> Deserialize<'de> for i64 {
    fn deserialize<D>(deserializer: D) -> Result<Self>
        where
            D: Deserializer<'de>,
    {
        struct Integer64Visitor;
        impl<'de> Visitor<'de> for Integer64Visitor {
            type Value = Integer64;

            fn visit_str(self, v: &str) -> Result<Self::Value>
            {
                Ok(Integer64::from_str(v)?)
            }
        }

        deserializer.deserialize_number(Integer64Visitor)
    }
}

impl<'de> Deserialize<'de> for AnyType {
    fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {

        pub struct AnyVisitor;

        impl<'de> Visitor<'de> for AnyVisitor {
            type Value = AnyType;

            fn visit_enum<De>(self, name: &str, _deserializer: De) -> Result<Self::Value>
            where
                De: Deserializer<'de>,
            {
                println!("Any Type2: {}", name);
                Ok(AnyType::String(StringDt::new("a")))
            }
        }

        deserializer.deserialize_enum(AnyVisitor)
    }
}

macro_rules! impl_deserializer_for_primitive {
    (
        $(($inner: ident, $ty: ident, $visitor:ident),)+
    ) => {
        $(
            impl<'de> Deserialize<'de> for $ty {
                fn deserialize<De>(deserializer: De) -> Result<Self> where De: Deserializer<'de> {

                    pub struct $visitor;
                    impl<'de> Visitor<'de> for $visitor {
                        type Value = $ty;

                        fn visit_str(self, v: &str) -> Result<Self::Value> {
                            $ty::from_str(v)
                        }

                        fn visit_map<M>(self, mut mapp: M) -> Result<Self::Value> where M: MapAccess<'de> {
                            let mut id: Option<String> = None;
                            let mut extension: Option<Vec<Extension>> = None;
                            let mut value: Option<$inner> = None;

                            while let Some(key) = mapp.next_key()? {
                                match key.as_str() {
                                    "id" => id = Some(mapp.next_value()?),
                                    "extension" => extension = Some(mapp.next_value()?),
                                    "value" => value = Some(mapp.next_value()?),
                                    _ => { /* skip */ },
                                }
                            }

                            Ok( $ty { id, extension, value } )
                        }
                    }

                    deserializer.deserialize_primitive("", $visitor)
                }
            }
        )+
    };
}

impl_deserializer_for_primitive!{
    (String, StringDt, StringDtVisitor),
    (Id, IdDt, IdDtVisitor),
    (Base64Binary, Base64BinaryDt, Base64BinaryDtVisitor),
    (Markdown, MarkdownDt, MarkdownDtVisitor),
    (Uri, UriDt, UriDtVisitor),
    (Url, UrlDt, UrlDtVisitor),
    (Oid, OidDt, OidDtVisitor),
    (Uuid, UuidDt, UuidDtVisitor),
    (Canonical, CanonicalDt, CanonicalDtVisitor),
    (Code, CodeDt, CodeDtVisitor),
    (Boolean, BooleanDt, BooleanDtVisitor),
    (DateTime, DateTimeDt, DateTimeDtVisitor),
    (Date, DateDt, DateDtVisitor),
    (Time, TimeDt, TimeDtVisitor),
    (Instant, InstantDt, InstantDtVisitor),
    (UnsignedInt, UnsignedIntDt, UnsignedIntDtVisitor),
    (PositiveInt, PositiveIntDt, PositiveIntDtVisitor),
    (Integer, IntegerDt, IntegerDtVisitor),
    (Integer64, Integer64Dt, Integer64DtVisitor),
    (Decimal, DecimalDt, DecimalDtVisitor),
}
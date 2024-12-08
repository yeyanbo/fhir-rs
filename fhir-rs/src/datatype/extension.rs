use crate::impl_element;
use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub struct Extension {
    pub id: Option<Id>,
    pub extension: Option<Vec<Extension>>,
    pub url: Option<Uri>,
    pub value: Option<AnyType>,
}

impl_element!(Extension);
impl Extension {
    pub fn new<U: Into<Url>>(url: U, value: AnyType) -> Extension {
        Extension {
            id: None,
            extension: None,
            url: Some(url.into()),
            value: Some(value),
        }
    }

    pub fn with_url<U: Into<Url>>(url: U) -> Extension {
        Extension {
            id: None,
            extension: None,
            url: Some(url.into()),
            value: None,
        }
    }
}

impl Executor for Extension {
    fn to_collection(&self, _index: &Option<usize>) -> Collection {
        Collection::new_any(Box::new(self.clone()))
    }
}

impl Convert for Extension {}
impl Compare for Extension {}

impl Serialize for Extension {
    fn serialize<Ser>(&self, serializer: Ser) -> crate::prelude::Result<()>
    where Ser: Serializer {
        let mut extension  = serializer.serialize_extension()?;
        extension.serialize_id(&self.id)?;
        extension.serialize_url(&self.url)?;
        extension.serialize_extension(&self.extension)?;
        extension.serialize_value(&self.value)?;
        extension.serialize_end()
    }
}

macro_rules! impl_deserialize_for_extension {
    (
        $(($anytype: ident, $dt: ident, $matched: literal),)+
    ) => {
        impl<'de> Deserialize<'de> for Extension {
    fn deserialize<De>(deserializer: De) -> crate::prelude::Result<Self>
    where De: Deserializer<'de> {
        pub struct ExtensionVisitor;
        impl<'de> Visitor<'de> for ExtensionVisitor {
            type Value = Extension;

            fn visit_map<V>(self, mut map: V) -> crate::prelude::Result<Extension>
            where
                V: MapAccess<'de>,
            {
                let mut id: Option<String> = None;
                let mut url : Option<String> = None;
                let mut extension: Option<Vec<Extension>> = None;
                let mut value: Option<AnyType> = None;

                while let Some(key) = map.next_key()? {
                    match key.as_str() {
                        "id" => {
                            id = Some(map.next_value()?);
                        },
                        "url" => {
                            url = Some(map.next_value()?);
                        }
                        "extension" => extension = Some(map.next_value()?),
                        $(
                            $matched => {
                                let next: $dt = map.next_value()?;
                                value = Some(AnyType::$anytype(next));
                            }
                        )+
                        other => {return Err(FhirError::Message(format!("Extension读到不存在的[{other}]了")));},
                    }
                }

                Ok(Extension { id, url, extension, value })
            }
        }

        deserializer.deserialize_struct("Extension", ExtensionVisitor)
    }
}
    };
}

impl_deserialize_for_extension!{
    (String, StringDt, "valueString"),
    (Id, IdDt, "valueId"),
    (Base64Binary, Base64BinaryDt, "valueBase64Binary"),
    (Markdown, MarkdownDt, "valueMarkdown"),
    (Uri, UriDt, "valueUri"),
    (Url, UrlDt, "valueUrl"),
    (Oid, OidDt, "valueOid"),
    (Uuid, UuidDt, "valueUuid"),
    (Canonical, CanonicalDt, "valueCanonical"),
    (Code, CodeDt, "valueCode"),
    (Boolean, BooleanDt, "valueBoolean"),
    (DateTime, DateTimeDt, "valueDateTime"),
    (Date, DateDt, "valueDate"),
    (Time, TimeDt, "valueTime"),
    (Instant, InstantDt, "valueInstant"),
    (UnsignedInt, UnsignedIntDt, "valueUnsignedInt"),
    (PositiveInt, PositiveIntDt, "valuePositiveInt"),
    (Integer, IntegerDt, "valueInteger"),
    (Integer64, Integer64Dt, "valueInteger64"),
    (Decimal, DecimalDt, "valueDecimal"),
    (Address, Address, "valueAddress"),
    (Age, Age, "valueAge"),
    (Annotation, Annotation, "valueAnnotation"),
    (Attachment, Attachment, "valueAttachment"),
    (CodeableConcept, CodeableConcept, "valueCodeableConcept"),
    (CodeableReference, CodeableReference, "valueCodeableReference"),
    (Coding, Coding, "valueCoding"),
    (ContactPoint, ContactPoint, "valueContactPoint"),
    (Count, Count, "valueCount"),
    (Distance, Distance, "valueDistance"),
    (Duration, Duration, "valueDuration"),
    (HumanName, HumanName, "valueHumanName"),
    (Identifier, Identifier, "valueIdentifier"),
    (Money, Money, "valueMoney"),
    (Period, Period, "valuePeriod"),
    (Quantity, Quantity, "valueQuantity"),
    (Range, Range, "valueRange"),
    (Ratio, Ratio, "valueRatio"),
    (RatioRange, RatioRange, "valueRatioRange"),
    (Reference, Reference, "valueReference"),
    (SampledData, SampledData, "valueSampledData"),
    (Signature, Signature, "valueSignature"),
    (Timing, Timing, "valueTiming"),
    (ContactDetail, ContactDetail, "valueContactDetail"),
    (DataRequirement, DataRequirement, "valueDataRequirement"),
    (Expression, Expression, "valueExpression"),
    (ParameterDefinition, ParameterDefinition, "valueParameterDefinition"),
    (RelatedArtifact, RelatedArtifact, "valueRelatedArtifact"),
    (TriggerDefinition, TriggerDefinition, "valueTriggerDefinition"),
    (UsageContext, UsageContext, "valueUsageContext"),
    (Availability, Availability, "valueAvailability"),
    (ExtendedContactDetail, ExtendedContactDetail, "valueExtendedContactDetail"),
    (Dosage, Dosage, "valueDosage"),
    (Meta, Meta, "valueMeta"),
}
use crate::prelude::*;

macro_rules! anytype {
    (
        $(($id: ident, $ty: ident),)+
    ) => {
        #[derive(Clone, Debug)]
        pub enum AnyType {
            $($id($ty),)+
        }

        impl Base for AnyType {
            fn type_name(&self) -> &str {
                match self {
                    $(AnyType::$id(vlaue) => vlaue.type_name(),)+
                }
            }
        }
    };
}

anytype!{
    (String, StringDt),
    (Id, IdDt),
    (Base64Binary, Base64BinaryDt),
    (Markdown, MarkdownDt),
    (Uri, UriDt),
    (Url, UrlDt),
    (Oid, OidDt),
    (Uuid, UuidDt),
    (Canonical, CanonicalDt),
    (Code, CodeDt),
    (Boolean, BooleanDt),
    (DateTime, DateTimeDt),
    (Date, DateDt),
    (Time, TimeDt),
    (Instant, InstantDt),
    (UnsignedInt, UnsignedIntDt),
    (PositiveInt, PositiveIntDt),
    (Integer, IntegerDt),
    (Integer64, Integer64Dt),
    (Decimal, DecimalDt),
    (Address, Address),
    (Age, Age),
    (Annotation, Annotation),
    (Attachment, Attachment),
    (CodeableConcept, CodeableConcept),
    (CodeableReference, CodeableReference),
    (Coding, Coding),
    (ContactPoint, ContactPoint),
    (Count, Count),
    (Distance, Distance),
    (Duration, Duration),
    (HumanName, HumanName),
    (Identifier, Identifier),
    (Money, Money),
    (Period, Period),
    (Quantity, Quantity),
    (Range, Range),
    (Ratio, Ratio),
    (RatioRange, RatioRange),
    (Reference, Reference),
    (SampledData, SampledData),
    (Signature, Signature),
    (Timing, Timing),
    (ContactDetail, ContactDetail),
    (DataRequirement, DataRequirement),
    (Expression, Expression),
    (ParameterDefinition, ParameterDefinition),
    (RelatedArtifact, RelatedArtifact),
    (TriggerDefinition, TriggerDefinition),
    (UsageContext, UsageContext),
    (Availability, Availability),
    (ExtendedContactDetail, ExtendedContactDetail),
    (Dosage, Dosage),
    (Meta, Meta),
}

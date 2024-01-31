use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use fhir_resource_r5::Patient;
use fhir_rs::prelude::*;
use core::str::FromStr;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let complex_extension = Extension::with_url("http://yeyanbo.cn")
        .add_extension(Extension::new("abc", Any::Coding(Coding::default().set_code("cc"))))
        .add_extension(Extension::new("xcv", Any::Coding(Coding::default().set_code("bb"))));

    let patient = Patient::default()
        .set_id("example")
        .set_meta(Meta::default()
            .set_version_id("v1")
            .set_last_updated(InstantDt::from_str("2001-10-10T10:10:12Z")?))
        .add_extension(Extension::new("dd", Any::String(StringDt::new("ddf"))))
        .add_extension(complex_extension)
        .set_active(true)
        .add_identifier(Identifier::default()
            .set_use_("usual")
            .set_system("urn:oid:1.2.36.146.595.217.0.1")
            .set_value("12345"))       
        .add_name(HumanName::default()
            .set_use_("maiden")
            .set_family("Windsor")
            .add_given("Peter"))
        .add_telecom(ContactPoint::default()
            .set_value("1234567890")
            .set_use_("work")
            .set_rank(1))
        .set_gender("male")
        .set_birth_date(DateDt::from_str("1974-12-25")?
            .add_extension(Extension::new(
                "http://hl7.org/fhir/StructureDefinition/patient-birthTime", 
                Any::DateTime(DateTimeDt::from_str("1974-12-25T14:35:45-05:00")?))));

    test_xml_serialize(&patient)?;
    test_json_serialize(&patient)?;
    Ok(())
}

fn test_json_serialize(patient: &Patient) -> Result<()> {
    let str = to_json_pretty(patient)?;
    info!("Patient Formatter: {}", str);
    Ok(())
}

fn test_xml_serialize(patient: &Patient) -> Result<()> {
    let str = to_xml_pretty(patient)?;
    info!("Patient Formatter: {}", str);
    Ok(())
}
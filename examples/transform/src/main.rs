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
        .set_id("12345")
        .set_meta(Meta::default()
            .set_version_id("v1")
            .set_last_updated(InstantDt::from_str("2001-10-10T10:10:12Z")?))
        .add_extension(Extension::new("dd", Any::String(StringDt::new("ddf"))))
        .add_name(HumanName::default().set_text("ZhangSan"))
        .add_telecom(ContactPoint::default().set_value("1234567890").add_extension(complex_extension));

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
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use fhir_rs::prelude::*;
use core::str::FromStr;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let complex_extension = Extension::with_url("http://yeyanbo.cn")
        .add_extension(Extension::new("abc", AnyType::Coding(Coding::default().set_code("cc"))))
        .add_extension(Extension::new("xcv", AnyType::Coding(Coding::default().set_code("bb"))));

    let patient = Patient::default()
        .set_id("example")
        .set_meta(Meta::default()
            .set_version_id("v1")
            .set_last_updated(InstantDt::from_str("2001-10-10T10:10:12Z")?))
        .add_extension(Extension::new("dd", AnyType::String(StringDt::new("ddf"))))
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
                AnyType::DateTime(DateTimeDt::from_str("1974-12-25T14:35:45-05:00")?))));

    // test_xml_serialize(&patient)?;
    // test_json_serialize(&patient)?;

    // test_xml_deserialize()?;
    test_xml_file_deserialize()?;
    // test_json_file_deserialize()?;
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

fn test_xml_file_deserialize() -> Result<()> {
  let script_str = include_str!("../testscript.xml");
  let ret: Result<TestScript> = from_xml(script_str);
    match ret {
        Ok(script) => {
            tracing::info!("Script Name: {:#?}", script);
        }
        Err(err) => {
            tracing::error!("{:?}", err);
        }
    }

  Ok(())
}

fn test_json_file_deserialize() -> Result<()> {
    let script_str = include_str!("../patient.json");
    let ret: Result<Patient> = from_json(script_str);
    match ret {
        Ok(patient) => {
            tracing::info!("Script Name: {:#?}", patient);

            let str = to_xml_pretty(&patient)?;
            info!("Patient Formatter: {}", str);
        }
        Err(err) => {
            tracing::error!("{:?}", err);
        }
    }

    Ok(())
}

fn test_xml_deserialize() -> Result<()> {
    let patient_str = r##"<?xml version="1.0" encoding="UTF-8"?>
    <Patient xmlns="http://hl7.org/fhir">
      <meta>
        <versionId value="v1" />
        <lastUpdated value="2001-10-10T18:10:12+08:00" />
      </meta>
      <extension url="dd">
        <valueString value="ddf" />
      </extension>
      <extension url="http://yeyanbo.cn">
        <extension url="abc">
          <valueCoding>
            <code value="cc" />
          </valueCoding>
        </extension>
        <extension url="xcv">
          <valueCoding>
            <code value="bb" />
          </valueCoding>
        </extension>
      </extension>
      <identifier>
        <use value="usual" />
        <system value="urn:oid:1.2.36.146.595.217.0.1" />
        <value value="12345" />
      </identifier>
      <active value="true" />
      <name>
        <use value="maiden" />
        <family value="Windsor" />
        <given value="Peter" />
      </name>
      <telecom>
        <value value="1234567890" />
        <use value="work" />
        <rank value="1" />
      </telecom>
      <gender value="male" />
      <birthDate value="1974-12-25">
        <extension url="http://hl7.org/fhir/StructureDefinition/patient-birthTime">
          <valueDateTime value="1974-12-25T14:35:45-05:00" />
        </extension>
      </birthDate>
    </Patient>"##;

    let ret: Result<Patient> = from_xml(patient_str);
    match ret {
        Ok(patient) => {
            tracing::info!("Patient Name: {:?}", patient);
        }
        Err(err) => {
            tracing::error!("{:?}", err);
        }
    }
    Ok(())
}



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

    let patient = Patient::default()
        .set_id("12345".to_string())
        .set_meta(Meta::default()
            .set_version_id(IdDt::new("v1"))
            .set_last_updated(InstantDt::from_str("2001-10-10T10:10:12Z")?))
        .add_extension(Extension::new("dd".to_string(), Any::String(StringDt::new("ddf"))));

    test_xml_serialize(&patient)?;
    test_json_serialize(&patient)?;
    Ok(())
}

// fn test_json_deserialize() {
//     let patient_str = r##"
//     {
//         "resourceType": "Patient",
//         "name": "zhang",
//         "age": 13,
//         "_age": {
//             "id": "123456",
//             "extension":[
//                 {
//                 "url":"birth_date_time",
//                 "valueString":"2008-12-19"
//                 },
//                 {
//                 "url":"number",
//                 "valuePositiveInt": 2023
//                 }
//             ]
//         },
//         "telecom":[
//             "010-2345678",
//             "022-23456567545"
//         ],
//         "class":[
//             {"code":"abc","system":"gender","display":"student"},
//             {"code":"kiss","system":"gender","display":"kiss"}
//         ]
//     }
//     "##;
//
//     let ret: Result<Patient> = from_json(patient_str);
//     match ret {
//         Ok(patient) => {
//             tracing::info!("Patient Name: {:?}", patient);
//         }
//         Err(err) => {
//             tracing::error!("{:?}", err);
//         }
//     }
// }

// fn test_xml_deserialize() {
//     let patient_str = r##"<?xml version="1.0" encoding="UTF-8"?>
//     <Patient xmlns="http://hl7.org/fhir">
//         <name value="zhangsan"/>
//         <age value="32">
//             <extension url="count">
//                 <valuePositiveInt value="200"/>
//             </extension>
//             <extension url="hello">
//                 <extension url="first">
//                     <valuePositiveInt value="200" />
//                 </extension>
//                 <extension url="second">
//                     <valueString value="hello2" />
//                 </extension>
//                 <valueString value="hello2" />
//             </extension>
//             <extension url="world">
//                 <valueString value="world"/>
//             </extension>
//         </age>
//         <telecom value="010-12345678"/>
//         <telecom value="022-98765432"/>
//         <class>
//             <code value="123"/>
//             <system value="http://fhir.org"/>
//             <display value="Feed"/>
//         </class>
//         <class>
//             <code value="453"/>
//             <system value="http://fhir.org"/>
//             <display value="Food"/>
//         </class>
//     </Patient>
//     "##;
//
//     let ret: Result<Patient> = from_xml(patient_str);
//     match ret {
//         Ok(patient) => {
//             tracing::info!("Patient Name: {:?}", patient);
//         }
//         Err(err) => {
//             tracing::error!("{:?}", err);
//         }
//     }
// }

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

// fn test_date() -> Result<()> {
//     let d1 = DateDt::from_str("2009")?;
//     tracing::debug!("Date: {}", d1.to_string());
//     let d1 = DateDt::from_str("2009-12")?;
//     tracing::debug!("Date: {}", d1);
//     let d1 = DateDt::from_str("2009-12-23")?;
//     tracing::debug!("Date: {}", d1);
//
//     let t1 = TimeDt::from_str("23:12:45")?;
//     tracing::debug!("Time: {:?}", t1);
//     let t1 = TimeDt::from_str("23:12:45.234")?;
//     tracing::debug!("Time: {}", t1);
//     let t1 = TimeDt::from_str("23:12:60.040")?;
//     tracing::debug!("Time: {}", t1);
//
//     tracing::debug!("=======================");
//
//     let dt1 = DateTimeDt::from_str("2009")?;
//     tracing::debug!("DateTime: {}", dt1.to_string());
//     let dt1 = DateTimeDt::from_str("2009-12")?;
//     tracing::debug!("DateTime: {:?}", dt1);
//     let dt1 = DateTimeDt::from_str("2009-12-23")?;
//     tracing::debug!("DateTime: {}", dt1);
//     let dt1 = DateTimeDt::from_str("2009-12-23T23:12:45Z")?;
//     tracing::debug!("DateTime: {:?}", dt1);
//     let dt1 = DateTimeDt::from_str("2009-12-23T23:12:45.234Z")?;
//     tracing::debug!("DateTime: {}", dt1);
//     let dt1 = DateTimeDt::from_str("2009-12-23T23:12:45-06:00")?;
//     tracing::debug!("DateTime: {}", dt1);
//
//     tracing::debug!("=======================");
//
//     let instant1 = InstantDt::from_str("2009-12-23T23:12:45Z")?;
//     tracing::debug!("Instant: {}", instant1);
//     let instant1 = InstantDt::from_str("2009-12-23T23:12:45.234Z")?;
//     tracing::debug!("Instant: {}", instant1);
//     let instant1 = InstantDt::from_str("2009-12-23T23:12:45-06:00")?;
//     tracing::debug!("Instant: {}", instant1);
//     let instant1 = InstantDt::from_str("2009-12-23T23:12:45.456-06:00")?;
//     tracing::debug!("Instant: {}", instant1);
//     Ok(())
// }
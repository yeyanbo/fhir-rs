use tracing::Level;
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
        .set_id("example")
        .set_meta(Meta::default()
            .set_version_id("v1")
            .set_last_updated(InstantDt::from_str("2001-10-10T10:10:12Z")?))
        .set_active(true)
        .add_identifier(Identifier::default()
            .set_use_("usual")
            .set_system("urn:oid:1.2.36.146.595.217.0.1")
            .set_value("12345"))       
        .add_name(HumanName::default()
            .set_use_("maiden")
            .set_family("Windsor")
            .add_given("Peter")
            .add_given("Tom"))
        .add_name(HumanName::default()
            .set_use_("maiden")
            .set_family("Blacksmith")
            .add_given("Jack")
            .add_given("Tom2"))
        .add_telecom(ContactPoint::default()
            .set_value("1234567890")
            .set_use_("work")
            .set_rank(1))
        .set_gender("male");
    
    println!("Resource Name: {}", &patient.resource_name());

    match test_fhirpath_1(&patient) {
        Ok(_) => println!("执行完成"),
        Err(err) => println!("出错了：{}", err),
    }

    match test_fhirpath_2(&patient) {
        Ok(_) => println!("评估执行完成"),
        Err(err) => println!("评估出错了：{}", err),
    }

    match test_fhirpath_3(&patient) {
        Ok(_) => println!("评估执行完成"),
        Err(err) => println!("评估出错了：{}", err),
    }
    Ok(())
}

fn test_fhirpath_1(patient: &Patient) -> Result<()> {

    let mut paths = FhirPaths::parse("Patient.name.given".to_string())?;
    let collection = patient.path(&mut paths)?;
    
    println!("Result count: {}", collection.count());

    for item in collection.0 {
        println!("Item: {:?}", item);
    }

    Ok(())
}

fn test_fhirpath_2(patient: &Patient) -> Result<()> {

    let mut paths = FhirPaths::parse("Patient.name.given.empty()".to_string())?;
    let rs = patient.eval(&mut paths)?;
    
    println!("Eval Result : {}", rs);

    Ok(())
}

fn test_fhirpath_3(patient: &Patient) -> Result<()> {

    let collection = patient.path2("Patient.name[1].given.value".to_string())?;
    
    match collection {
        Some(coll) => {
            println!("Result count: {}", coll.count());

            for item in coll.0 {
                println!("Item: {:?}", item);
            }
        },
        None => println!("Result Empty"),
    }
    
    Ok(())
}

use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use fhir_rs::prelude::*;

fn main() -> Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    let encounter_str = include_str!("encounter_example_02.xml");
    let encounter: Encounter = from_xml(encounter_str)?;

    let profile_str = include_str!("profile-core-outpatient-encounter.xml");
    let profile: StructureDefinition = from_xml(profile_str)?;

    let mut validator = Validator::new(profile);
    let outcome = validator.validate(&encounter)?;

    println!("Validate Outcome: {:#?}", &outcome);
    Ok(())
}

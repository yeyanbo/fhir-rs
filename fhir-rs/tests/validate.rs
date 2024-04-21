use fhir_rs::prelude::*;

#[test]
pub fn test_validate() -> Result<()> {
    let encounter_str = include_str!("encounter_example_02.xml");
    let encounter: Encounter = from_xml(encounter_str)?;

    tracing::info!("Encounter Class: {:?}", encounter.class);

    let profile_str = include_str!("profile-core-outpatient-encounter.xml");
    let profile: StructureDefinition = from_xml(profile_str)?;
   
    tracing::info!("Profle Class: {:?}", profile.name);

    assert_eq!("a", "a");

    Ok(())
}
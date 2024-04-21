use fhir_rs::prelude::*;

#[test]
pub fn test_validate() -> Result<()> {
    let encounter_str = include_str!("encounter_example_02.xml");
    let encounter: Encounter = from_xml(encounter_str)?;

    assert_eq!(encounter.id, Some("encounter-example-02".to_string()));
    Ok(())
}
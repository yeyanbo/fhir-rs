use fhir_rs::prelude::*;

#[test]
pub fn test_validate() -> Result<()> {
    let encounter_str = include_str!("encounter_example_02.xml");
    let encounter: Encounter = from_xml(encounter_str)?;

    assert_eq!(encounter.id, Some("encounter-example-02".to_string()));
    Ok(())
}

#[test]
pub fn validate_choice_element() -> Result<()> {
    let patient_str = include_str!("patient-example.xml");
    let patient: Patient = from_xml(patient_str)?;
    println!("{:?}", patient.deceased);
    Ok(())
}

#[test]
pub fn validate_profile() -> Result<()> {
    let profile_str = include_str!("profile-core-outpatient-encounter.xml");
    let profile: StructureDefinition = from_xml(profile_str)?;
    println!("{:?}", profile.snapshot.unwrap().element.unwrap().get(0).unwrap().pattern);
    Ok(())
}
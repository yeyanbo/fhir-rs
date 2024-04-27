
#[cfg(test)]
mod fhirpath {
    use fhir_rs::prelude::*;
    use tracing::Level;
    use tracing_subscriber::FmtSubscriber;
    use tracing::info;

    pub fn setup() {
        let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    #[test]
    pub fn test_encounter_fhirpath() -> Result<()> {
        let encounter_str = include_str!("encounter_example_02.xml");
        let encounter: Encounter = from_xml(encounter_str)?;

        let mut expr = PathExpression::parse("Encounter.meta".to_string())?;
        let collection = encounter.path(&mut expr)?;
    
        println!("Result count: {}", &collection.count());
        
        for item in collection.0 {
            println!("Item: {:?}", item);
        }
        Ok(())
    }
    
    #[test]
    pub fn test_patient_fhirpath() -> Result<()> {
        let patient_str = include_str!("patient-example.xml");
        let patient: Patient = from_xml(patient_str)?;
    
        let mut expr = PathExpression::parse("Patient.gender".to_string())?;
        let collection = patient.path(&mut expr)?;
     
        println!("Result count: {}", &collection.count());
        
        for item in collection.0 {
            println!("Item: {:?}", item);
        }
        Ok(())
    }
    
    #[test]
    pub fn test_patient_name_fhirpath() -> Result<()> {
        info!("abc");

        let patient_str = include_str!("patient-example.xml");
        let patient: Patient = from_xml(patient_str)?;
    
        let mut expr = PathExpression::parse("Patient.name.given".to_string())?;
        let collection = patient.path(&mut expr)?;
    
        assert_eq!(collection.count(), 5);
        Ok(())
    }

    #[test]
    pub fn test_patient_name_with_index_fhirpath() -> Result<()> {
        let patient_str = include_str!("patient-example.xml");
        let patient: Patient = from_xml(patient_str)?;
    
        let mut expr = PathExpression::parse("Patient.name[2].given".to_string())?;
        let collection = patient.path(&mut expr)?;
        
        assert_eq!(collection.count(), 2);
        Ok(())
    }

    #[test]
    pub fn test_filter() -> Result<()> {
        let patient_str = include_str!("patient-example.xml");
        let patient: Patient = from_xml(patient_str)?;
    
        let mut expr = PathExpression::parse("Patient.name.where(given = 'abc' )".to_string())?;
        let collection = patient.path(&mut expr)?;
        
        assert_eq!(collection.count(), 0);
        Ok(())
    }
}
#[test]
fn test_all_none() -> Result<()>{
    let patient = Patient {
        name: None,
        age: None,
        telecom:None,
        class:None,
    };
    let str = to_xml(&patient)?;
    let expect = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Patient xmlns=\"http://hl7.org/fhir\" />";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_with_value() -> Result<()>{
    let patient = Patient {
        name: None,
        age: Some(PositiveIntDt {
            id: None,
            extension: None,
            value:  Some(32)}),
        telecom:None,
        class:None,
    };
    let str = to_xml(&patient)?;
    let expect = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Patient xmlns=\"http://hl7.org/fhir\"><age value=\"32\" /></Patient>";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_with_id_but_value_none() -> Result<()>{
    let patient = Patient {
        name: None,
        age: Some(PositiveIntDt {
            id: Some("32".to_string()),
            extension: None,
            value:  None}),
        telecom:None,
        class:None,
    };
    let str = to_xml(&patient)?;
    let expect = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Patient xmlns=\"http://hl7.org/fhir\"><age id=\"32\" /></Patient>";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_vec() -> Result<()>{
    let patient = Patient {
        name: None,
        age: None,
        telecom: Some(vec![
            StringDt { id: None, extension: None, value: Some("010-12345678".to_string()) },
            StringDt { id: None, extension: None, value: Some("022-98765432".to_string()) }
        ]),
        class:None,
    };
    let str = to_xml(&patient)?;
    let expect = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Patient xmlns=\"http://hl7.org/fhir\"><telecom value=\"010-12345678\" /><telecom value=\"022-98765432\" /></Patient>";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_complex_vec() -> Result<()>{
    let patient = Patient {
        name: None,
        age: None,
        telecom: None,
        class: Some(vec![
            Coding {
                id: None,
                extension: None,
                code: Some(StringDt { id: None, extension: None, value: Some("123".to_string()) }),
                system: Some(StringDt { id: None, extension: None, value: Some("http://fhir.org".to_string()) }),
                version: None,
                display: Some(StringDt { id: None, extension: None, value: Some("Feed".to_string()) })
            },
            Coding {
                id: Some("id-456".to_string()),
                extension: None,
                code: Some(StringDt { id: None, extension: None, value: Some("453".to_string()) }),
                system: Some(StringDt { id: None, extension: None, value: Some("http://fhir.org".to_string()) }),
                version: None,
                display: Some(StringDt { id: None, extension: None, value: Some("Food".to_string()) })
            }
        ])
    };
    let str = to_xml(&patient)?;
    let expect = "<?xml version=\"1.0\" encoding=\"UTF-8\"?><Patient xmlns=\"http://hl7.org/fhir\"><class><code value=\"123\" /><system value=\"http://fhir.org\" /><display value=\"Feed\" /></class><class id=\"id-456\"><code value=\"453\" /><system value=\"http://fhir.org\" /><display value=\"Food\" /></class></Patient>";
    Ok(assert_eq!(str, expect))
}
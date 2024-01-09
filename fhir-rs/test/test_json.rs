use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use fhir_resource_r5::Patient;
use fhir_rs::{Any, Coding, Extension, PositiveIntDt, StringDt};
use fhir_rs::ser::to_json;
use crate::Result;


fn prepare() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
}
#[test]
fn test_all_none() -> Result<()>{

    let patient = Patient {
        name: None,
        age: None,
        telecom:None,
        class:None,
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\"}";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_string() -> Result<()>{

    let patient = Patient {
        name: Some(StringDt{
            id: None,
            extension: None,
            value: Some("zhangsan".to_string()),
        }),
        age: None,
        telecom: None,
        class:None,
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\",\"name\":\"zhangsan\"}";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_int() -> Result<()>{
    prepare();
    let patient = Patient {
        name: None,
        age: Some(PositiveIntDt {
            id: None,
            extension: None,
            value: Some(989) }),
        telecom: None,
        class:None,
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\",\"age\":989}";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_with_extension() -> Result<()>{

    let patient = Patient {
        name: None,
        age: Some(PositiveIntDt {
            id: Some("id-468".to_string()),
            // extension: None,
            extension: Some(vec![
                Extension {
                    id: None,
                    url: Some("count".to_string()),
                    extension: None,
                    value: Some(Any::PositiveInt(PositiveIntDt { id: None, extension: None, value: Some(200) }))
                },
                Extension {
                    id: None,
                    url: Some("world".to_string()),
                    extension: None,
                    value: Some(Any::Coding(Coding {
                        id: Some("id-890".to_string()),
                        extension: None,
                        code: Some(StringDt { id: None, extension: None, value: Some("123".to_string()) }),
                        system: Some(StringDt { id: None, extension: None, value: Some("http://fhir.org".to_string()) }),
                        version: None,
                        display: Some(StringDt { id: None, extension: None, value: Some("Feed".to_string()) })
                    }))
                }
            ]),
            value: Some(989) }),
        telecom: None,
        class:None,
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\",\"age\":989,\"_age\":{\"id\":\"id-468\",\"extension\":[{\"url\":\"count\",\"valuePositiveInt\":200},{\"url\":\"world\",\"valueCoding\":{\"id\":\"id-890\",\"code\":\"123\",\"system\":\"http://fhir.org\",\"display\":\"Feed\"}}]}}";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_primitive_with_extension_but_none_value() -> Result<()>{

    let patient = Patient {
        name: None,
        age: Some(PositiveIntDt {
            id: Some("id-468".to_string()),
            // extension: None,
            extension: Some(vec![
                Extension {
                    id: None,
                    url: Some("count".to_string()),
                    extension: None,
                    value: Some(Any::PositiveInt(PositiveIntDt { id: None, extension: None, value: Some(200) }))
                },
                Extension {
                    id: None,
                    url: Some("world".to_string()),
                    extension: None,
                    value: Some(Any::Coding(Coding {
                        id: Some("id-890".to_string()),
                        extension: None,
                        code: Some(StringDt { id: None, extension: None, value: Some("123".to_string()) }),
                        system: Some(StringDt { id: None, extension: None, value: Some("http://fhir.org".to_string()) }),
                        version: None,
                        display: Some(StringDt { id: None, extension: None, value: Some("Feed".to_string()) })
                    }))
                }
            ]),
            value: None }),
        telecom: None,
        class:None,
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\",\"_age\":{\"id\":\"id-468\",\"extension\":[{\"url\":\"count\",\"valuePositiveInt\":200},{\"url\":\"world\",\"valueCoding\":{\"id\":\"id-890\",\"code\":\"123\",\"system\":\"http://fhir.org\",\"display\":\"Feed\"}}]}}";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_vec() -> Result<()>{

    let patient = Patient {
        name: None,
        age: None,
        telecom: Some(vec![
            StringDt{
                id: None,
                extension: None,
                value: Some("010-123456".to_string()),
            },
            StringDt{
                id: None,
                extension: None,
                value: Some("010-456789".to_string()),
            },
        ]),
        class:None,
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\",\"telecom\":[\"010-123456\",\"010-456789\"]}";
    Ok(assert_eq!(str, expect))
}

#[test]
fn test_coding() -> Result<()>{

    let patient = Patient {
        name: None,
        age: None,
        telecom: None,
        class: Some(vec![
            Coding {
                id: Some("id-123".to_string()),
                extension: None,
                code: Some(StringDt { id: None, extension: None, value: Some("123".to_string()) }),
                system: Some(StringDt { id: None, extension: None, value: Some("http://fhir.org".to_string()) }),
                version: None,
                display: Some(StringDt { id: None, extension: None, value: Some("Feed".to_string()) })
            },
            Coding {
                id: None,
                extension: Some(vec![
                    Extension {
                        id: None,
                        url: Some("count".to_string()),
                        extension: None,
                        value: Some(Any::PositiveInt(PositiveIntDt { id: None, extension: None, value: Some(200) }))
                    }
                ]),
                code: Some(StringDt { id: None, extension: None, value: Some("453".to_string()) }),
                system: Some(StringDt { id: None, extension: None, value: Some("http://fhir.org".to_string()) }),
                version: None,
                display: Some(StringDt { id: None, extension: None, value: Some("Food".to_string()) })
            }
        ])
    };
    let str = to_json(&patient)?;
    let expect = "{\"resourceType\":\"Patient\",\"class\":[{\"id\":\"id-123\",\"code\":\"123\",\"system\":\"http://fhir.org\",\"display\":\"Feed\"},{\"extension\":[{\"url\":\"count\",\"valuePositiveInt\":200}],\"code\":\"453\",\"system\":\"http://fhir.org\",\"display\":\"Food\"}]}";
    Ok(assert_eq!(str, expect))
}
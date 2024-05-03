# fhir-rs

## About

FHIR-RS library is an implementation of [HL7 FHIR Specification](http://hl7.org/fhir/) for Rust.

Read this in other languages: 
* [简体中文](README_zh_CN.md)

## Versioning

This is still a draft version. The functionality is not yet fully developed and the performance has not been optimized, making it unsuitable for production environments.

## Usage

Healthcare Interoperability.

### Parse

```rust
use fhir_rs::prelude::*;

fn main() -> Result<()> {

    let complex_extension = Extension::with_url("http://yeyanbo.cn")
        .add_extension(Extension::new("abc", Any::Coding(Coding::default().set_code("cc"))))
        .add_extension(Extension::new("xcv", Any::Coding(Coding::default().set_code("bb"))));

    let patient = Patient::default()
        .set_id("example")
        .set_meta(Meta::default()
            .set_version_id("v1")
            .set_last_updated(InstantDt::from_str("2001-10-10T10:10:12Z")?))
        .add_extension(Extension::new("dd", Any::String(StringDt::new("ddf"))))
        .add_extension(complex_extension)
        .add_name(HumanName::default().set_text("Mike"))
        .add_telecom(ContactPoint::default().set_value("1234567890"));

    test_xml_serialize(&patient)?;
    test_json_serialize(&patient)?;
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
```

### Fhirpath

At present, the framework for FHIRPath syntax parsing has been completed, but the function support is limited and only a few function methods are supported:
* empty()
* exists()

operator：
* and
* =

### Validate

```rust
use fhir_rs::prelude::*;

fn main() -> Result<()> {
    let encounter_str = include_str!("encounter_example_02.xml");
    let encounter: Encounter = from_xml(encounter_str)?;

    let profile_str = include_str!("profile-core-outpatient-encounter.xml");
    let profile: StructureDefinition = from_xml(profile_str)?;

    let mut validator = Validator::new(profile);
    let outcome = validator.validate(&encounter)?;

    println!("Validate Outcome: {:#?}", &outcome);
    Ok(())
}
```

## Complete examples

* Transform 
* FhirPath
* Validate

## Done

- [x] datatype
- [x] resource
- [x] from json to resource
- [x] from xml  to resource
- [x] from resource to json
- [x] from resource to xml
- [x] fhirpath
- [x] validator

## Todo

- [ ] fhir client
- [ ] fhir server

## License

CC0
# fhir-rs

## About

FHIR-RS是[HL7 FHIR规范](http://hl7.org/fhir/)的Rust实现。

FHIR-RS library is an implementation of [HL7 FHIR Specification](http://hl7.org/fhir/) for Rust.

## Versioning

这仅是一个草稿版。功能尚未开发完成，性能尚未优化，因此不适合生产环境。

This is still a draft version. The functionality is not yet fully developed and the performance has not been optimized, making it unsuitable for production environments.

## Usage

健康信息数据交互。

Healthcare Interoperability.


```rust
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

## Todo

### Ver 0.0.3

以完善已有功能为主要目标。新建三个新模块，占位而已。

- [ ] fhir client
- [ ] fhir server
- [ ] fhir validate

### Ver 0.0.2

- [x] datatype
- [x] resource
- [x] from json to resource
- [x] from xml  to resource
- [x] from resource to json
- [x] from resource to xml

## License

CC0
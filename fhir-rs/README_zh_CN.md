# fhir-rs

## 简介

FHIR-RS是[HL7 FHIR规范](http://hl7.org/fhir/)的Rust实现。

以其它语言阅读:
* [英文](README.md)


## 版本说明

这仅是一个草稿版。功能尚未开发完成，性能尚未优化，因此不适合生产环境。

## 使用

健康信息数据交互。

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

目前FHIRPath语法解析的框架已经完成，但是，函数支持有限，仅支持有限的几个函数方法：
* empty()
* exists()

操作符：
* and
* =


### 基于Profile的验证

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

## 完整示例

* [Transform](examples/transform)
* [FhirPath](examples/fhirpath)
* [Validate](examples/validator)

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
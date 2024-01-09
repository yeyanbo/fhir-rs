mod generate_resource;
mod generate_type;

use std::env;
use std::fs::File;
use std::io::{Cursor, Read, Write};
use std::path::Path;
use convert_case::Casing;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use generate_resource::DefinitionParser;
use generate_type::DataTypeDefinitionParser;

fn main() -> anyhow::Result<()> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");

    generate_resource()?;
    // generate_datatype()?;

    Ok(())
}

fn generate_resource() -> anyhow::Result<()> {
    let mut reader = Cursor::new(include_str!("../profiles-resources.xml"));

    let current_path = env::current_dir()?;
    let dest_path = Path::new(&current_path)
        .join("fhir-resource-r5")
        .join("src")
        .join("resource");

    let mut mod_file = File::create(dest_path.join("mod.rs"))?;

    let mut parser = DefinitionParser::new(reader);
    parser.work_through()?;

    parser.write_mod(mod_file)?;
    parser.write_resources(&dest_path)?;
    Ok(())
}

fn generate_datatype() -> anyhow::Result<()> {
    let mut reader = Cursor::new(include_str!("../profiles-types.xml"));

    let current_path = env::current_dir()?;
    let dest_path = Path::new(&current_path)
        .join("fhir-rs")
        .join("src")
        .join("datatype");

    let mut datatype_file = File::create(dest_path.join("datatype.rs"))?;

    let mut parser = DataTypeDefinitionParser::new(reader);
    parser.work_through()?;

    parser.write_resources(&mut datatype_file)?;
    Ok(())
}

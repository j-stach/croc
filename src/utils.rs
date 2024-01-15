
use cargo_metadata::MetadataCommand;
use serde_json::Value;

macro_rules! croc_doc {{
    "TODO Documentation  >,==,~".to_string()
}}


pub(crate) fn get_option_from_metadata(opt_name: &str) -> Option<Value> {
    // Use cargo_metadata to read values from Cargo.toml
    let metadata = MetadataCommand::new().exec().unwrap();

    // Find the value in the specified custom subsection
    if let Some(package) = metadata.packages.iter().find(|p| p.name == "croc") {
        if let Some(section) = package.metadata.get("croc") {
            if let Some(value) = section.get(opt_name) {
                return Some(value.clone());
            }
        }
    }

    None
}









use serde::Serialize;
use std::{collections::HashMap, fs, path::PathBuf};
use toml::Value;
use regex::Regex;

#[derive(Serialize, Debug)]
pub struct Metadata {
    pub author: String,
    pub r#type: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub license: String,
    pub repository: Option<String>,
    pub distributor: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FunctionParam {
    pub name: String,
    pub r#type: Option<String>,
    pub description: Option<String>,
    pub default: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct FunctionExample {
    pub name: String,
    pub description: String,
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct FunctionDoc {
    pub definition: String,
    pub description: Option<String>,
    pub params: Vec<FunctionParam>,
    pub examples: Vec<FunctionExample>,
}

#[derive(Serialize, Debug)]
pub struct TypeDoc {
    pub definition: String,
    pub description: Option<String>,
    pub usage: Option<String>,
    pub implementations: Vec<String>,
}

#[derive(Serialize, Debug)]
pub struct ProjectDoc {
    pub metadata: Metadata,
    pub dependencies: Vec<String>,
    pub types: HashMap<String, TypeDoc>,
    pub functions: HashMap<String, FunctionDoc>,
}

fn format_dependency_purl(name: &str, version: Option<&str>) -> String {
    match version {
        Some(v) => format!("pkg:cargo/{}@{}", name, v),
        None => format!("pkg:cargo/{}", name),
    }
}

pub fn parsecargo(project_dir: PathBuf) -> std::io::Result<ProjectDoc> {
    let cargo_toml_path = project_dir.join("Cargo.toml");
    let cargo_content = fs::read_to_string(&cargo_toml_path)?;
    let cargo_toml: Value = toml::from_str(&cargo_content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    
    let binding = toml::map::Map::new();
    let package = cargo_toml.get("package")
        .and_then(|v| v.as_table())
        .unwrap_or(&binding);

    let metadata = Metadata {
        author: package.get("authors")
            .and_then(|a| a.as_array())
            .and_then(|arr| arr.get(0))
            .and_then(|v| v.as_str())
            .unwrap_or("").to_string(),
        r#type: "rust-project".to_string(),
        name: package.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        version: package.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        description: package.get("description").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        license: package.get("license").and_then(|v| v.as_str()).unwrap_or("").to_string(),
        repository: package.get("repository").and_then(|v| v.as_str()).map(|s| s.to_string()),
        distributor: None,
    };

    let dependencies: Vec<String> = cargo_toml.get("dependencies")
        .and_then(|v| v.as_table())
        .unwrap_or(&toml::map::Map::new())
        .iter()
        .map(|(k, v)| {
            let version = if v.is_str() {
                v.as_str()
            } else if let Some(tbl) = v.as_table() {
                tbl.get("version").and_then(|vv| vv.as_str())
            } else {
                None
            };
            format_dependency_purl(k, version)
        })
        .collect();

    // implement types and functions
    Ok( ProjectDoc{
        metadata: metadata,
        dependencies: dependencies,
        types: HashMap::new(),
        functions: HashMap::new() 
    })
}

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

    let mut types = HashMap::new();
    let mut functions = HashMap::new();

    let src_path = project_dir.join("src");
    let doc_comment_re = Regex::new(r"(?m)/// ?(.*)").unwrap();
    let fn_re = Regex::new(r"(?m)^[ \t]*pub\s+fn\s+(\w+)\s*\(([^)]*)\)").unwrap();
    let type_re = Regex::new(r"(?ms)^[ \t]*pub\s+(struct|enum|trait)\s+(\w+)(.*?\n\})").unwrap();

    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let content = fs::read_to_string(entry.path())?;

        for cap in fn_re.captures_iter(&content) {
            let name = cap[1].to_string();
            let definition = format!("fn {}({})", cap[1].to_string(), cap[2].to_string());

            let params: Vec<FunctionParam> = cap[2]
                .split(',')
                .filter_map(|p| {
                    let trimmed = p.trim();
                    if trimmed.is_empty() { return None; }
                    let parts: Vec<&str> = trimmed.split(':').map(|s| s.trim()).collect();
                    Some(FunctionParam {
                        name: parts.get(0).unwrap_or(&"").to_string(),
                        r#type: parts.get(1).map(|s| s.to_string()),
                        description: None,
                        default: None,
                    })
                })
                .collect();

            let description = doc_comment_re.captures_iter(&content)
                .map(|c| c[1].to_string())
                .collect::<Vec<_>>()
                .join(" ");

            functions.insert(name.clone(), FunctionDoc {
                definition,
                description: Some(description),
                params,
                examples: vec![],
            });
        }

        for cap in type_re.captures_iter(&content) {
            let name = cap[2].to_string();
            let definition = format!("{} {}{}", cap[1].to_string(), name, cap[3].to_string());
            let description = doc_comment_re.captures_iter(&content)
                .map(|c| c[1].to_string())
                .collect::<Vec<_>>()
                .join(" ");

            types.insert(name.clone(), TypeDoc {
                definition,
                description: Some(description),
                usage: None,
                implementations: vec![],
            });
        }
    }

    Ok(ProjectDoc {
        metadata,
        dependencies,
        types,
        functions,
    })
}

use cargo_dok;
use std::path::PathBuf;
use crate::detect::ProjectType;
use serde_json;

pub fn parse(path: PathBuf, pt: ProjectType) -> String {
    match pt{
        ProjectType::Cargo => {
            let res = cargo_dok::parsecargo(path).unwrap();
            let json = serde_json::to_string_pretty(&res).unwrap();
            return json;
        }
        _ => {
            return String::new();
        }
    }
}

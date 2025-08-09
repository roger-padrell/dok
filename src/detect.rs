use std::path::PathBuf;
use std::fs;
use strum_macros::Display;

#[derive(PartialEq, Eq, Display)]
pub enum ProjectType{
    Cargo, Node, Python // to expand
}

pub fn detect_project(p: &PathBuf) -> ProjectType {
    // Cargo
    if p.join("Cargo.toml").exists() {
        return ProjectType::Cargo;
    } else {
        eprintln!("Error: The entered directory ({}) is not of a valid project structure.", p.display());
        std::process::exit(1);
    }
}

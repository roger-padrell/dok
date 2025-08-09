use std::path::PathBuf;
use clap::{Parser, ValueHint};
use strum_macros::EnumString;
use std::fs;
use std::io;

mod parse;
mod detect;

#[derive(PartialEq, Eq, EnumString, Clone, Debug, Default)]
pub enum Action{
    #[default]
    parse,
    md,
    html
}

#[derive(Debug, Parser)]
#[command(version, about = "Require an existing directory as input")]
struct Args {
    #[arg(value_parser = parse_action)]
    action: Action,
    #[arg(value_parser = path_exists, value_hint = ValueHint::DirPath)]
    path: PathBuf,
}

fn parse_action(s: &str) -> Result<Action, String> {
    let parsed: Result<Action, _> = s.parse();
    match parsed{
        Ok(action) => Ok(action),
        Err(_) => Err(format!("'{}' is not an available command", s))
    }
}

fn path_exists(s: &str) -> Result<PathBuf, String> {
    let p = PathBuf::from(s);
    if p.exists() {
        Ok(p)
    } else {
        Err(format!("'{}' does not exist", s))
    }
}

fn main() {
    let args = Args::parse();

    match args.action {
        Action::parse => {
            if !args.path.is_dir() {
                eprintln!("Error: For 'parse', the path must be a directory.");
                std::process::exit(1);
            }
            println!("=> Directory exists: {}", args.path.display());
            println!("=> Detecting project type...");
            let ptype = crate::detect::detect_project(&args.path);
            println!("  -> Project type detected: {}", &ptype);
            println!("=> Parsing project...");
            let cont = crate::parse::parse(args.path, ptype);
            fs::write("documentation.dok", cont).expect("Error writing output .dok file");
        }
        _ => {
            if !args.path.is_file() {
                eprintln!("Error: For '{:?}', the path must be a file.", args.action);
                std::process::exit(1);
            }
        }
    }
}


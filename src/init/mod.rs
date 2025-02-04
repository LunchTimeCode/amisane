use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::utils::Print;
pub mod config;

pub fn init(print: &mut Print) -> Result<(), Box<dyn std::error::Error>> {
    gitignore(print);
    default_config(print);
    print.success("Init healthy");
    Ok(())
}

fn default_config(print: &mut Print) {
    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true)
        .open("amisane.toml")
        .unwrap();

    let toml_contents = std::fs::read_to_string("amisane.toml").unwrap();

    if !toml_contents.is_empty() {
        print.info("Amisane toml already exists");
        return;
    }

    let c = config::Config::default();

    if let Err(e) = writeln!(file, "{}", toml::to_string(&c).unwrap()) {
        let x = format!("Couldn't write to file: {}", e);
        print.failure(x);
    }

    print.intermediate("amisane.toml created");
}

fn gitignore(print: &mut Print) {
    let mut file: File = OpenOptions::new()
        .append(true)
        .create(true)
        .open(".gitignore")
        .unwrap();

    let gitignore_contents = std::fs::read_to_string(".gitignore").unwrap();

    if gitignore_contents.contains("amisane") {
        print.info("Amisane folder already ignored");
        return;
    }

    if let Err(e) = writeln!(file) {
        let x = format!("Couldn't write to file: {}", e);
        print.failure(x);
    }

    if let Err(e) = writeln!(file, "# generated from amisane ") {
        let x = format!("Couldn't write to file: {}", e);
        print.failure(x);
    }

    if let Err(e) = writeln!(file, "amisane") {
        let x = format!("Couldn't write to file: {}", e);
        print.failure(x);
    }

    print.intermediate("Ignoring amisane folder");
}

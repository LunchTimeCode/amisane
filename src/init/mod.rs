use std::fs::{File, OpenOptions};
use std::io::Write;

use crate::utils::Print;

pub fn init(print: &mut Print) -> Result<(), Box<dyn std::error::Error>> {
    gitignore(print);

    print.warning("Not yet fully implemented");
    Ok(())
}

fn gitignore(print: &mut Print) {
    let mut file: File = OpenOptions::new().append(true).open(".gitignore").unwrap();

    let gitignore_contents = std::fs::read_to_string(".gitignore").unwrap();

    if gitignore_contents.contains("amisane") {
        print.intermediate("Amisane folder already ignored");
        return;
    }

    if let Err(e) = writeln!(file) {
        let x = format!("Couldn't write to file: {}", e);
        print.failure(x);
    }

    if let Err(e) = writeln!(file, "# amisane generated") {
        eprintln!("Couldn't write to file: {}", e);
    }

    if let Err(e) = writeln!(file, "amisane") {
        eprintln!("Couldn't write to file: {}", e);
    }

    print.intermediate("Amisane folder already ignored");
}

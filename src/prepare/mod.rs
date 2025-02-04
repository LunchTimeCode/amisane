use std::{
    fs::{File, OpenOptions, create_dir_all},
    io::Write,
    path::Path,
};

use crate::{
    init::config::{Config, SanityPaths},
    utils::Print,
};

mod meta;

pub fn prepare(print: &mut Print) {
    let Some(c) = find_config(print) else {
        return;
    };

    let Ok(_) = create_dir_all("amisane") else {
        print.failure("Couldn't create amisane dir");
        return;
    };

    let Ok(_) = create_dir_all("amisane/files") else {
        print.failure("Couldn't create amisane files dir");
        return;
    };

    let read = meta::sanity_read(print, c.get_paths());

    let to_write = match toml::to_string_pretty(&read) {
        Ok(o) => o,
        Err(e) => {
            let msg = format!("Couldn't serialize amisane_read.toml {}", e);
            print.failure(msg);
            return;
        }
    };

    let mut file: File = match OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("amisane/amisane_read.toml")
    {
        Ok(f) => f,
        Err(e) => {
            let msg = format!("Couldn't write to file: {}", e);
            print.failure(msg);
            return;
        }
    };

    if let Err(e) = writeln!(file, "{}", to_write) {
        let msg = format!("Couldn't write to file: {}", e);
        print.failure(msg);
        return;
    }

    if !read.get_paths().is_empty()
        && path_exists("amisane/amisane_read.toml")
        && files_copied(print, c.get_paths())
    {
        print.success("Prepare is healthy");
    } else {
        print.failure("Prepare is corrupted");
    };
}

fn find_config(print: &mut Print) -> Option<Config> {
    let Ok(config) = std::fs::read_to_string("amisane.toml") else {
        print.failure("Couldn't read file: amisane.toml");
        return None;
    };

    let config = match toml::from_str(&config) {
        Ok(c) => c,
        Err(e) => {
            let msg = format!("Couldn't parse file: amisane.toml: {}", e);
            print.failure(msg);
            return None;
        }
    };
    Some(config)
}

pub fn path_exists(path: &str) -> bool {
    Path::new(path).exists()
}

fn files_copied(print: &mut Print, paths: Vec<SanityPaths>) -> bool {
    let mut success = true;
    for path in paths {
        if path.get_copy_files() && !path_exists(&path.get_path()) {
            let msg = format!("Couldn't find file: {}", path.get_path());
            print.failure(msg);

            if success {
                success = false;
            }
        }
    }
    success
}

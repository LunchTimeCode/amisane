use std::{
    fs,
    hash::{DefaultHasher, Hash, Hasher},
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{
    init::config::{Sanity, SanityPaths},
    utils::Print,
};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SanityReadPaths {
    paths: Vec<SanityRead>,
}

impl SanityReadPaths {
    pub fn get_paths(&self) -> Vec<SanityRead> {
        self.paths.clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileHash(String);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SanityRead {
    path: String,

    file_hash: Option<FileHash>,
}

pub fn sanity_read(print: &mut Print, paths: Vec<SanityPaths>) -> SanityReadPaths {
    let reads: Vec<SanityRead> = paths
        .iter()
        .map(|path| sanity_read_path(print, path))
        .collect();

    SanityReadPaths { paths: reads }
}

fn sanity_read_path(print: &mut Print, path: &SanityPaths) -> SanityRead {
    let tests = path.get_tests();
    let copy_files = path.get_copy_files();

    let mut file_hash: Option<FileHash> = None;

    if let Ok(file_contents) = std::fs::read_to_string(path.get_path()) {
        for test in tests {
            match test {
                Sanity::FileHash => {
                    let hash = calculate_hash(&file_contents);
                    file_hash = Some(FileHash(hash.to_string()));
                }
            }
        }
    } else {
        let message = format!("Couldn't read file: {}", path.get_path());
        print.failure(message);
    };

    if copy_files {
        let target_path = format!(
            "amisane/files/{}",
            Path::new(&path.get_path())
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        );

        match fs::copy(path.get_path(), target_path.clone()) {
            Ok(_) => {
                let msg = format!("Copied file: {}", path.get_path());
                print.info(&msg);
            }
            Err(e) => {
                let msg = format!(
                    "Couldn't copy file: from {} to {}, from: {e}",
                    path.get_path(),
                    target_path
                );
                print.failure(&msg);
            }
        }
    };

    SanityRead {
        path: path.get_path(),
        file_hash,
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

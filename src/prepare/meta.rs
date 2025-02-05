use std::{fs, path::Path};

use serde::{Deserialize, Serialize};

use crate::{
    hash::{self},
    init::config::SanityPaths,
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

impl FileHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }
}

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

    let file_hash: Option<FileHash> = hash::hash(print, path, tests);

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

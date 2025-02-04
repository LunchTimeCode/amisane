use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    copy_files: bool,
    paths: Vec<SanityPaths>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SanityPaths {
    path: String,
    tests: Vec<Sanity>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
enum Sanity {
    #[default]
    FileHash,
}

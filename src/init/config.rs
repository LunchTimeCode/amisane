use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    paths: Vec<SanityPaths>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            paths: sanity_example_paths(),
        }
    }
}

fn sanity_example_paths() -> Vec<SanityPaths> {
    vec![
        SanityPaths {
            path: "example/somefile.csv".to_string(),
            copy_files: false,
            tests: vec![Sanity::FileHash],
        },
        SanityPaths {
            path: "otherexample/otherfile.csv".to_string(),
            copy_files: false,
            tests: vec![Sanity::FileHash],
        },
    ]
}

#[derive(Debug, Deserialize, Serialize)]
struct SanityPaths {
    path: String,
    copy_files: bool,
    tests: Vec<Sanity>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
enum Sanity {
    #[default]
    FileHash,
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    paths: Vec<SanityPaths>,
}

impl Config {
    pub fn get_paths(&self) -> Vec<SanityPaths> {
        self.paths.clone()
    }
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
            path: "example/otherfile.csv".to_string(),
            copy_files: false,
            tests: vec![Sanity::FileHash],
        },
    ]
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SanityPaths {
    path: String,
    copy_files: bool,
    tests: Vec<Sanity>,
}

impl SanityPaths {
    pub fn get_path(&self) -> String {
        self.path.clone()
    }

    pub fn get_copy_files(&self) -> bool {
        self.copy_files
    }

    pub fn get_tests(&self) -> Vec<Sanity> {
        self.tests.clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum Sanity {
    #[default]
    FileHash,
}

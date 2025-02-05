use crate::{
    init::config::{Sanity, SanityPaths},
    prepare::FileHash,
    utils::Print,
};

use std::hash::{DefaultHasher, Hash, Hasher};

pub fn hash(print: &mut Print, path: &SanityPaths, tests: Vec<Sanity>) -> Option<FileHash> {
    let mut file_hash: Option<FileHash> = None;

    if let Ok(file_contents) = std::fs::read_to_string(path.get_path()) {
        for test in tests {
            match test {
                Sanity::FileHash => {
                    let hash = calculate_hash(&file_contents);
                    file_hash = Some(FileHash::new(hash.to_string()));
                }
            }
        }
    } else {
        let message = format!("Couldn't read file: {}", path.get_path());
        print.failure(message);
    };

    file_hash
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

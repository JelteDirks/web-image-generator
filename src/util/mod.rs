use std::path::{Path, PathBuf};
use std::fs;

pub fn check_output(path: &PathBuf) -> PathBuf {
    let as_path: &Path = path.as_path();
    let path_exists = as_path.exists();

    if !path_exists {
        fs::create_dir_all(&as_path);
    } else if !as_path.is_dir(){
        // exists but is not a directory, problem!
        panic!("output path is not a directory {:?}", as_path);
    }
    return as_path.to_path_buf();
}

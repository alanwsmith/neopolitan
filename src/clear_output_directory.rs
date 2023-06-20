use std::fs;
use std::path::PathBuf;

pub fn clear_output_directory() {
    let output_dir = PathBuf::from("site");
    if output_dir.exists() {
        fs::remove_dir_all(output_dir).unwrap();
    }
}

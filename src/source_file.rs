use std::path::PathBuf;

pub mod output_path;
pub mod template;
pub mod title;

pub struct SourceFile {
    pub source_data: String,
    pub source_path: PathBuf,
}

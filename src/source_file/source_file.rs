use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceFile {
    pub source_data: String,
    pub source_path: PathBuf,
}

use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceFile {
    pub source_data: Option<String>,
    pub source_path: Option<PathBuf>,
}

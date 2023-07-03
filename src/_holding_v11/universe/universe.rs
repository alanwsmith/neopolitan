use crate::source_file::source_file::SourceFile;
use minijinja::Environment;
use std::collections::HashMap;
use std::path::PathBuf;

// Deprecate and remove this once the new
// method of using content files vec is in
// place

#[derive(Clone, Debug)]
pub struct Universe<'a> {
    pub assets_dir: Option<PathBuf>,
    // pub categories: HashMap<String, Vec<PathBuf>>,
    pub content_dir: Option<PathBuf>,
    pub content_files: HashMap<PathBuf, SourceFile>,
    pub env: Option<Environment<'a>>,
    pub output_root: Option<PathBuf>,
}

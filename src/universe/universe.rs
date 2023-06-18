use crate::source_file::source_file::SourceFile;
use minijinja::Environment;
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Universe<'a> {
    pub assets_dir: Option<PathBuf>,
    pub content_dir: Option<PathBuf>,
    pub content_files: HashMap<PathBuf, SourceFile>,
    pub env: Option<Environment<'a>>,
    pub output_root: Option<PathBuf>,
}

// impl Universe<'_> {
//     pub fn scrub_url_path(source: String) -> String {
//         let re = Regex::new(r"\s+").unwrap();
//         re.replace_all(&source, "-").to_lowercase()
//     }
// }

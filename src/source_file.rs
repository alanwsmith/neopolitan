use std::path::PathBuf;
use serde::Serialize;

pub mod content;
pub mod output_path;
pub mod template;
pub mod title;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub struct SourceFile {
    pub source_data: String,
    pub source_path: PathBuf,
}

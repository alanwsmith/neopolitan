pub mod content;
pub mod content_type;
pub mod output_dir;
pub mod output_path;
pub mod parse_block;
pub mod sections;
pub mod template;
pub mod title;
pub mod url;


use std::path::PathBuf;

#[derive(Debug)]
pub struct SourceFile {
    pub source_data: String,
    pub source_path: PathBuf,
}

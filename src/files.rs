use crate::source_file::SourceFile;

pub mod all_posts;

#[derive(Debug)]
pub struct Files {
    pub files: Vec<SourceFile>,
}

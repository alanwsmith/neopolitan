use crate::source_file::SourceFile;

pub mod all_posts;
pub mod new;

#[derive(Debug)]
pub struct Files {
    pub files: Vec<SourceFile>,
}

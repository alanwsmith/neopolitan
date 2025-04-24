use crate::source_file::SourceFile;

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            source_data: "".to_string(),
            source_path: "".into()
        }
    }
}
use crate::source_file::SourceFile;
use std::path::PathBuf;

impl SourceFile {
    pub fn output_dir(&self) -> Option<PathBuf> {
        let output_dir = &self.source_path;
        let output_dir = output_dir.parent().unwrap();
        Some(output_dir.to_path_buf())
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_path() {
        let sf = SourceFile {
            source_data: "".to_string(),
            source_path: PathBuf::from("site/example/index.neo"),
        };
        let expected = Some(PathBuf::from("site/example"));
        assert_eq!(expected, sf.output_dir());
    }
}

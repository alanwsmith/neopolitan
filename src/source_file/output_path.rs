use crate::source_file::source_file::SourceFile;
use std::path::PathBuf;

impl SourceFile {
    pub fn output_path(&self) -> Option<PathBuf> {
        let mut output_path = self.source_path.to_path_buf();
        output_path.set_extension("html");
        Some(output_path)
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_path() {
        let expected = Some(PathBuf::from("site/example/index.html"));
        let mut sf = SourceFile {
            source_data: "".to_string(),
            source_path: PathBuf::from("site/example/index.neo"),
        };
        assert_eq!(expected, sf.output_path());
    }
}

use crate::source_file::source_file::SourceFile;
use std::path::PathBuf;

impl SourceFile {
    pub fn output_path(&self) -> Option<PathBuf> {
        let mut output_path = self.source_path.as_ref().unwrap().clone();
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
        let mut sf = SourceFile::new();
        sf.source_path = Some(PathBuf::from("site/example/index.neo"));
        let expected = Some(PathBuf::from("site/example/index.html"));
        assert_eq!(expected, sf.output_path());
    }
}

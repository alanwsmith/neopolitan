use crate::source_file::source_file::SourceFile;
use std::path::PathBuf;

impl SourceFile {
    pub fn output_path(&self) -> Option<PathBuf> {
        let mut output_path = self.raw_path.clone().unwrap();
        output_path.set_extension("html");
        Some(output_path)
    }
}

#[cfg(test)]
mod test {
    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_basic_output_path() {
        let mut sf = SourceFile::new();
        sf.raw_path = Some(PathBuf::from("index.neo"));
        let expected = PathBuf::from("index.html");
        let result = sf.output_path().unwrap();
        assert_eq!(expected, result);
    }

    #[test]
    pub fn test_multi_dir_path() {
        let mut sf = SourceFile::new();
        sf.raw_path = Some(PathBuf::from("some/path/alfa.neo"));
        let expected = PathBuf::from("some/path/alfa.html");
        let result = sf.output_path().unwrap();
        assert_eq!(expected, result);
    }
}

use crate::source_file::source_file::SourceFile;
use std::path::PathBuf;

impl SourceFile {
    pub fn url(&self) -> Option<PathBuf> {
        let mut output_path = PathBuf::from("/");
        output_path.push(self.source_path.as_ref().unwrap());
        output_path.set_extension("html");
        Some(output_path)
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn check_url() {
        let mut sf = SourceFile::new();
        sf.source_path = Some(PathBuf::from("site/example/index.neo"));
        assert_eq!(sf.url(), Some(PathBuf::from("/site/example/index.html")));
    }
}

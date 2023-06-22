use crate::source_file::SourceFile;
use std::path::PathBuf;

impl SourceFile {
    pub fn url(&self) -> Option<PathBuf> {
        let mut output_path = PathBuf::from("/");
        output_path.push(&self.source_path);
        output_path.set_extension("html");
        Some(output_path)
    }
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn check_url() {
        let sf = SourceFile {
            source_path: PathBuf::from("site/example/index.neo"),
            source_data: "".to_string(),
        };
        assert_eq!(sf.url(), Some(PathBuf::from("/site/example/index.html")));
    }
}

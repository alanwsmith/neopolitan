use std::path::PathBuf;

pub fn output_path() -> Option<PathBuf> {
    // let mut output_path = self.source_path.to_path_buf();
    // output_path.set_extension("html");
    None
}

#[cfg(test)]

mod test {
    use crate::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn test_path() {
        let expected = Some(PathBuf::from("site/example/index.html"));
        let sf = SourceFile {
            source_data: "".to_string(),
            source_path: PathBuf::from("site/example/index.neo"),
        };
        // assert_eq!(expected, output_path());
    }
}

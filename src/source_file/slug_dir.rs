use crate::source_file::source_file::SourceFile;
use regex::Regex;
use std::path::PathBuf;

impl SourceFile {
    pub fn slug_dir(&self) -> Option<PathBuf> {
        let mut base_string = self
            .raw_path
            .as_ref()
            .unwrap()
            .with_extension("")
            .display()
            .to_string();

        let re = Regex::new(r"index$").unwrap();
        base_string = re.replace(base_string.as_str(), "").to_string();

        let re = Regex::new(r"\s+").unwrap();
        Some(PathBuf::from(
            re.replace_all(base_string.as_str(), "-").to_lowercase(),
        ))
    }
}



#[cfg(test)]

mod test {

    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn slug_dir_with_scrubbed_path() {
        let mut sf = SourceFile::new();
        sf.raw_path = Some(PathBuf::from("This Is/Some/Path.neo"));
        let expected = Some(PathBuf::from("this-is/some/path"));
        let result = sf.slug_dir();
        assert_eq!(expected, result);
    }

    #[test]
    pub fn slug_dir_with_scrubbed_path_for_index() {
        let mut sf = SourceFile::new();
        sf.raw_path = Some(PathBuf::from("a/Path stop/index.neo"));
        let expected = Some(PathBuf::from("a/path-stop"));
        let result = sf.slug_dir();
        assert_eq!(expected, result);
    }


}

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

        let re = Regex::new(r"^\w+-+").unwrap();
        base_string = re.replace(base_string.as_str(), "").to_string();

        let re = Regex::new(r"index$").unwrap();
        base_string = re.replace(base_string.as_str(), "").to_string();
        let mut dir_check = base_string.clone();

        let re = Regex::new(r"\s+").unwrap();
        let mut dash_string = re.replace_all(base_string.as_str(), "-").to_lowercase();
        match dir_check.pop() {
            Some(char_check) => {
                if char_check != '/' {
                    dash_string.push_str("-");
                    dash_string.push_str(self.id().unwrap().as_str());
                }
            }
            None => (),
        }
        Some(PathBuf::from(dash_string))
        // Some(PathBuf::from("a/path-stop"))
    }
}

#[cfg(test)]

mod test {

    use crate::parse::parse::parse;
    use crate::source_file::source_file::SourceFile;
    use std::path::PathBuf;

    #[test]
    pub fn slug_dir_with_scrubbed_path() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> attributes", ">> id: 1234asdf", ""];
        let text = lines.join("\n");
        let parsed_data = parse(text.as_str());
        sf.parsed = parsed_data.unwrap().1;
        sf.raw_path = Some(PathBuf::from("This  Is/Some/Path.neo"));
        let expected = Some(PathBuf::from("this-is/some/path-1234asdf"));
        let result = sf.slug_dir();
        assert_eq!(expected, result);
    }

    #[test]
    pub fn slug_dir_with_scrubbed_path_for_index() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> attributes", ">> id: 4343dsds", ""];
        let text = lines.join("\n");
        let parsed_data = parse(text.as_str());
        sf.parsed = parsed_data.unwrap().1;
        sf.raw_path = Some(PathBuf::from("a/Path stop/index.neo"));
        let expected = Some(PathBuf::from("a/path-stop"));
        let result = sf.slug_dir();
        assert_eq!(expected, result);
    }

    #[test]
    pub fn slug_dir_with_scrubbed_path_for_root_index() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> attributes", ">> id: 2323rtrt", ""];
        let text = lines.join("\n");
        let parsed_data = parse(text.as_str());
        sf.parsed = parsed_data.unwrap().1;
        sf.raw_path = Some(PathBuf::from("index.html"));
        let expected = Some(PathBuf::from(""));
        let result = sf.slug_dir();
        assert_eq!(expected, result);
    }

    #[test]
    pub fn remove_initial_filename_slug() {
        let mut sf = SourceFile::new();
        let lines = vec!["-> attributes", ">> id: 7612iuiu", ""];
        let text = lines.join("\n");
        let parsed_data = parse(text.as_str());
        sf.parsed = parsed_data.unwrap().1;
        sf.raw_path = Some(PathBuf::from("something-whatever.neo"));
        let expected = Some(PathBuf::from("whatever-7612iuiu"));
        let result = sf.slug_dir();
        assert_eq!(expected, result);
    }
}

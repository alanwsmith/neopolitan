use crate::source_file::source_file::SourceFile;

impl SourceFile {
    pub fn template(&self) -> String {
        String::from("")
    }
}

#[cfg(test)]
mod test {
    // use crate::parse::parse::*;
    use crate::source_file::source_file::SourceFile;
    #[test]
    pub fn file_status() {
        let mut sf = SourceFile::new();
        let lines = ["-> attributes", ">> template: delta", ""];
        let expected = Some("delta".to_string());
        let raw = lines.join("\n").to_string();
        // sf.parsed = parse(raw.as_str()).unwrap().1;
        assert_eq!(1, 1);
    }
}

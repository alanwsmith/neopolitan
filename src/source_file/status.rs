use crate::section::section::Section;
use crate::section::section_attributes::SectionAttribute::Attribute;
use crate::source_file::source_file::SourceFile;

impl SourceFile {
    pub fn status(&self) -> Option<String> {
        for section in self.parsed.as_ref().unwrap().iter() {
            match section {
                Section::AttributesSection {
                    attributes,
                    children: _,
                } => {
                    for attribute in attributes.as_ref().unwrap().iter() {
                        match attribute {
                            Attribute { key, value } => {
                                if key.as_ref().unwrap() == "status" {
                                    return Some(value.as_ref().unwrap().to_string());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use crate::parse::parse::*;
    use crate::source_file::source_file::SourceFile;
    #[test]
    pub fn file_status() {
        let mut sf = SourceFile::new();
        let lines = ["-> attributes", ">> status: draft", ""];
        let expected = Some("draft".to_string());
        let raw = lines.join("\n").to_string();
        sf.parsed = parse(raw.as_str()).unwrap().1;
        assert_eq!(expected, sf.status());
    }
}

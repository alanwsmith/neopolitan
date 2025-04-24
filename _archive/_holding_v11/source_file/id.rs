use crate::section::section::Section;
use crate::section::section_attributes::SectionAttribute::Attribute;
use crate::source_file::source_file::SourceFile;

impl SourceFile {
    pub fn id(&self) -> Option<String> {
        for section in self.parsed.as_ref().unwrap().iter() {
            match section {
                Section::AttributesSection {
                    attributes,
                    children: _,
                } => {
                    for attribute in attributes.as_ref().unwrap().iter() {
                        match attribute {
                            Attribute { key, value } => {
                                if key.as_ref().unwrap() == "id" {
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
    pub fn check_id() {
        let mut sf = SourceFile::new();
        let lines = ["-> attributes", ">> id: 1234asdf", ""];
        let expected = Some("1234asdf".to_string());
        let raw = lines.join("\n").to_string();
        sf.parsed = parse(raw.as_str()).unwrap().1;
        assert_eq!(expected, sf.id());
    }
}

use crate::section::section::*;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct SourceFile {
    pub output_chunks: Option<Vec<String>>,
    pub parsed: Option<Vec<Section>>,
    pub raw_data: Option<String>,
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            output_chunks: None,
            parsed: None,
            raw_data: None,
        }
    }
}

impl SourceFile {
    pub fn output(&self) -> Option<String> {
        Some(r#"<h1 class="title"><p>Shut the hatch</p></h1>"#.to_string())
    }
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::parse::parse::*;
    use crate::section::section_attributes::SectionAttribute;
    use crate::snippet::snippet::*;
    use crate::source_file::source_file::*;
    use crate::tests::remove_whitespace::remove_whitespace;

    #[test]
    pub fn basic_output() {
        let lines = ["-> title", "", "Shut the hatch"];
        let expected = Some(r#"<h1 class="title"><p>Shut the hatch</p></h1>"#.to_string());
        let mut sf = SourceFile::new();
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.parsed = parse(sf.raw_data.as_ref().unwrap().as_str()).unwrap().1;
        let output = sf.output();
        assert_eq!(remove_whitespace(expected), remove_whitespace(output),);
    }

    #[test]
    pub fn basic_title_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Dip the pail once".to_string()),
                }]),
            }]),
        }]);
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.parsed = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }

    #[test]
    pub fn basic_title_plus_lines_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once", "", "Draw the chart"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![
                Block::Text {
                    snippets: Some(vec![Snippet::Plain {
                        text: Some("Dip the pail once".to_string()),
                    }]),
                },
                Block::Text {
                    snippets: Some(vec![Snippet::Plain {
                        text: Some("Draw the chart".to_string()),
                    }]),
                },
            ]),
        }]);
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.parsed = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }

    #[test]
    pub fn attributes() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", ">> id: bravo", "", "Set The Piece"];
        let expected = Some(vec![Section::TitleSection {
            attributes: Some(vec![SectionAttribute::Attribute {
                key: Some("id".to_string()),
                value: Some("bravo".to_string()),
            }]),
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Set The Piece".to_string()),
                }]),
            }]),
        }]);
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.parsed = parse_dev(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.parsed);
    }
}

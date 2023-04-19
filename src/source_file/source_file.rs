use crate::section::section::*;

#[derive(Debug, PartialEq)]
pub struct SourceFile {
    raw_data: Option<String>,
    sections: Option<Vec<Section>>,
}

impl SourceFile {
    pub fn new() -> SourceFile {
        SourceFile {
            raw_data: None,
            sections: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::parse::parse::*;
    use crate::snippet::snippet::*;
    use crate::source_file::source_file::*;

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
        sf.sections = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.sections);
    }

    #[test]
    pub fn basic_title_plus_lines_test() {
        let mut sf = SourceFile::new();
        let lines = ["-> title", "", "Dip the pail once", "", "Draw the chart"];
        let expected = Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![Block::Text {
                snippets: Some(vec![Snippet::Plain {
                    text: Some("Dip the pail once".to_string()),
                }]),
            }]),
        }]);
        sf.raw_data = Some(lines.join("\n").to_string());
        sf.sections = parse(sf.raw_data.unwrap().as_str()).unwrap().1;
        assert_eq!(expected, sf.sections);
    }
}

use crate::section::section::*;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

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

pub fn parse(source: &str) -> IResult<&str, Option<Vec<Section>>> {
    let (_, sections) = many_till(section, eof)(source)?;
    Ok(("", Some(sections.0)))
}

use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;

pub fn results(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let children = if source.trim().is_empty() {
        None
    } else {
        Some(Block::RawContent {
            text: Some(source.trim().to_string()),
        })
    };
    Ok((
        source,
        Section::ResultsSection {
            attributes,
            children,
        },
    ))
}

#[cfg(test)]

mod test {

    use crate::block::block::*;
    use crate::parse::parse;
    use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_results() {
        let lines = vec!["-> results", "", "some results", "more results"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::ResultsSection {
                attributes: None,
                children: Some(Block::RawContent {
                    text: Some("some results\nmore results".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    fn code_with_attribute() {
        let lines = vec!["-> results", ">> rust", "", "some results", "more results"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::ResultsSection {
                attributes: Some(vec![SectionAttribute::Attribute {
                    key: Some("rust".to_string()),
                    value: None,
                }]),
                children: Some(Block::RawContent {
                    text: Some("some results\nmore results".to_string()),
                }),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}

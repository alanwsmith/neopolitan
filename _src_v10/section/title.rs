use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let (source, b) = many_till(block, eof)(source.trim()).unwrap();
    let children = if b.0.is_empty() { None } else { Some(b.0) };
    Ok((
        source,
        Section::Title {
            attributes,
            children,
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::block::block::*;
    use crate::content::content::*;
    use crate::parse::parse;
    use crate::section::attributes_for_section::*;
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_title() {
        let lines = vec!["-> title", ">> id: bravo", "", "some content"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::Title {
                attributes: Some(vec![SectionAttribute::Attribute {
                    key: Some("id".to_string()),
                    value: Some("bravo".to_string()),
                }]),
                children: Some(vec![Block::P {
                    children: Some(vec![
                        Content::Text {
                            text: Some("some".to_string()),
                        },
                        Content::Space,
                        Content::Text {
                            text: Some("content".to_string()),
                        },
                    ]),
                }]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }

    #[test]
    fn multi_paragraph_title() {
        let lines = vec![
            "-> title",
            ">> id: bravo",
            "",
            "some content",
            "with lines",
            "",
            "and another",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::Title {
                attributes: Some(vec![SectionAttribute::Attribute {
                    key: Some("id".to_string()),
                    value: Some("bravo".to_string()),
                }]),
                children: Some(vec![
                    Block::P {
                        children: Some(vec![
                            Content::Text {
                                text: Some("some".to_string()),
                            },
                            Content::Space,
                            Content::Text {
                                text: Some("content".to_string()),
                            },
                            Content::Space,
                            Content::Text {
                                text: Some("with".to_string()),
                            },
                            Content::Space,
                            Content::Text {
                                text: Some("lines".to_string()),
                            },
                        ]),
                    },
                    Block::P {
                        children: Some(vec![
                            Content::Text {
                                text: Some("and".to_string()),
                            },
                            Content::Space,
                            Content::Text {
                                text: Some("another".to_string()),
                            },
                        ]),
                    },
                ]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}

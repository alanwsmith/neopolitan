use crate::block::block::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn p(source: &str) -> IResult<&str, Section> {
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
        Section::Paragraphs {
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
    use crate::section::section::*;
    use crate::wrapper::wrapper::*;

    #[test]
    fn basic_p() {
        let lines = vec![
            "-> p",
            "",
            "the book cover",
            "",
            "random string",
            "with content",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![
                Section::Paragraphs {
                    attributes: None,
                    children: Some(vec![
                        Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("the".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("book".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("cover".to_string()),
                                },
                            ]),
                        },
                        Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("random".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("string".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("with".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("content".to_string()),
                                },
                            ]),
                        },
                    ]),
                },
            ]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}

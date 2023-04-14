use crate::block::notes_item::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn notes(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let _attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let (source, b) = many_till(notes_item, eof)(source.trim()).unwrap();
    let children = if b.0.is_empty() { None } else { Some(b.0) };
    Ok((
        source,
        Section::NotesSection {
            attributes: None,
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
    fn basic_list() {
        let lines = vec!["-> notes", "", "- note alfa", "", "- note bravo"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::NotesSection {
                attributes: None,
                children: Some(vec![
                    Block::NotesItem {
                        attributes: None,
                        children: Some(vec![Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("note".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("alfa".to_string()),
                                },
                            ]),
                        }]),
                    },
                    Block::NotesItem {
                        attributes: None,
                        children: Some(vec![Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("note".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("bravo".to_string()),
                                },
                            ]),
                        }]),
                    },
                ]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }

    #[ignore]
    #[test]
    fn multi_item_list() {
        let lines = vec![
            "-> list",
            "",
            "- note alfa",
            "apple",
            "",
            "bravo",
            "",
            "- note charlie",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::NoteSection {
                attributes: None,
                children: Some(vec![
                    Block::NotesItem {
                        attributes: None,
                        children: Some(vec![
                            Block::P {
                                children: Some(vec![
                                    Content::Text {
                                        text: Some("note".to_string()),
                                    },
                                    Content::Space,
                                    Content::Text {
                                        text: Some("alfa".to_string()),
                                    },
                                    Content::Space,
                                    Content::Text {
                                        text: Some("apple".to_string()),
                                    },
                                ]),
                            },
                            Block::P {
                                children: Some(vec![Content::Text {
                                    text: Some("bravo".to_string()),
                                }]),
                            },
                        ]),
                    },
                    Block::NotesItem {
                        attributes: None,
                        children: Some(vec![Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("note".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("charlie".to_string()),
                                },
                            ]),
                        }]),
                    },
                ]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}

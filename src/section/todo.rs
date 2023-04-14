use crate::block::todo_item::*;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::IResult;

pub fn todo(source: &str) -> IResult<&str, Section> {
    let (source, att_capture) = many0(preceded(tag(">> "), section_attribute))(source).unwrap();
    let _attributes = if att_capture.is_empty() {
        None
    } else {
        Some(att_capture)
    };
    let (source, b) = many_till(todo_item, eof)(source.trim()).unwrap();
    let children = if b.0.is_empty() { None } else { Some(b.0) };
    Ok((
        source,
        Section::ToDoSection {
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
    fn basic_todo() {
        let lines = vec!["-> todo", "", "[] todo alfa", "", "[] todo bravo"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::ToDoSection {
                attributes: None,
                children: Some(vec![
                    Block::ToDoItem {
                        status: None,
                        attributes: None,
                        children: Some(vec![Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("todo".to_string()),
                                },
                                Content::Space,
                                Content::Text {
                                    text: Some("alfa".to_string()),
                                },
                            ]),
                        }]),
                    },
                    Block::ToDoItem {
                        status: None,
                        attributes: None,
                        children: Some(vec![Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("todo".to_string()),
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

    #[test]
    fn multi_item_list() {
        let lines = vec![
            "-> todo",
            "",
            "[] item alfa",
            "apple",
            "",
            "bravo",
            "",
            "[] item charlie",
        ]
        .join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::ToDoSection {
                attributes: None,
                children: Some(vec![
                    Block::ToDoItem {
                        status: None,
                        attributes: None,
                        children: Some(vec![
                            Block::P {
                                children: Some(vec![
                                    Content::Text {
                                        text: Some("item".to_string()),
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
                    Block::ToDoItem {
                        status: None,
                        attributes: None,
                        children: Some(vec![Block::P {
                            children: Some(vec![
                                Content::Text {
                                    text: Some("item".to_string()),
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

    #[test]
    fn todo_with_status() {
        let lines = vec!["-> todo", "", "[x] todo alfa"].join("\n");
        let source = lines.as_str();
        let expected = Wrapper::Page {
            children: Some(vec![Section::ToDoSection {
                attributes: None,
                children: Some(vec![Block::ToDoItem {
                    status: Some("x".to_string()),
                    attributes: None,
                    children: Some(vec![Block::P {
                        children: Some(vec![
                            Content::Text {
                                text: Some("todo".to_string()),
                            },
                            Content::Space,
                            Content::Text {
                                text: Some("alfa".to_string()),
                            },
                        ]),
                    }]),
                }]),
            }]),
        };
        let result = parse(source).unwrap().1;
        assert_eq!(expected, result);
    }
}

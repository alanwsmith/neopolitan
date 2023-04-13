use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
// use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;

#[test]
fn hotel() {
    let lines = vec!["-> list", "", "- item alfa", "", "- item bravo"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::List {
            attributes: None,
            children: Some(vec![
                Block::UnorderedListItem {
                    attributes: None,
                    children: Some(vec![Block::P {
                        children: Some(vec![
                            Content::Text {
                                text: Some("item".to_string()),
                            },
                            Content::Space,
                            Content::Text {
                                text: Some("alfa".to_string()),
                            },
                        ]),
                    }]),
                },
                Block::UnorderedListItem {
                    attributes: None,
                    children: Some(vec![Block::P {
                        children: Some(vec![
                            Content::Text {
                                text: Some("item".to_string()),
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
fn hotel_2() {
    let lines = vec![
        "-> list",
        "",
        "- item alfa",
        "apple",
        "",
        "bravo",
        "",
        "- item charlie",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::List {
            attributes: None,
            children: Some(vec![
                Block::UnorderedListItem {
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
                Block::UnorderedListItem {
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

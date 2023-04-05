use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
// use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;

#[test]
fn alfa() {
    let lines = vec![
        "-> title",
        "",
        "quick <<link|example.com|brown>> fox",
        "",
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
            Section::Title {
                attributes: None,
                children: Some(vec![Block::P {
                    children: Some(vec![
                        Content::Text {
                            text: Some("quick".to_string()),
                        },
                        Content::Space,
                        Content::Link {
                            attributes: None,
                            text: Some("brown".to_string()),
                            url: Some("example.com".to_string()),
                        },
                        Content::Space,
                        Content::Text {
                            text: Some("fox".to_string()),
                        },
                    ]),
                }]),
            },
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
    println!("Process complete");
}

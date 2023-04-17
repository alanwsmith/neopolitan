use crate::attribute::*;
use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::attributes_for_section::*;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[ignore]
#[test]
fn charlie() {
    let lines = vec![
        "-> title",
        ">> id: opening",
        "",
        "quick <<link|brown|example.com|class: active>> fox",
        "",
        "-> p",
        ">> alfa: bravo charlie",
        ">> delta: echo",
        "",
        "the book cover",
        "",
        "random string",
        "with content",
        "",
        "-> p",
        "",
        "the long boat",
        "",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![
            Section::Title {
                attributes: Some(vec![
                    (SectionAttribute::Attribute {
                        key: Some("id".to_string()),
                        value: Some("opening".to_string()),
                    }),
                ]),
                children: Some(vec![Block::P {
                    children: Some(vec![
                        Content::Text {
                            text: Some("quick".to_string()),
                        },
                        Content::Space,
                        Content::Link {
                            attributes: Some(vec![Attribute::Basic {
                                key: Some("class".to_string()),
                                value: Some("active".to_string()),
                            }]),
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
                attributes: Some(vec![
                    SectionAttribute::Attribute {
                        key: Some("alfa".to_string()),
                        value: Some("bravo charlie".to_string()),
                    },
                    SectionAttribute::Attribute {
                        key: Some("delta".to_string()),
                        value: Some("echo".to_string()),
                    },
                ]),
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
            Section::Paragraphs {
                attributes: None,
                children: Some(vec![Block::P {
                    children: Some(vec![
                        Content::Text {
                            text: Some("the".to_string()),
                        },
                        Content::Space,
                        Content::Text {
                            text: Some("long".to_string()),
                        },
                        Content::Space,
                        Content::Text {
                            text: Some("boat".to_string()),
                        },
                    ]),
                }]),
            },
        ]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
    println!("Process complete");
}

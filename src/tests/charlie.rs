use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;

// #[ignore]
#[test]
fn charlie() {
    let lines = vec![
        "-> title",
        ">> id: opening",
        "",
        "quick <<link|example.com|brown>> fox",
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
                            text: "quick".to_string(),
                        },
                        Content::Space,
                        Content::Link {
                            text: "brown".to_string(),
                            url: "example.com".to_string(),
                        },
                        Content::Space,
                        Content::Text {
                            text: "fox".to_string(),
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
                                text: "the".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "book".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "cover".to_string(),
                            },
                        ]),
                    },
                    Block::P {
                        children: Some(vec![
                            Content::Text {
                                text: "random".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "string".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "with".to_string(),
                            },
                            Content::Space,
                            Content::Text {
                                text: "content".to_string(),
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
                            text: "the".to_string(),
                        },
                        Content::Space,
                        Content::Text {
                            text: "long".to_string(),
                        },
                        Content::Space,
                        Content::Text {
                            text: "boat".to_string(),
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

use crate::enums::*;
use crate::parse::parse;

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
        children: vec![
            Section::Title {
                children: vec![Block::P {
                    children: vec![
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
                    ],
                }],
            },
            Section::Paragraphs {
                children: vec![
                    Block::P {
                        children: vec![
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
                        ],
                    },
                    Block::P {
                        children: vec![
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
                        ],
                    },
                ],
            },
        ],
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
    println!("Process complete");
}

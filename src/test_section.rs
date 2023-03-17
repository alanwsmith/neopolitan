use crate::chunk::Chunk;
use crate::section::*;
use std::collections::HashMap;

// TODO: Add test with spaces after the
// title and then on the empty line
// and then in both places.

#[test]
fn test_basic_title() {
    let source = vec!["-> TITLE", "", "Alfa Bravo"].join("\n");
    let expected = Section::TitleSection {
        children: vec![Chunk::H1 {
            attributes: HashMap::from([("class".to_string(), "title".to_string())]),
            children: vec![Chunk::Text {
                value: "Alfa Bravo".to_string(),
            }],
        }],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_paragraphs_after_title() {
    let source = vec![
        "-> TITLE",
        "",
        "Alfa Bravo",
        "",
        "Charlie delta echo",
        "",
        "Foxtrot golf hotel",
    ]
    .join("\n");
    let expected = Section::TitleSection {
        children: vec![
            Chunk::H1 {
                attributes: HashMap::from([("class".to_string(), "title".to_string())]),
                children: vec![Chunk::Text {
                    value: "Alfa Bravo".to_string(),
                }],
            },
            Chunk::P {
                attributes: HashMap::new(),
                children: vec![Chunk::Text {
                    value: "Charlie delta echo".to_string(),
                }],
            },
            Chunk::P {
                attributes: HashMap::new(),
                children: vec![Chunk::Text {
                    value: "Foxtrot golf hotel".to_string(),
                }],
            },
        ],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_attribute_on_title() {
    let source = vec!["-> TITLE", ">> id: main", "", "Alfa Bravo"].join("\n");
    let expected = Section::TitleSection {
        children: vec![Chunk::H1 {
            attributes: HashMap::from([
                ("id".to_string(), "main".to_string()),
                ("class".to_string(), "title".to_string()),
            ]),
            children: vec![Chunk::Text {
                value: "Alfa Bravo".to_string(),
            }],
        }],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_basic_paragraph() {
    let source = vec!["-> P", "", "The tree top"].join("\n");
    let expected = Section::ParagraphSection {
        children: vec![Chunk::P {
            attributes: HashMap::from([]),
            children: vec![Chunk::Text {
                value: "The tree top".to_string(),
            }],
        }],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_multiple_paragraphs() {
    let source = vec!["-> P", "", "Echo Foxtrot", "", "Our plans right now."].join("\n");
    let expected = Section::ParagraphSection {
        children: vec![
            Chunk::P {
                attributes: HashMap::from([]),
                children: vec![Chunk::Text {
                    value: "Echo Foxtrot".to_string(),
                }],
            },
            Chunk::P {
                attributes: HashMap::from([]),
                children: vec![Chunk::Text {
                    value: "Our plans right now.".to_string(),
                }],
            },
        ],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_attributes_on_paragraphs() {
    let source = vec![
        "-> P",
        ">> class: mighty",
        "",
        "The sand drifts",
        "",
        "Twist the valve",
    ]
    .join("\n");
    let expected = Section::ParagraphSection {
        children: vec![
            Chunk::P {
                attributes: HashMap::from([("class".to_string(), "mighty".to_string())]),
                children: vec![Chunk::Text {
                    value: "The sand drifts".to_string(),
                }],
            },
            Chunk::P {
                attributes: HashMap::from([("class".to_string(), "mighty".to_string())]),
                children: vec![Chunk::Text {
                    value: "Twist the valve".to_string(),
                }],
            },
        ],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_inline_code() {
    let source = vec!["-> P", "", "The `sand`rust` drifts"].join("\n");
    let expected = Section::ParagraphSection {
        children: vec![Chunk::P {
            attributes: HashMap::from([]),
            children: vec![
                Chunk::Text {
                    value: "The ".to_string(),
                },
                Chunk::InlineCode {
                    language: Some("rust".to_string()),
                    attributes: None,
                    value: Some("sand".to_string()),
                },
                Chunk::Text {
                    value: " drifts".to_string(),
                },
            ],
        }],
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

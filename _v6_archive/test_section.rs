#![allow(warnings)]
use crate::chunk::Chunk;
use crate::section::*;
use std::collections::HashMap;

#[test]
fn test_basic_title() {
    let source = vec!["-> TITLE", "", "Alfa Bravo"].join("\n");
    let expected = Section::TitleSection {
        attributes: None,
        children: Some(vec![Chunk::H1 {
            attributes: None,
            children: Some(vec![Chunk::Text {
                attributes: None,
                value: Some("Alfa Bravo".to_string()),
            }]),
        }]),
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
        attributes: None,
        children: Some(vec![
            Chunk::H1 {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Alfa Bravo".to_string()),
                }]),
            },
            Chunk::P {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Charlie delta echo".to_string()),
                }]),
            },
            Chunk::P {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Foxtrot golf hotel".to_string()),
                }]),
            },
        ]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_attribute_on_title() {
    let source = vec!["-> TITLE", ">> id: main", "", "Alfa Bravo"].join("\n");
    let expected = Section::TitleSection {
        attributes: None,
        children: Some(vec![Chunk::H1 {
            attributes: Some(HashMap::from([(
                "id".to_string(),
                Some("main".to_string()),
            )])),
            children: Some(vec![Chunk::Text {
                attributes: None,
                value: Some("Alfa Bravo".to_string()),
            }]),
        }]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_basic_paragraph() {
    let source = vec!["-> P", "", "The tree top"].join("\n");
    let expected = Section::ParagraphSection {
        attributes: None,
        children: Some(vec![Chunk::P {
            attributes: None,
            children: Some(vec![Chunk::Text {
                attributes: None,
                value: Some("The tree top".to_string()),
            }]),
        }]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_multiple_paragraphs() {
    let source = vec!["-> P", "", "Echo Foxtrot", "", "Our plans right now."].join("\n");
    let expected = Section::ParagraphSection {
        attributes: None,
        children: Some(vec![
            Chunk::P {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Echo Foxtrot".to_string()),
                }]),
            },
            Chunk::P {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Our plans right now.".to_string()),
                }]),
            },
        ]),
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
        attributes: None,
        children: Some(vec![
            Chunk::P {
                attributes: Some(HashMap::from([(
                    "class".to_string(),
                    Some("mighty".to_string()),
                )])),
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("The sand drifts".to_string()),
                }]),
            },
            Chunk::P {
                attributes: Some(HashMap::from([(
                    "class".to_string(),
                    Some("mighty".to_string()),
                )])),
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Twist the valve".to_string()),
                }]),
            },
        ]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn test_inline_code() {
    let source = vec!["-> P", "", "The `sand`rust` drifts"].join("\n");
    let expected = Section::ParagraphSection {
        attributes: None,
        children: Some(vec![Chunk::P {
            attributes: None,
            children: Some(vec![
                Chunk::Text {
                    attributes: None,
                    value: Some("The ".to_string()),
                },
                Chunk::InlineCode {
                    language: Some("rust".to_string()),
                    // attributes: Some(vec![(Some("rust".to_string()), None)]),
                    attributes: Some(HashMap::from([("rust".to_string(), None)])),
                    value: Some("sand".to_string()),
                },
                Chunk::Text {
                    attributes: None,
                    value: Some(" drifts".to_string()),
                },
            ]),
        }]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn section_code_test() {
    let source = vec!["-> CODE", "", "The logs fell"].join("\n");
    let expected = Section::CodeSection {
        attributes: None,
        language: None,
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some("The logs fell".to_string()),
        }]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn section_code_test_with_language() {
    let source = vec!["-> CODE", ">> rust", "", "a long list"].join("\n");
    let expected = Section::CodeSection {
        attributes: Some(HashMap::from([("rust".to_string(), None)])),
        language: Some("rust".to_string()),
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some("a long list".to_string()),
        }]),
    };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn horizontal_rule() {
    let source = vec!["-> hr", "", "a long list"].join("\n");
    let expected = Section::HRSection { attributes: None };
    let result = section(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}
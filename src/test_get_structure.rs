/////////////////////////////////////////////
// This is the basic integration test
// location. GOAL: It has a few samples
// that demonstrate the core features
// of the format. Detailed variatsions are
// checked via unit tests.
/////////////////////////////////////////////

#![allow(warnings)]
use crate::chunk::Chunk;
use crate::get_structure::get_structure;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

#[test]
fn basic_integration() {
    let source = vec![
        "-> TITLE",
        ">> id: main",
        "",
        "Alfa Bravo",
        "",
        "The quick brown fox",
        "and the lazy dog",
        "",
        "-> P",
        "",
        "They took the axe and the",
        "saw to the forest.",
        "",
        "The bark of the pine tree.",
        "",
        "-> P",
        ">> class: main",
        "",
        "The `chink`rust` in the wall.",
        "",
        "The `desk`python` and `both`javascript` chairs",
        "",
    ]
    .join("\n");
    let expected = Page {
        attributes: None,
        children: vec![
            Section::TitleSection {
                children: vec![
                    Chunk::H1 {
                        attributes: Some(HashMap::from([
                            ("id".to_string(), "main".to_string()),
                            ("class".to_string(), "title".to_string()),
                        ])),
                        children: Some(vec![Chunk::Text {
                            value: "Alfa Bravo".to_string(),
                        }]),
                    },
                    Chunk::P {
                        attributes: None,
                        children: Some(vec![Chunk::Text {
                            value: "The quick brown fox\nand the lazy dog".to_string(),
                        }]),
                    },
                ],
            },
            Section::ParagraphSection {
                children: vec![
                    Chunk::P {
                        attributes: None,
                        children: Some(vec![Chunk::Text {
                            value: "They took the axe and the\nsaw to the forest.".to_string(),
                        }]),
                    },
                    Chunk::P {
                        attributes: None,
                        children: Some(vec![Chunk::Text {
                            value: "The bark of the pine tree.".to_string(),
                        }]),
                    },
                ],
            },
            Section::ParagraphSection {
                children: vec![
                    Chunk::P {
                        attributes: Some(HashMap::from([(
                            "class".to_string(),
                            "main".to_string(),
                        )])),
                        children: Some(vec![
                            Chunk::Text {
                                value: "The ".to_string(),
                            },
                            Chunk::InlineCode {
                                attributes: None,
                                language: Some("rust".to_string()),
                                value: Some("chink".to_string()),
                            },
                            Chunk::Text {
                                value: " in the wall.".to_string(),
                            },
                        ]),
                    },
                    Chunk::P {
                        attributes: Some(HashMap::from([(
                            "class".to_string(),
                            "main".to_string(),
                        )])),
                        children: Some(vec![
                            Chunk::Text {
                                value: "The ".to_string(),
                            },
                            Chunk::InlineCode {
                                attributes: None,
                                language: Some("python".to_string()),
                                value: Some("desk".to_string()),
                            },
                            Chunk::Text {
                                value: " and ".to_string(),
                            },
                            Chunk::InlineCode {
                                attributes: None,
                                language: Some("javascript".to_string()),
                                value: Some("both".to_string()),
                            },
                            Chunk::Text {
                                value: " chairs".to_string(),
                            },
                        ]),
                    },
                ],
            },
        ],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

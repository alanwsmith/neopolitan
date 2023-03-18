/////////////////////////////////////////////
// This is the basic integration test
// location. GOAL: It has a few samples
// that demonstrate the core features
// of the format. Detailed variations are
// checked via unit tests.
/////////////////////////////////////////////

#![allow(warnings)]
use crate::chunk::Chunk;
use crate::get_structure::get_structure;
use crate::page::Page;
use crate::section::Section;
use std::collections::HashMap;

#[test]
fn basic_title_and_paragraph() {
    let source = vec![
        "-> TITLE",
        ">> id: main",
        "",
        "Alfa Bravo",
        "",
        "The brown fox\nand the lazy dog",
    ]
    .join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::TitleSection {
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
                        value: "The brown fox\nand the lazy dog".to_string(),
                    }]),
                },
            ],
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn multiple_paragraphs() {
    let source = vec![
        "-> P",
        "",
        "They took the axe and the",
        "saw to the forest.",
        "",
        "The bark of the pine tree.",
        "",
    ]
    .join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::ParagraphSection {
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
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn basic_integration() {
    let source = vec![
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
        children: vec![Section::ParagraphSection {
            children: vec![
                Chunk::P {
                    attributes: Some(HashMap::from([("class".to_string(), "main".to_string())])),
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
                    attributes: Some(HashMap::from([("class".to_string(), "main".to_string())])),
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
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn inline_code_snippets() {
    let source = vec![
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
        children: vec![Section::ParagraphSection {
            children: vec![
                Chunk::P {
                    attributes: Some(HashMap::from([("class".to_string(), "main".to_string())])),
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
                    attributes: Some(HashMap::from([("class".to_string(), "main".to_string())])),
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
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn inline_links() {
    let source = vec![
        "-> P",
        "",
        "Raise the <<link|sail|https://www.example.com/>> and steer",
        "",
        "He <<link|ordered|1>> <<link|peach pie|2>>",
        "",
    ]
    .join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::ParagraphSection {
            children: vec![
                Chunk::P {
                    attributes: None,
                    children: Some(vec![
                        Chunk::Text {
                            value: "Raise the ".to_string(),
                        },
                        Chunk::Link {
                            attributes: None,
                            url: Some("https://www.example.com/".to_string()),
                            value: Some("sail".to_string()),
                        },
                        Chunk::Text {
                            value: " and steer".to_string(),
                        },
                    ]),
                },
                Chunk::P {
                    attributes: None,
                    children: Some(vec![
                        Chunk::Text {
                            value: "He ".to_string(),
                        },
                        Chunk::Link {
                            attributes: None,
                            url: Some("1".to_string()),
                            value: Some("ordered".to_string()),
                        },
                        Chunk::Text {
                            value: " ".to_string(),
                        },
                        Chunk::Link {
                            attributes: None,
                            url: Some("2".to_string()),
                            value: Some("peach pie".to_string()),
                        },
                    ]),
                },
            ],
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn code_block_without_language() {
    let source = vec!["-> CODE", "", "fn main() {", "  let alfa = 1;", "}", ""].join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::CodeSection {
            language: None,
            attributes: None,
            children: vec![Chunk::Text {
                value: "fn main() {\n  let alfa = 1;\n}".to_string(),
            }],
        }],
    };
}

#[test]
fn code_block_with_language() {
    let source = vec![
        "-> CODE",
        ">> rust",
        "",
        "fn main() {",
        "  let bravo = 2;",
        "}",
    ]
    .join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::CodeSection {
            language: Some("rust".to_string()),
            attributes: None,
            children: vec![Chunk::Text {
                value: "fn main() {\n  let bravo = 2;\n}".to_string(),
            }],
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn code_block_with_language_and_attributes() {
    let source = vec![
        "-> CODE",
        ">> rust",
        ">> class: river",
        ">> id: spring",
        "",
        "fn main() {",
        "  let charlie = 3;",
        "}",
    ]
    .join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::CodeSection {
            language: Some("rust".to_string()),
            attributes: Some(vec![
                (Some("class".to_string()), Some("river".to_string())),
                (Some("id".to_string()), Some("spring".to_string())),
            ]),
            children: vec![Chunk::Text {
                value: "fn main() {\n  let charlie = 3;\n}".to_string(),
            }],
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn basic_note() {
    let source = vec!["-> NOTE", "", "This is a note"].join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::NoteSection {
            attributes: None,
            children: Some(vec![Chunk::P {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    value: "This is a note".to_string(),
                }]),
            }]),
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn note_with_multiple_lines() {
    let source = vec!["-> NOTE", "", "Oak is strong", "", "and also gives shade"].join("\n");
    let expected = Page {
        attributes: None,
        children: vec![Section::NoteSection {
            attributes: None,
            children: Some(vec![
                Chunk::P {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        value: "Oak is strong".to_string(),
                    }]),
                },
                Chunk::P {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        value: "and also gives shade".to_string(),
                    }]),
                },
            ]),
        }],
    };
    let result = get_structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

/////////////////////////////////////////////
// This is the basic integration test
// location. GOAL: It has a few samples
// that demonstrate the core features
// of the format. Detailed variations are
// checked via unit tests.
/////////////////////////////////////////////

use crate::chunk::*;
use crate::section::*;
use crate::structure::*;
use crate::wrapper::*;
use std::collections::HashMap;

#[test]
fn basic_title_and_paragraph() {
    let source = vec!["-> TITLE", "", "Kickoff"].join("\n");
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::TitleSection {
            attributes: None,
            children: Some(vec![Chunk::H1 {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("Kickoff".to_string()),
                }]),
            }]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
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
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::ParagraphSection {
            attributes: None,
            children: Some(vec![
                Chunk::P {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        attributes: None,
                        value: Some("They took the axe and the\nsaw to the forest.".to_string()),
                    }]),
                },
                Chunk::P {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        attributes: None,
                        value: Some("The bark of the pine tree.".to_string()),
                    }]),
                },
            ]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

// #[test]
// fn basic_integration() {
//     let source = vec![
//         "-> P",
//         ">> class: main",
//         "",
//         "The `chink`rust` in the wall.",
//         "",
//         "The `desk`python` and `both`javascript` chairs",
//         "",
//     ]
//     .join("\n");
//     let expected = Some(Wrapper::Post {
//         attributes: None,
//         children: Some(vec![Section::ParagraphSection {
//             attributes: None,
//             children: Some(vec![
//                 Chunk::P {
//                     attributes: Some(vec![(Some("class".to_string()), Some("main".to_string()))]),
//                     children: Some(vec![
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some("The ".to_string()),
//                         },
//                         Chunk::InlineCode {
//                             attributes: Some(vec![(Some("rust".to_string()), None)]),
//                             language: Some("rust".to_string()),
//                             value: Some("chink".to_string()),
//                         },
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some(" in the wall.".to_string()),
//                         },
//                     ]),
//                 },
//                 Chunk::P {
//                     attributes: Some(vec![(Some("class".to_string()), Some("main".to_string()))]),
//                     children: Some(vec![
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some("The ".to_string()),
//                         },
//                         Chunk::InlineCode {
//                             attributes: Some(vec![(Some("python".to_string()), None)]),
//                             language: Some("python".to_string()),
//                             value: Some("desk".to_string()),
//                         },
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some(" and ".to_string()),
//                         },
//                         Chunk::InlineCode {
//                             attributes: Some(vec![(Some("javascript".to_string()), None)]),
//                             language: Some("javascript".to_string()),
//                             value: Some("both".to_string()),
//                         },
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some(" chairs".to_string()),
//                         },
//                     ]),
//                 },
//             ]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

// #[test]
// fn inline_code_snippets() {
//     let source = vec![
//         "-> P",
//         ">> class: main",
//         "",
//         "The `chink`rust` in the wall.",
//         "",
//         "The `desk`python` and `both`javascript` chairs",
//         "",
//     ]
//     .join("\n");
//     let expected = Some(Wrapper::Post {
//         attributes: None,
//         children: Some(vec![Section::ParagraphSection {
//             attributes: None,
//             children: Some(vec![
//                 Chunk::P {
//                     attributes: Some(vec![(Some("class".to_string()), Some("main".to_string()))]),
//                     children: Some(vec![
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some("The ".to_string()),
//                         },
//                         Chunk::InlineCode {
//                             attributes: Some(vec![(Some("rust".to_string()), None)]),
//                             language: Some("rust".to_string()),
//                             value: Some("chink".to_string()),
//                         },
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some(" in the wall.".to_string()),
//                         },
//                     ]),
//                 },
//                 Chunk::P {
//                     attributes: Some(vec![(Some("class".to_string()), Some("main".to_string()))]),
//                     children: Some(vec![
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some("The ".to_string()),
//                         },
//                         Chunk::InlineCode {
//                             attributes: Some(vec![(Some("python".to_string()), None)]),
//                             language: Some("python".to_string()),
//                             value: Some("desk".to_string()),
//                         },
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some(" and ".to_string()),
//                         },
//                         Chunk::InlineCode {
//                             attributes: Some(vec![(Some("javascript".to_string()), None)]),
//                             language: Some("javascript".to_string()),
//                             value: Some("both".to_string()),
//                         },
//                         Chunk::Text {
//                             attributes: None,
//                             value: Some(" chairs".to_string()),
//                         },
//                     ]),
//                 },
//             ]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

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
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::ParagraphSection {
            attributes: None,
            children: Some(vec![
                Chunk::P {
                    attributes: None,
                    children: Some(vec![
                        Chunk::Text {
                            attributes: None,
                            value: Some("Raise the ".to_string()),
                        },
                        Chunk::Link {
                            attributes: None,
                            url: Some("https://www.example.com/".to_string()),
                            value: Some("sail".to_string()),
                        },
                        Chunk::Text {
                            attributes: None,
                            value: Some(" and steer".to_string()),
                        },
                    ]),
                },
                Chunk::P {
                    attributes: None,
                    children: Some(vec![
                        Chunk::Text {
                            attributes: None,
                            value: Some("He ".to_string()),
                        },
                        Chunk::Link {
                            attributes: None,
                            url: Some("1".to_string()),
                            value: Some("ordered".to_string()),
                        },
                        Chunk::Text {
                            attributes: None,
                            value: Some(" ".to_string()),
                        },
                        Chunk::Link {
                            attributes: None,
                            url: Some("2".to_string()),
                            value: Some("peach pie".to_string()),
                        },
                    ]),
                },
            ]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn code_block_without_language() {
    let source = vec!["-> CODE", "", "fn main() {", "  let alfa = 1;", "}", ""].join("\n");
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::CodeSection {
            language: None,
            attributes: None,
            children: Some(vec![Chunk::Text {
                attributes: None,
                value: Some("fn main() {\n  let alfa = 1;\n}".to_string()),
            }]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

// #[test]
// fn code_block_with_language() {
//     let source = vec![
//         "-> CODE",
//         ">> rust",
//         "",
//         "fn main() {",
//         "  let bravo = 2;",
//         "}",
//     ]
//     .join("\n");
//     let expected = Some(Wrapper::Post {
//         attributes: None,
//         children: Some(vec![Section::CodeSection {
//             language: Some("rust".to_string()),
//             attributes: None,
//             children: Some(vec![Chunk::Text {
//                 attributes: None,
//                 value: Some("fn main() {\n  let bravo = 2;\n}".to_string()),
//             }]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

// #[test]
// fn code_block_with_language_and_attributes() {
//     let source = vec![
//         "-> CODE",
//         ">> rust",
//         ">> class: river",
//         ">> id: spring",
//         "",
//         "fn main() {",
//         "  let charlie = 3;",
//         "}",
//     ]
//     .join("\n");
//     let expected = Some(Wrapper::Post {
//         attributes: None,
//         children: Some(vec![Section::CodeSection {
//             language: Some("rust".to_string()),
//             attributes: Some(vec![
//                 (Some("class".to_string()), Some("river".to_string())),
//                 (Some("id".to_string()), Some("spring".to_string())),
//             ]),
//             children: Some(vec![Chunk::Text {
//                 attributes: None,
//                 value: Some("fn main() {\n  let charlie = 3;\n}".to_string()),
//             }]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

#[test]
fn basic_note() {
    let source = vec!["-> NOTE", "", "This is a note"].join("\n");
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::NoteSection {
            attributes: None,
            children: Some(vec![Chunk::P {
                attributes: None,
                children: Some(vec![Chunk::Text {
                    attributes: None,
                    value: Some("This is a note".to_string()),
                }]),
            }]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn note_with_multiple_lines() {
    let source = vec!["-> NOTE", "", "Oak is strong", "", "and also gives shade"].join("\n");
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::NoteSection {
            attributes: None,
            children: Some(vec![
                Chunk::P {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        attributes: None,
                        value: Some("Oak is strong".to_string()),
                    }]),
                },
                Chunk::P {
                    attributes: None,
                    children: Some(vec![Chunk::Text {
                        attributes: None,
                        value: Some("and also gives shade".to_string()),
                    }]),
                },
            ]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

// #[test]
// fn note_with_code_sections() {
//     let source = vec!["-> NOTE", "", "Here is `some code`rust`"].join("\n");
//     let expected = Some(Wrapper::Post {
//         attributes: None,
//         children: Some(vec![Section::NoteSection {
//             attributes: None,
//             children: Some(vec![Chunk::P {
//                 attributes: None,
//                 children: Some(vec![
//                     Chunk::Text {
//         attributes: None,
//                         value: Some("Here is ".to_string()),
//                     },
//                     Chunk::InlineCode {
//                         attributes: Some(vec![(Some("rust".to_string()), None)]),
//                         language: Some("rust".to_string()),
//                         value: Some("some code".to_string()),
//                     },
//                 ]),
//             }]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

// #[test]
// fn note_with_attributes() {
//     let source = vec!["-> note", ">> id: rose", "", "Lift the square"].join("\n");
//     let expected = Some(Wrapper::Post {
//             attributes: None,
//         children: Some(vec![Section::NoteSection {
//             attributes: Some(vec![(Some("id".to_string()), Some("rose".to_string()))]),
//             children: Some(vec![Chunk::P {
//             attributes: None,
//                 children: Some(vec![Chunk::Text {
//                     attributes: None,
//                     value: Some("Lift the square".to_string()),
//                 }]),
//             }]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

#[test]
fn basic_list() {
    let source = vec!["-> list", "", "- The long journey", "", "- A gold ring"].join("\n");
    let expected = Some(Wrapper::Post {
        attributes: None,
        children: Some(vec![Section::ListSection {
            attributes: None,
            children: Some(vec![
                Chunk::ListItem {
                    attributes: None,
                    children: Some(vec![Chunk::P {
                        attributes: None,
                        children: Some(vec![Chunk::Text {
                            attributes: None,
                            value: Some("The long journey".to_string()),
                        }]),
                    }]),
                },
                Chunk::ListItem {
                    attributes: None,
                    children: Some(vec![Chunk::P {
                        attributes: None,
                        children: Some(vec![Chunk::Text {
                            attributes: None,
                            value: Some("A gold ring".to_string()),
                        }]),
                    }]),
                },
            ]),
        }]),
    });
    let result = structure(source.as_str()).unwrap().1;
    assert_eq!(expected, result);
}

// #[test]
// fn list_with_attributes() {
//     let source = vec!["-> list", ">> id: echo", "", "- Draw the chart"].join("\n");
//     let expected = Some(Wrapper::Post {
//              attributes: None,
//         children: Some(vec![Section::ListSection {
//              attributes: None,
//             children: Some(vec![Chunk::ListItem {
//                 attributes: Some(vec![(Some("id".to_string()), Some("echo".to_string()))]),
//                 children: Some(vec![Chunk::P {
//              attributes: None,
//                     children: Some(vec![Chunk::Text {
//                         attributes: None,
//                         value: Some("Draw the chart".to_string()),
//                     }]),
//                 }]),
//             }]),
//         }]),
//     });
//     let result = structure(source.as_str()).unwrap().1;
//     assert_eq!(expected, result);
// }

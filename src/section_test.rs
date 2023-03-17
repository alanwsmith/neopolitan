use crate::chunk::Chunk;
use crate::section::*;
use std::collections::HashMap;

// TODO: Add test with spaces after the
// title and then on the empty line
// and then in both places.

#[test]
fn section_test_001() {
    let source = vec!["-> TITLE", "", "Alfa Bravo"].join("\n");
    let expected = Section::TITLE {
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
fn section_test_002() {
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
    let expected = Section::TITLE {
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
fn section_test_003() {
    let source = vec!["-> TITLE", ">> id: main", "", "Alfa Bravo"].join("\n");
    let expected = Section::TITLE {
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

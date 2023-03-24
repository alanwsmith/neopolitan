#![allow(warnings)]
use crate::chunk::Chunk;
use crate::code::*;
use crate::section::Section;
use std::collections::HashMap;

// NOTE: all the code that comes in
// at this point will start with
// two `\n\n`` if there isn't an
// attribute

#[test]
fn code_with_no_language_or_attributes() {
    let source = "\n\nOn the islands";
    let expected = Section::CodeSection {
        attributes: None,
        language: None,
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some("On the islands".to_string()),
        }]),
    };
    let result = code(source);
    assert_eq!(expected, result.unwrap().1);
}

#[test]
fn code_with_language_but_no_attributes() {
    let source = ">> rust\n\nBring your best compass";
    let expected = Section::CodeSection {
        attributes: Some(HashMap::from([("rust".to_string(), None)])),
        language: Some("rust".to_string()),
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some("Bring your best compass".to_string()),
        }]),
    };
    let result = code(source);
    assert_eq!(expected, result.unwrap().1);
}

#[test]
fn code_with_language_and_attributes() {
    let source = ">> rust\n>> fence: stone\n>> air: frosty\n\nTwo blue fish";
    let expected = Section::CodeSection {
        attributes: Some(HashMap::from([
            ("rust".to_string(), None),
            ("fence".to_string(), Some("stone".to_string())),
            ("air".to_string(), Some("frosty".to_string())),
        ])),
        language: Some("rust".to_string()),
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some("Two blue fish".to_string()),
        }]),
    };
    let result = code(source);
    assert_eq!(expected, result.unwrap().1);
}

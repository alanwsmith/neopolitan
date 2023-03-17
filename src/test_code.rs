#![allow(warnings)]
use crate::chunk::Chunk;
use crate::code::*;
use crate::section::Section;

#[test]
fn basic_code() {
    let source = "On the islands";
    let expected = Section::CodeSection {
        attributes: None,
        language: None,
        children: vec![Chunk::Text {
            value: "On the islands".to_string(),
        }],
    };
    let result = code(source);
    assert_eq!(expected, result.unwrap().1);
}

#[test]
fn code_with_language() {
    let source = ">> rust\n\nBring your best compass";
    let expected = Section::CodeSection {
        attributes: None,
        language: Some("rust".to_string()),
        children: vec![Chunk::Text {
            value: "Bring your best compass".to_string(),
        }],
    };
    let result = code(source);
    assert_eq!(expected, result.unwrap().1);
}

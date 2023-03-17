#![allow(warnings)]
use crate::chunk::*;
use crate::code::*;
use crate::section::*;
// use std::collections::HashMap;

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

use crate::chunk::Chunk;
use crate::process_text::process_text;
// use crate::section::*;
// use std::collections::HashMap;

#[test]
fn process_text_test_001() {
    let source = "beside the shore";
    let expected: Vec<Chunk> = vec![Chunk::Text {
        value: "Open the crate".to_string(),
    }];

    let result = process_text(source).unwrap().1;
    assert_eq!(expected, result);
}

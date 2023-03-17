use crate::chunk::Chunk;
use crate::process_text::*;

#[test]
fn process_text_test_001() {
    let source = "beside the shore";
    let expected: Vec<Chunk> = vec![Chunk::Text {
        value: "beside the shore".to_string(),
    }];
    let result = process_text_dev(source).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn process_text_test_002() {
    let source = "The `frosty`rust` air";
    let expected_result: Vec<Chunk> = vec![
        Chunk::Text {
            value: "The ".to_string(),
        },
        Chunk::InlineCode {
            attributes: None,
            language: Some("rust".to_string()),
            value: Some("frosty".to_string()),
        },
    ];
    let expected_remainder = " air";
    let (remainder, result) = process_text_dev(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

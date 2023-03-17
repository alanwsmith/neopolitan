use crate::chunk::Chunk;
use crate::process_text::*;

#[test]
fn basic_text() {
    let source = "beside the shore";
    let expected: Vec<Chunk> = vec![Chunk::Text {
        value: "beside the shore".to_string(),
    }];
    let result = process_text(source).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn one_inline_code_snippet() {
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
        Chunk::Text {
            value: " air".to_string(),
        },
    ];
    let expected_remainder = "";
    let (remainder, result) = process_text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

#[test]
fn two_inline_code_snippets() {
    let source = "A `castle`python` built `from`javascript` sand";
    let expected_result: Vec<Chunk> = vec![
        Chunk::Text {
            value: "A ".to_string(),
        },
        Chunk::InlineCode {
            attributes: None,
            language: Some("python".to_string()),
            value: Some("castle".to_string()),
        },
        Chunk::Text {
            value: " built ".to_string(),
        },
        Chunk::InlineCode {
            attributes: None,
            language: Some("javascript".to_string()),
            value: Some("from".to_string()),
        },
        Chunk::Text {
            value: " sand".to_string(),
        },
    ];
    let expected_remainder = "";
    let (remainder, result) = process_text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

#[test]
fn single_link() {
    let source = "The <<link|paper|https://paper.example.com/>> box";
    let expected_result: Vec<Chunk> = vec![
        Chunk::Text {
            value: "The ".to_string(),
        },
        Chunk::Link {
            attributes: None,
            url: Some("https://paper.example.com/".to_string()),
            value: Some("paper".to_string()),
        },
        Chunk::Text {
            value: " box".to_string(),
        },
    ];
    let expected_remainder = "";
    let (remainder, result) = process_text_dev(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

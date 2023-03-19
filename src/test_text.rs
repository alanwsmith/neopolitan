use crate::chunk::Chunk;
use crate::text::*;

#[test]
fn basic_text() {
    let source = "beside the shore";
    let expected: Option<Vec<Chunk>> = Some(vec![Chunk::Text {
        value: "beside the shore".to_string(),
    }]);
    let result = text(source).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn one_inline_code_snippet() {
    let source = "The `frosty`rust` air";
    let expected_result: Option<Vec<Chunk>> = Some(vec![
        Chunk::Text {
            value: "The ".to_string(),
        },
        Chunk::InlineCode {
            attributes: Some(vec![(Some("rust".to_string()), None)]),
            language: Some("rust".to_string()),
            value: Some("frosty".to_string()),
        },
        Chunk::Text {
            value: " air".to_string(),
        },
    ]);
    let expected_remainder = "";
    let (remainder, result) = text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

#[test]
fn code_with_attributes() {
    let source = "The `frosty`rust|class: sail` air";
    let expected_result: Option<Vec<Chunk>> = Some(vec![
        Chunk::Text {
            value: "The ".to_string(),
        },
        Chunk::InlineCode {
            attributes: Some(vec![
                (Some("rust".to_string()), None),
                (Some("class".to_string()), Some("sail".to_string())),
            ]),
            language: Some("rust".to_string()),
            value: Some("frosty".to_string()),
        },
        Chunk::Text {
            value: " air".to_string(),
        },
    ]);
    let expected_remainder = "";
    let (remainder, result) = text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

#[test]
fn two_inline_code_snippets() {
    let source = "A `castle`python` built `from`javascript` sand";
    let expected_result: Option<Vec<Chunk>> = Some(vec![
        Chunk::Text {
            value: "A ".to_string(),
        },
        Chunk::InlineCode {
            attributes: Some(vec![(Some("python".to_string()), None)]),
            language: Some("python".to_string()),
            value: Some("castle".to_string()),
        },
        Chunk::Text {
            value: " built ".to_string(),
        },
        Chunk::InlineCode {
            attributes: Some(vec![(Some("javascript".to_string()), None)]),
            language: Some("javascript".to_string()),
            value: Some("from".to_string()),
        },
        Chunk::Text {
            value: " sand".to_string(),
        },
    ]);
    let expected_remainder = "";
    let (remainder, result) = text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

// #[test]
// fn single_link_with_attributes() {
//     let source = "The <<link|paper|https://paper.example.com/|>> box";
//     let expected_result: Option<Vec<Chunk>> = Some(vec![
//         Chunk::Text {
//             value: "The ".to_string(),
//         },
//         Chunk::Link {
//             attributes: None,
//             url: Some("https://paper.example.com/".to_string()),
//             value: Some("paper".to_string()),
//         },
//         Chunk::Text {
//             value: " box".to_string(),
//         },
//     ]);
//     let expected_remainder = "";
//     let (remainder, result) = text(source).unwrap();
//     assert_eq!(expected_result, result);
//     assert_eq!(expected_remainder, remainder);
// }

#[test]
fn single_link() {
    let source = "The <<link|paper|https://paper.example.com/>> box";
    let expected_result: Option<Vec<Chunk>> = Some(vec![
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
    ]);
    let expected_remainder = "";
    let (remainder, result) = text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

#[test]
fn multiple_links() {
    let source = "In a <<link|high|alfa>> <<link|wind|bravo>>";
    let expected_result: Option<Vec<Chunk>> = Some(vec![
        Chunk::Text {
            value: "In a ".to_string(),
        },
        Chunk::Link {
            attributes: None,
            url: Some("alfa".to_string()),
            value: Some("high".to_string()),
        },
        Chunk::Text {
            value: " ".to_string(),
        },
        Chunk::Link {
            attributes: None,
            url: Some("bravo".to_string()),
            value: Some("wind".to_string()),
        },
    ]);
    let expected_remainder = "";
    let (remainder, result) = text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

#[test]
fn single_strong() {
    let source = "The *dune** rose";
    let expected_result: Option<Vec<Chunk>> = Some(vec![
        Chunk::Text {
            value: "The ".to_string(),
        },
        Chunk::Strong {
            attributes: None,
            value: Some("dune".to_string()),
        },
        Chunk::Text {
            value: " rose".to_string(),
        },
    ]);
    let expected_remainder = "";
    let (remainder, result) = text(source).unwrap();
    assert_eq!(expected_result, result);
    assert_eq!(expected_remainder, remainder);
}

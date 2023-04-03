use crate::block::block::Block;
use crate::content::content::Content;
use crate::section::section::Section;
use crate::section::title::*;

#[test]
fn to_paragraphs() {
    let source = "\nHello, World\n\nquick brown fox";
    let expected = Ok((
        "",
        Section::Title {
            attributes: None,
            children: Some(vec![
                Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("Hello, World".to_string()),
                    }]),
                },
                Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("quick brown fox".to_string()),
                    }]),
                },
            ]),
        },
    ));
    let result = title(source);
    assert_eq!(expected, result);
}

#[test]
fn single_line() {
    let source = "\nHello, World\n\n-> p\n\nmore content";
    let expected = Ok((
        "-> p\n\nmore content",
        Section::Title {
            attributes: None,
            children: Some(vec![Block::P {
                attributes: None,
                children: Some(vec![Content::Text {
                    text: Some("Hello, World".to_string()),
                }]),
            }]),
        },
    ));
    let result = title(source);
    assert_eq!(expected, result);
}

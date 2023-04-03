use crate::block::block::*;
use crate::content::content::*;
use crate::section::section::*;
use crate::wrapper::*;

// This is the primary integration test file

#[test]
fn hello_world() {
    let lines = vec!["-> title", "", "Hello, Neopolitan"].join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        Wrapper::Page {
            children: Some(vec![Section::Title {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("Hello, Neopolitan".to_string()),
                    }]),
                }]),
            }]),
        },
    ));
    let result = wrapper(source);
    assert_eq!(expected, result);
}

#[test]
fn p_section() {
    let lines = vec!["-> p", "", "The blue sky"].join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        Wrapper::Page {
            children: Some(vec![Section::P {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("The blue sky".to_string()),
                    }]),
                }]),
            }]),
        },
    ));
    let result = wrapper(source);
    assert_eq!(expected, result);
}

// The colt reared and threw the tall rider.
// It snowed, rained, and hailed the same morning.
// Take the winding path to reach the lake.
// The wide road shimmered in the hot sun.
// Lift the square stone over the fence.

#[test]
fn title_and_paragraphs() {
    let lines = vec![
        "-> title",
        "",
        "The Colt Reared",
        "",
        "-> p",
        "",
        "The road shimmered",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Ok((
        "",
        Wrapper::Page {
            children: Some(vec![
                Section::Title {
                    attributes: None,
                    children: Some(vec![Block::P {
                        attributes: None,
                        children: Some(vec![Content::Text {
                            text: Some("The Colt Reared".to_string()),
                        }]),
                    }]),
                },
                Section::P {
                    attributes: None,
                    children: Some(vec![Block::P {
                        attributes: None,
                        children: Some(vec![Content::Text {
                            text: Some("The road shimmered".to_string()),
                        }]),
                    }]),
                },
            ]),
        },
    ));
    let result = wrapper(source);
    assert_eq!(expected, result);
}

use crate::block::*;
use crate::content::*;
use crate::section::*;
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

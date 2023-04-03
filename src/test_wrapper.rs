// This is the primary integration test file
// The colt reared and threw the tall rider.
// It snowed, rained, and hailed the same morning.
// Take the winding path to reach the lake.
// The wide road shimmered in the hot sun.
// Lift the square stone over the fence.

use crate::block::block::Block;
use crate::content::content::Content;
use crate::section::section::Section;
use crate::wrapper::*;

#[test]
fn integration_alfa() {
    let lines = vec![
        "-> title",
        "",
        "The Colt Reared",
        "",
        "-> p",
        "",
        "The road shimmered",
        "in the rain",
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
                            text: Some("The road shimmered\nin the rain".to_string()),
                        }]),
                    }]),
                },
            ]),
        },
    ));
    let result = wrapper(source);
    assert_eq!(expected, result);
}

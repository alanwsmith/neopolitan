use crate::block::block::*;
use crate::content::content::*;
use crate::section::section::*;

#[test]
fn test1() {
    let source = "-> title\n\nHere it is";
    let expected = Section::Title {
        attributes: None,
        children: Some(vec![Block::P {
            attributes: None,
            children: Some(vec![Content::Text {
                text: Some("Here it is".to_string()),
            }]),
        }]),
    };
    let (_, result) = section(source).unwrap();
    assert_eq!(expected, result);
}

#[test]
fn blurb_response() {
    let lines = vec!["-> blurb", "", "walking on sunshine"].join("\n");
    let source = lines.as_str();
    let expected = Section::Blurb {
        attributes: None,
        children: Some(vec![Block::P {
            attributes: None,
            children: Some(vec![Content::Text {
                text: Some("walking on sunshine".to_string()),
            }]),
        }]),
    };
    let (_, result) = section(source).unwrap();
    assert_eq!(expected, result);
}

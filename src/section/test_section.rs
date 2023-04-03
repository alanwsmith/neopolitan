use crate::block::block::*;
use crate::content::content::*;
use crate::section::section::*;

// The tree top waved in a graceful way.
// The spot on the blotter was made by green ink.
// The cigar burned a hole in the desk top.
// He broke a new shoelace that day.
// The coffee stand is too high for the couch.
// The urge to write short stories is rare.
// The pencils have all been used.
// The sofa cushion is red and of light weight.
// The jacket hung on the back of the wide chair.
// At that high level the air is pure.
// The office paint was a dull sad tan.
// Steam hissed from the broken valve.
// There was a sound of dry leaves outside.
// Torn scraps littered the stone floor.

#[test]
fn blurb_test() {
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

#[test]
fn list_test() {
    let lines = vec![
        "-> list",
        "",
        "- These thistles bend",
        "",
        "- in a high wind",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Section::List {
        attributes: None,
        children: Some(vec![
            Block::ListItem {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("These thistles bend".to_string()),
                    }]),
                }]),
            },
            Block::ListItem {
                attributes: None,
                children: Some(vec![Block::P {
                    attributes: None,
                    children: Some(vec![Content::Text {
                        text: Some("in a high wind".to_string()),
                    }]),
                }]),
            },
        ]),
    };
    let (_, result) = section(source).unwrap();
    assert_eq!(expected, result);
}

#[test]
fn title_test() {
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

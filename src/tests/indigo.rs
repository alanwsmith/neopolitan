use crate::block::block::*;
// use crate::content::content::*;
// use crate::attribute::*;
// use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
// use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;

#[test]
fn indigo() {
    let lines = vec!["-> note", "", "This is a note", "with a second line"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::NoteSection {
            attributes: None,
            children: Some(vec![Block::P {
                children: Some(vec![
                    Content::Text {
                        text: Some("This".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("is".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("a".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("note".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("with".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("a".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("second".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("line".to_string()),
                    },
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

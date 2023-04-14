// use crate::block::block::*;
// use crate::content::content::*;
// use crate::attribute::*;
use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[test]
fn november() {
    let lines = vec!["-> comment", "", "comment test"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::CommentSection {
            attributes: None,
            children: Some(vec![Block::P {
                children: Some(vec![
                    Content::Text {
                        text: Some("comment".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("test".to_string()),
                    },
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

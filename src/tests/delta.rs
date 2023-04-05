// use crate::block::block::*;
// use crate::content::content::*;
use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[test]
fn delta() {
    let lines = vec![
        "-> p",
        "",
        "alfa <<b|bravo>> charlie",
        "<<b|delta>> <<b|echo>> <<b|foxtrot>>",
    ]
    .join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::Paragraphs {
            attributes: None,
            children: Some(vec![Block::P {
                children: Some(vec![
                    Content::Text {
                        text: Some("alfa".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("bravo".to_string()),
                    },
                    Content::Space,
                    Content::Text {
                        text: Some("charlie".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("delta".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("echo".to_string()),
                    },
                    Content::Space,
                    Content::B {
                        attributes: None,
                        text: Some("foxtrot".to_string()),
                    },
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

// use crate::block::block::*;
// use crate::content::content::*;
use crate::attribute::*;
use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[test]
fn echo() {
    let lines = vec!["-> p", "", "`tango uniform``", "`alfa`rust`"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::Paragraphs {
            attributes: None,
            children: Some(vec![Block::P {
                children: Some(vec![
                    Content::CodeShorthand {
                        attributes: None,
                        text: Some("tango uniform".to_string()),
                    },
                    Content::Space,
                    Content::CodeShorthand {
                        attributes: Some(vec![Attribute::Basic {
                            key: Some("rust".to_string()),
                            value: None,
                        }]),
                        text: Some("alfa".to_string()),
                    },
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

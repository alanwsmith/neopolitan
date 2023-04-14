use crate::block::block::*;
use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[test]
fn golf() {
    let lines = vec!["-> subtitle", "", "this is a subtitle"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::Subtitle {
            attributes: None,
            children: Some(vec![Block::P {
                children: Some(vec![
                    Content::Text {
                        text: Some("this".to_string()),
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
                        text: Some("subtitle".to_string()),
                    },
                ]),
            }]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

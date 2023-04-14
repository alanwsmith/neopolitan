use crate::block::block::*;
// use crate::content::content::*;
// use crate::attribute::*;
// use crate::block::block::*;
// use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::section::attributes_for_section::*;
use crate::wrapper::wrapper::*;

#[test]
fn foxtrot() {
    let lines = vec!["-> code", "", "some code", "more code"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::CodeSection {
            attributes: None,
            children: Some(Block::CodeBlock {
                text: Some("some code\nmore code".to_string()),
            }),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

#[test]
fn foxtrot2_attributes() {
    let lines = vec!["-> code", ">> language: rust", "", "some code", "more code"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::CodeSection {
            attributes: Some(vec![SectionAttribute::Attribute {
                key: Some("language".to_string()),
                value: Some("rust".to_string()),
            }]),
            children: Some(Block::CodeBlock {
                text: Some("some code\nmore code".to_string()),
            }),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

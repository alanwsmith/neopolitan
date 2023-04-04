// use crate::block::block::*;
// use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;

#[test]
fn bravo() {
    let lines = vec!["-> title", ">> id: bravo", ""].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::Title {
            attributes: Some(vec![SectionAttribute::Attribute {
                key: Some("id".to_string()),
                value: Some("bravo".to_string()),
            }]),
            children: None,
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

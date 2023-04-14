// use crate::block::block::*;
// use crate::content::content::*;
// use crate::attribute::*;
// use crate::block::block::*;
// use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::section::attributes_for_section::*;
use crate::wrapper::wrapper::*;

#[test]
fn oscar() {
    let lines = vec!["-> attributes", ">> somekey: some value"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::AttributesSection {
            attributes: Some(vec![
                (SectionAttribute::Attribute {
                    key: Some("somekey".to_string()),
                    value: Some("some value".to_string()),
                }),
            ]),
            children: None,
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

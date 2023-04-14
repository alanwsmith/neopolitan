// use crate::block::block::*;
// use crate::content::content::*;
// use crate::attribute::*;
// use crate::block::block::*;
// use crate::content::content::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::section::section_attributes::*;
use crate::wrapper::wrapper::*;

#[test]
fn lima() {
    let lines = vec!["-> youtube", ">> asdf1234"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::YouTubeSection {
            attributes: Some(vec![
                (SectionAttribute::Attribute {
                    key: Some("asdf1234".to_string()),
                    value: None,
                }),
            ]),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

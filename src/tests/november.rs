use crate::block::block::*;
use crate::parse::parse;
use crate::section::section::*;
use crate::wrapper::wrapper::*;

#[test]
fn november() {
    let lines = vec!["-> comment", "", "comment text"].join("\n");
    let source = lines.as_str();
    let expected = Wrapper::Page {
        children: Some(vec![Section::CommentSection {
            attributes: None,
            children: Some(Block::RawContent {
                text: Some("comment text".to_string())
            }),
        }]),
    };
    let result = parse(source).unwrap().1;
    assert_eq!(expected, result);
}

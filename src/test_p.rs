use crate::block::Block;
use crate::content::Content;
use crate::p::p;

#[test]
fn basic() {
    let source = "alfa bravo\n\n";
    let expected = Ok((
        "",
        Block::P {
            attributes: None,
            children: Some(vec![Content::Text {
                text: Some("alfa bravo".to_string()),
            }]),
        },
    ));
    let result = p(source);
    assert_eq!(expected, result);
}

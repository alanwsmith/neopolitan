use crate::content::content::*;
use crate::content::link::link;

#[test]
fn link_basic() {
    let source = ("<<link|", "localhost", "|", "alfa", ">>");
    let expected = Ok((
        "",
        Content::Link {
            attributes: None,
            url: Some("localhost".to_string()),
            text: Some("alfa".to_string()),
        },
    ));
    let result = link(source);
    assert_eq!(expected, result);
}

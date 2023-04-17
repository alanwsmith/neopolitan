use crate::attribute::*;
use crate::content::content::*;
use crate::content::link::*;


#[ignore]
#[test]
fn link_basic() {
    let source = ("<<link|", "alfa", "|", "localhost", ">>");
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


#[ignore]
#[test]
fn link_with_attributes() {
    let source = ("<<link|", "alfa", "|", "localhost|class: important", ">>");
    let expected = Ok((
        "",
        Content::Link {
            attributes: Some(vec![Attribute::Basic {
                key: Some("class".to_string()),
                value: Some("important".to_string()),
            }]),
            url: Some("localhost".to_string()),
            text: Some("alfa".to_string()),
        },
    ));
    let result = link(source);
    assert_eq!(expected, result);
}

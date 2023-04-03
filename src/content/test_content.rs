use crate::content::content::*;

#[test]
fn basic_content() {
    let source = "alfa bravo";
    let expected = Ok((
        "",
        vec![Content::Text {
            text: Some("alfa bravo".to_string()),
        }],
    ));
    let result = content(source);
    assert_eq!(expected, result);
}

#[test]
fn test_link() {
    let source = "<<link|https://bravo.example.com/|bravo link>>";
    let expected = Ok((
        "",
        vec![Content::Link {
            source: None,
            attributes: None,
            url: Some("https://bravo.example.com/".to_string()),
            text: Some("bravo link".to_string()),
        }],
    ));
    let result = content(source);
    assert_eq!(expected, result);
}

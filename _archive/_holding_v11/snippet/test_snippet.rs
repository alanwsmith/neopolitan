use crate::snippet::snippet::snippet;
use crate::snippet::snippet_enum::Snippet;

#[test]
pub fn basic_link_test() {
    let expected = Ok((
        " with more words",
        Snippet::LinkTag {
            string: Some(r#"<a href="example.com">Alfa Test</a>"#.to_string()),
        },
    ));
    let result = snippet(" <<Alfa Test|link|example.com>> with more words");
    assert_eq!(expected, result);
}

#[test]
pub fn basic_link_with_escaped_character() {
    let expected = Ok((
        " with more words",
        Snippet::LinkTag {
            string: Some(r#"<a href="example.com">Alfa | Test</a>"#.to_string()),
        },
    ));
    let result = snippet(" <<Alfa \\| Test|link|example.com>> with more words");
    assert_eq!(expected, result);
}


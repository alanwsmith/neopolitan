use crate::page_builder::PageBuilder;

#[test]
fn basic_content_test() {
    let pb = PageBuilder::new();
    let source = "alfa line is here".to_string();
    let expected: Vec<String> = vec![
        "<p>alfa line is here</p>".to_string(),
    ];
    assert_eq!(expected, pb.content(source));
}

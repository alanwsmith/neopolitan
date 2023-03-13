use crate::page_builder::PageBuilder;

#[test]
fn test_title_block() {
    let pb = PageBuilder::new();
    let input = "Test Title Text";
    let expected =
        String::from("<h1>Test Title Text</h1>");
    let result = pb.title_from(input);
    assert_eq!(expected, result);
}

use crate::page_builder::PageBuilder;

#[test]
fn test_split() {
    // GIVEN
    let source = r#"-> title

This is the title

-> c

This is some content

With a couple of lines

-> h2 

And an h2

-> c

With some more content

"#;

    // WHEN
    let pb = PageBuilder::new();

    // THEN
    let expected = vec![
        ("title".to_string(), "This is the title".to_string()),
        ("c".to_string(), "This is some content\n\nWith a couple of lines".to_string()),
        ("h2".to_string(), "And an h2".to_string()),
        ("c".to_string(), "With some more content".to_string())
    ];

    let result = pb.split(source);

    assert_eq!(expected, result);
}

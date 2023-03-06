use crate::page_builder::PageBuilder;

#[test]
fn test_blocks() {
    // WHEN
    let mut pb = PageBuilder::new();

    pb.input = Some(String::from(
        r#"-> title

This is the title

-> c

This is some content

With a couple of lines

-> h2 

And an h2

-> c

With some more content

"#,
    ));

    // THEN
    let expected = vec![
        ("title".to_string(), "This is the title".to_string()),
        ("c".to_string(), "This is some content\n\nWith a couple of lines".to_string()),
        ("h2".to_string(), "And an h2".to_string()),
        ("c".to_string(), "With some more content".to_string())
    ];

    let result = pb.blocks();

    assert_eq!(expected, result);
}

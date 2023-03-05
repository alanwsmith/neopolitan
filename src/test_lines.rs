use crate::page_builder::PageBuilder;

#[test]
fn test_output_basic() {
    // GIVEN
    let source = r#"-> title

Welcome To Neopolitan

-> c

This is the first test.

With a couple of paragraphs.

-> h3

And another header

-> c

And some more content

That ends here
"#
    .to_string();

    let expected: Vec<String> = vec![
        "<h1>Welcome To Neopolitan</h1>"
            .to_string(),
        "<p>This is the first test.</p>"
            .to_string(),
        "<p>With a couple of paragraphs.</p>"
            .to_string(),
        "<h3>And another header</h3>".to_string(),
        "<p>And some more content</p>".to_string(),
        "<p>The ends here</p>".to_string(),
    ];

    let pb = PageBuilder::new();

    let lines = pb.lines(source);
    assert_eq!(expected, lines);
}

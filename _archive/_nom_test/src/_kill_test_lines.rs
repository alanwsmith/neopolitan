use crate::page_builder::PageBuilder;

// This returns the vec of lines that will
// be assembed into the final page. It
// takes the vec of raw lines produced
// by .split()

#[test]
fn test_lines_basic() {
    // GIVEN

    let source_lines = vec![
        (
            "title".to_string(),
            "Welcome To Neopolitan".to_string(),
        ),
        (
            "c".to_string(),
            "This is the first test.".to_string(),
        ),
        (
            "h2".to_string(),
            "And another header".to_string(),
        ),
        (
            "c".to_string(),
            "And some more content".to_string(),
        ),
    ];

    let expected: Vec<String> = vec![
        "<h1>Welcome To Neopolitan</h1>"
            .to_string(),
        "<p>This is the first test.</p>"
            .to_string(),
        "<h2>And another header</h2>".to_string(),
        "<p>And some more content</p>".to_string(),
    ];

    let pb = PageBuilder::new();

    let lines = pb.lines(source_lines);
    assert_eq!(expected, lines);
}

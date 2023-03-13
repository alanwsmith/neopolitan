#[test]
fn test_find_tag() {
    let input = r#"-> title

This is the title

-> c

This is some content"#;

    let b = Builder::new(input.to_string());

    let expected = (
        "title",
        r#"-> c

This is some content"#,
    );

    assert_eq!(expected, b.find_next_tag());
}

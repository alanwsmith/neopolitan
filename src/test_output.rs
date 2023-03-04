#![allow(warnings)]
use crate::builder::Builder;
use crate::output;

#[test]
fn basic_build() {
    // GIVEN
    let b = Builder::new(
        r#"-> title

Welcome To Neopolitan"#
            .to_string(),
    );

    let expected =
        r#"<h1>Welcome To Neopolitan</h1>"#
            .to_string();
    assert_eq!(expected, b.output());
}

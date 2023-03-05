#![allow(warnings)]
use crate::builder::Builder;
use crate::output;
use std::include_str;

#[test]
fn basic_headline() {
    let input =
        include_str!("../test_sets/full/1/in.neo");

    let b = Builder::new(input.to_string());

    let expected = include_str!(
        "../test_sets/full/1/target.html"
    )
    .to_string();

    assert_eq!(expected, b.output());
}

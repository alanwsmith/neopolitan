#![allow(warnings)]
use crate::builder::Builder;
use crate::output;
use std::include_str;
use std::path::PathBuf;

// #[test]
// fn test_title() {
//     let input =
//         include_str!("../test_sets/full/1/in.neo");
//     let b = Builder::new(input.to_string());
//     let expected = include_str!(
//         "../test_sets/full/1/target.html"
//     )
//     .to_string();
//     assert_eq!(expected, b.output(
//         PathBuf::from("/Users/alan/workshop/neopolitan/test_sets/full/1/post.html")
//     ));
// }

// #[test]
// fn test_first_paragraph() {
//     let input =
//         include_str!("../test_sets/full/2/in.neo");
//     let b = Builder::new(input.to_string());
//     let expected = include_str!(
//         "../test_sets/full/2/target.html"
//     )
//     .to_string();
//     assert_eq!(
//         expected, b.output_dev(
//         PathBuf::from("/Users/alan/workshop/neopolitan/test_sets/full/2/post.html")
//     ).ok().unwrap().0);
// }

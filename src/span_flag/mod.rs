#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
use crate::span::Span;
use crate::span_metadata::RawShorthandMetadata;
use crate::span_metadata::RawShorthandMetadataDev;
use crate::span_strings::escaped_character::escaped_character;
use crate::span_strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_strings::plain_text_string_base::plain_text_string_base;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

pub fn span_flag<'a>(
    source: &'a str,
) -> IResult<&'a str, RawShorthandMetadataDev> {
    let (source, _) =
        (tag("|"), space0, opt(line_ending), space0).parse(source)?;
    let (source, spans) =
        many1(alt((plain_text_string_base,))).parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, _) = opt(line_ending).parse(source)?;
    let (source, _) = space0.parse(source)?;
    Ok((source, RawShorthandMetadataDev::Flag("alfa".to_string())))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // #[rstest]
    // #[case("|alfa", "alfa", "")]
    // fn span_flag_valid_tests(
    //     #[case] source: &str,
    //     #[case] found: &str,
    //     #[case] remainder: &str,
    // ) {
    //     let left = RawShorthandMetadataDev::Flag("alfa".to_string());
    //     let right = span_flag(source).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

    //
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use pretty_assertions::assert_eq;
//     use rstest::rstest;
//     #[rstest]
//     #[case("|alfa", "alfa", "")]
//     #[case("|alfa|", "alfa", "|")]
//     #[case("| alfa", "alfa", "")]
//     #[case("| alfa |", "alfa", "|")]
//     #[case("| alfa-bravo", "alfa-bravo", "")]
//     #[case("|http://www.example.com/", "http://www.example.com/", "")]
//     #[case("|alfa`", "alfa`", "")]
//     #[case("|alfa``", "alfa", "``")]
//     #[case("|alfa\\``", "alfa``", "")]
//     #[case("|alfa::bravo", "alfa::bravo", "")]
//     #[case("|alfa[[bravo]]", "alfa[[bravo]]", "")]
//     #[case("|\nalfa|", "alfa", "|")]
//     #[case("|alfa\n|", "alfa", "|")]
//     #[case("| \n alfa \n |", "alfa", "|")]
//     #[case("|alfa\\|", "alfa|", "")]
//     fn span_flag_valid_tests(
//         #[case] source: &str,
//         #[case] found: &str,
//         #[case] remainder: &str,
//     ) {
//         let characters = "%@~*^![]{}<>_#:".to_string();
//         let left = RawShorthandMetadata::Flag(found.to_string());
//         let right = span_flag(source, characters).unwrap();
//         assert_eq!(left, right.1);
//         assert_eq!(remainder, right.0);
//     }
//     #[rstest]
//     #[case("alfa")]
//     fn span_flag_invalid_tests(#[case] source: &str) {
//         let characters = "%@~*^![]{}<>_#:".to_string();
//         let result = span_flag(source, characters);
//         match result {
//             Ok(_) => assert!(false),
//             Err(_) => assert!(true),
//         }
//     }
// }

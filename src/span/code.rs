#![allow(unused)]
use super::escaped::escaped_span;
use crate::neo_config::NeoConfig;
use crate::section_attr::raw_section_attr;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
use crate::span::Span;
use crate::span_metadata::span_metadata;
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

pub fn code_span<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, spans) = preceded(
        pair(tag("``"), opt(plain_text_space1_as_single_space)),
        many1(alt((escaped_span, code_span_text))),
    )
    .parse(source)?;
    let (source, (flags, attrs)) = span_metadata(source, "`")?;
    let (source, _) = tag("``").parse(source)?;
    Ok((
        source,
        Span::Code {
            attrs,
            flags,
            spans,
        },
    ))
}

pub fn code_span_text<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, parts) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        is_a("%@~*^![]{}<>_#:"),
        plain_text_single_line_ending_as_space,
        escaped_character,
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: parts.join(""),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    // #[rstest]
    // #[case("``alfa``", Span::Code{
    //     attrs: BTreeMap::new(),
    //     flags: vec![],
    //     spans: vec![
    //         Span::Text{content: "alfa".to_string()}
    //     ],
    // }, "")]
    // #[case("``alfa|bravo``", Span::Code{
    //     attrs: BTreeMap::new(),
    //     flags: vec!["bravo".to_string()],
    //     spans: vec![
    //         Span::Text{content: "alfa".to_string()}
    //     ],
    // }, "")]
    // fn code_span_valid_tests(
    //     #[case] source: &str,
    //     #[case] left: Span,
    //     #[case] remainder: &str,
    // ) {
    //     let right = code_span(source).unwrap();
    //     assert_eq!(left, right.1);
    //     assert_eq!(remainder, right.0);
    // }

    //
}

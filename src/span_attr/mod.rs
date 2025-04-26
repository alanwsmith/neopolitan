#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
use crate::span::Span;
use crate::span::code::code_span;
use crate::span::text::text_span;
use crate::span_metadata::RawSpanMetadata;
use crate::span_strings::escaped_character::escaped_character;
use crate::span_strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_strings::plain_text_string_base::plain_text_string_base;
use crate::span_strings::single_character::single_character;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::character::complete::line_ending;
use nom::character::complete::one_of;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::not;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

// TODO: Verify the same rules
// for flags apply for attribute
// keys

pub fn not_character<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, &'a str> {
    let (source, result) =
        recognize(preceded(not(tag(character)), one_of("`~!@#$%^&*()<>[]{}")))
            .parse(source)?;
    Ok((source, result))
}

pub fn span_attr<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, RawSpanMetadata> {
    let (source, spans) =
        (tag("|"), space0, opt(line_ending), space0).parse(source)?;
    let (source, key_snippets) = many1(alt((
        is_not(": \r\n\t\\|~`!@#$%^&*()<>[]{}"),
        single_character,
        |src| not_character(src, character),
    )))
    .parse(source)?;
    let (source, _) = tag(":").parse(source)?;
    let (source, _) = (space0, opt(line_ending), space0).parse(source)?;
    let (source, spans) = many1(alt((text_span, code_span))).parse(source)?;
    Ok((
        source,
        RawSpanMetadata::Attr {
            key: key_snippets.join("").to_string(),
            spans,
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("|alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("| alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("| \n alfa: bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa:\nbravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa:\n bravo``", "`", RawSpanMetadata::Attr{ key: "alfa".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case("|alfa!!@@##$$%%^^&&**(())[[]]{{}}<<>>:\n bravo``", "`", RawSpanMetadata::Attr{ key: "alfa!!@@##$$%%^^&&**(())[[]]{{}}<<>>".to_string(), spans: vec![Span::Text{content: "bravo".to_string()}]} , "``")]
    #[case(
        "|alfa: bravo``code-here``>>", 
        ">", 
        RawSpanMetadata::Attr{ 
            key: "alfa".to_string(), 
            spans: vec![
                Span::Text{content: "bravo".to_string()},
                Span::Code{attrs: BTreeMap::new(), flags: vec![], spans: vec![
                    Span::Text{content: "code-here".to_string()}
                ]}
            ]
        }, 
        ">>")]
    fn span_flag_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] left: RawSpanMetadata,
        #[case] remainder: &str,
    ) {
        let right = span_attr(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }
}

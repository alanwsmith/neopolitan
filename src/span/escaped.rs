#![allow(unused)]
use crate::neo_config::NeoConfig;
use crate::section_attr::raw_section_attr;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
use crate::span::Span;
use crate::span_metadata::span_metadata;
use crate::span_strings::escaped_character;
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

pub fn escaped_span<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, content) = escaped_character(source)?;
    Ok((
        source,
        Span::Escaped {
            content: content.to_string(),
        },
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("\\|", Span::Escaped{
        content: "|".to_string()
    }, "")]
    #[case("\\\\ ", Span::Escaped{
        content: "\\".to_string()
    }, " ")]
    fn escaped_span_valid_tests(
        #[case] source: &str,
        #[case] left: Span,
        #[case] remainder: &str,
    ) {
        let right = escaped_span(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }
}

#![allow(unused)]
use crate::span::Span;
use crate::span_strings::plain_text_any_colons::plain_text_any_colons;
use crate::span_strings::plain_text_any_pipes::plain_text_any_pipes;
use crate::span_strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_strings::plain_text_span_for_block_paragraph::plain_text_span_for_block_paragraph;
use crate::span_strings::plain_text_string_base::plain_text_string_base;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::not;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;

pub fn span_of_plain_text_for_section_key_value_attr_value(
    source: &str,
) -> IResult<&str, Span> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_any_pipes,
        plain_text_any_colons,
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: results.join("").to_string(),
        },
    ))
}

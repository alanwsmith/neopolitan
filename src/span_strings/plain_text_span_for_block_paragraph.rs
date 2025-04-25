#![allow(unused)]
use crate::span_strings::plain_text_any_colons::plain_text_any_colons;
use crate::span_strings::plain_text_any_pipes::plain_text_any_pipes;
use crate::span_strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
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

pub fn plain_text_span_for_block_paragraph(
    source: &str,
) -> IResult<&str, Vec<&str>> {
    let (source, results) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        plain_text_single_line_ending_as_space,
        plain_text_any_pipes,
        plain_text_any_colons,
    )))
    .parse(source)?;
    Ok((source, results))
}

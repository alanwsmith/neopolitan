#![allow(unused)]
use crate::span::Span;
use crate::span::text::TextSpan;
use crate::span_strings::plain_text_span_for_block_paragraph::plain_text_span_for_block_paragraph;
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

pub fn span_of_plain_text_for_block_paragraph(
    source: &str,
) -> IResult<&str, Span> {
    let (source, results) =
        plain_text_span_for_block_paragraph.parse(source)?;
    Ok((
        source,
        Span::Text(TextSpan {
            text: results.join(""),
        }),
    ))
}

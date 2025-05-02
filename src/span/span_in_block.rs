use crate::shorthand_span::shorthand_span;
use crate::span::Span;
use crate::spans::text_span_in_block::text_span_in_block;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;

pub fn span_in_block<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, span) =
        alt((text_span_in_block, shorthand_span)).parse(source)?;
    Ok((source, span))
}

use crate::span::Span;
use crate::span::code::code_span;
use crate::span::escaped::escaped_span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;

pub fn shorthand_span(source: &str) -> IResult<&str, Span> {
    let (source, span) = alt((code_span, escaped_span)).parse(source)?;
    Ok((source, span))
}

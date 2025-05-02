use crate::span::Span;
use crate::span::code_span::code_span;
use crate::span::escaped_span::escaped_span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;

pub fn shorthand_span<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, span) = alt((code_span, escaped_span)).parse(source)?;
    Ok((source, span))
}

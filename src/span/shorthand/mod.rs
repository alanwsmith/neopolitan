// TODO: Remand to shorthand_wrapper

use crate::span::Span;
use crate::span::code_shorthand::code_shorthand;
use crate::span::escaped::escaped_span;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;

pub fn shorthand_span(source: &str) -> IResult<&str, Span> {
    let (source, span) = alt((code_shorthand, escaped_span)).parse(source)?;
    Ok((source, span))
}

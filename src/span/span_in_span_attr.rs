use crate::shorthand_span::shorthand_span;
use crate::span::Span;
use crate::spans::text_span_in_span_attr::text_span_in_span_attr;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;

// NOTE: Trailing space is required
// since there can be other spans
// after it.
// TODO: Figure out a way to chomp
// just the last one.

pub fn span_in_span_attr<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, span) =
        alt((text_span_in_span_attr, shorthand_span)).parse(source)?;
    Ok((source, span))
}

#[cfg(test)]
mod test {
    // testing is done in the individual span functions
}

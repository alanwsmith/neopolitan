use crate::span::Span;
use crate::span::escaped::escaped_span;
use crate::span_metadata::span_metadata;
use crate::span_metadata::strings::escaped_character::escaped_character;
use crate::span_metadata::strings::plain_text_single_line_ending_as_space::plain_text_single_line_ending_as_space;
use crate::span_metadata::strings::plain_text_space1_as_single_space::plain_text_space1_as_single_space;
use crate::span_metadata::strings::plain_text_string_base::plain_text_string_base;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::is_a;
use nom::bytes::complete::tag;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::pair;
use nom::sequence::preceded;

pub fn code_span<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, spans) = preceded(
        pair(tag("``"), opt(plain_text_space1_as_single_space)),
        many1(alt((escaped_span, code_span_text))),
    )
    .parse(source)?;
    let (source, (flags, attrs)) = span_metadata(source, "`")?;
    let (source, _) = tag("``").parse(source)?;
    Ok((
        source,
        Span::Code {
            attrs,
            flags,
            spans,
        },
    ))
}

// TODO: Move this to text_span_in_span and
// pass character to it.

pub fn code_span_text<'a>(source: &'a str) -> IResult<&'a str, Span> {
    let (source, parts) = many1(alt((
        plain_text_string_base,
        plain_text_space1_as_single_space,
        is_a("%@~*^![]{}<>_#:"),
        plain_text_single_line_ending_as_space,
        escaped_character,
    )))
    .parse(source)?;
    Ok((
        source,
        Span::Text {
            content: parts.join("").trim().to_string(),
        },
    ))
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use std::collections::BTreeMap;

    #[test]
    fn code_span_simple_test() {
        let source = "``alfa``";
        let left = Span::Code {
            attrs: BTreeMap::new(),
            flags: vec![],
            spans: vec![Span::Text {
                content: "alfa".to_string(),
            }],
        };
        let remainder = "";
        let right = code_span(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn code_span_single_flag_test() {
        let source = "``alfa|bravo``";
        let left = Span::Code {
            attrs: BTreeMap::new(),
            flags: vec!["bravo".to_string()],
            spans: vec![Span::Text {
                content: "alfa".to_string(),
            }],
        };
        let remainder = "";
        let right = code_span(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn code_span_newline_test() {
        let source = "``\nalfa\n|\nbravo:\ncharlie\n|\ndelta\n`` ping";
        let mut attrs = BTreeMap::new();
        attrs.insert(
            "bravo".to_string(),
            vec![Span::Text {
                content: "charlie ".to_string(),
            }],
        );
        let flags = vec!["delta".to_string()];
        let left = Span::Code {
            attrs,
            flags,
            spans: vec![Span::Text {
                content: "alfa".to_string(),
            }],
        };
        let remainder = " ping";
        let right = code_span(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn code_span_newline_test_2() {
        let source = "``\nalfa\n|\nbravo:\ncharlie \n echo\n|\ndelta\n`` ping";
        let mut attrs = BTreeMap::new();
        attrs.insert(
            "bravo".to_string(),
            vec![Span::Text {
                content: "charlie echo ".to_string(),
            }],
        );
        let flags = vec!["delta".to_string()];
        let left = Span::Code {
            attrs,
            flags,
            spans: vec![Span::Text {
                content: "alfa".to_string(),
            }],
        };
        let remainder = " ping";
        let right = code_span(source).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    //
}

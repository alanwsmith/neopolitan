pub fn code_span<'a>(source: &'a str) -> IResult<&'a str, Span> {
    //let (source, span) = shorthand_span(source, "`", "`")?;
    Ok((source, span))
}

use crate::span::Span;
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

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct CodeSpan {
    pub attrs: BTreeMap<String, Vec<Span>>,
    pub flags: Vec<String>,
    pub kind: String,
    pub text: String,
}

pub fn code_span(source: &str) -> IResult<&str, Span> {
    let (source, tokens) = preceded(
        pair(tag("``"), opt(plain_text_space1_as_single_space)),
        many1(alt((
            plain_text_string_base,
            plain_text_space1_as_single_space,
            is_a("%@~*^![]{}<>_#:"),
            plain_text_single_line_ending_as_space,
            escaped_character,
        ))),
    )
    .parse(source)?;
    let (source, (flags, attributes)) = code_span_metadata.parse(source)?;
    let (source, _) = tag("``").parse(source)?;
    Ok((
        source,
        Span::CodeSpan {
            kind: "code-shorthand".to_string(),
            attributes,
            flags,
            text: tokens.join(""),
        },
    ))
}

pub fn code_span_metadata(
    source: &str,
) -> IResult<&str, (Vec<String>, BTreeMap<String, Vec<Span>>)> {
    // Reminder: attrs first otherwise things go wrong with this setup
    let (source, metadata) =
        many0(alt((code_span_attribute, code_span_flag))).parse(source)?;
    let mut attrs = BTreeMap::new();
    let mut flags = vec![];

    // metadata.iter().for_each(|af| match &af {
    //     SpanBaseAttrV42::Flag(f) => flags.push(f.text.to_string()),
    //     SpanBaseAttrV42::KeyValue { key, spans } => {
    //         attrs.insert(key.to_string(), spans.clone());
    //         ()
    //     }
    // });

    Ok((source, (flags, attrs)))
}

pub fn code_span_flag(source: &str) -> IResult<&str, Span> {
    let (source, _) = tag("|").parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, _) = opt(line_ending).parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, parts) = many1(alt((
        plain_text_string_base,
        is_a("%@~*^![]{}<>_#:"),
        escaped_character,
    )))
    .parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, _) = opt(line_ending).parse(source)?;
    let (source, _) = space0.parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
            text: parts.join(""),
        }),
    ))
}

// TODO: Deprecate this
pub fn not_backtick_or_pipe_or_escape(source: &str) -> IResult<&str, Span> {
    let (source, result) = is_not("`\\|").parse(source)?;
    Ok((
        source,
        Span::PlainText(PlainTextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        }),
    ))
}

// TODO: Deprecate this
pub fn code_shorthand_span_flag_attr(
    source: &str,
) -> IResult<&str, SpanBaseAttrV42> {
    let (source, spans) =
        many1(alt((not_backtick_or_pipe_or_escape,))).parse(source)?;
    let (source, _) = not(tag(":")).parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
            text: flatten_spans_v42(&spans),
        }),
    ))
}

pub fn code_shorthand_attr(source: &str) -> IResult<&str, SpanBaseAttrV42> {
    let (source, _) = tag("|").parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, _) = opt(line_ending).parse(source)?;
    let (source, _) = space0.parse(source)?;
    let (source, key_spans) = terminated(
        many1(alt((alphanumeric1, is_a("-_")))),
        pair(tag(":"), space1),
    )
    .parse(source)?;
    let (source, spans) = many1(alt((
        span_of_plain_text_for_shorthand_attr_value,
        named_span,
        span_of_escaped_character,
        code_span_of_extra_shorthand_attr_value,
    )))
    .parse(source)?;
    Ok((
        source,
        SpanBaseAttrV42::KeyValue {
            key: key_spans.join(""),
            spans,
        },
    ))
}

pub fn code_span_of_extra_shorthand_attr_value(
    source: &str,
) -> IResult<&str, Span> {
    let (source, result) = is_a("~*^![]{}<>_#:").parse(source)?;
    Ok((
        source,
        Span::TextSpan {
            kind: "plain-text".to_string(),
            text: result.to_string(),
        },
    ))
}

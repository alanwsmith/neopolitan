#![allow(unused)]

use crate::neo_config::NeoConfig;
use crate::section_attribute::raw_section_attribute;
use crate::section_flag::raw_section_flag;
use crate::section_parent::SectionParent;
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

pub fn shorthand_span<'a>(
    source: &'a str,
    start_marker: &'a str,
    end_marker: &'a str,
) -> IResult<&'a str, Span> {
    let all_characters = "`%@~*^![]{}<>_#:";
    let characters = all_characters
        .split("")
        .filter(|c| *c != "" && *c != start_marker && *c != end_marker)
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("")
        .to_string();
    let (source, tokens) = preceded(
        pair(
            pair(tag(start_marker), tag(start_marker)),
            opt(plain_text_space1_as_single_space),
        ),
        many1(alt((
            plain_text_string_base,
            plain_text_space1_as_single_space,
            is_a("%@~*^![]{}<>_#:"),
            plain_text_single_line_ending_as_space,
            escaped_character,
        ))),
    )
    .parse(source)?;
    let (source, (flags, attributes)) =
        shorthand_metadata(source, characters.clone())?;
    let (source, _) = tag("``").parse(source)?;
    Ok((
        source,
        Span::CodeSpan {
            attributes: BTreeMap::new(),
            flags: vec![],
            kind: "code-shorthand".to_string(),
            spans: vec![Span::TextSpan {
                kind: "text".to_string(),
                text: "asdf".to_string(),
            }],
        },
    ))
}

// pub fn code_span_attribute(source: &str) -> IResult<&str, SpanBaseAttrV42> {
//     let (source, _) = tag("|").parse(source)?;
//     let (source, _) = space0.parse(source)?;
//     let (source, _) = opt(line_ending).parse(source)?;
//     let (source, _) = space0.parse(source)?;
//     let (source, key_spans) = terminated(
//         many1(alt((alphanumeric1, is_a("-_")))),
//         pair(tag(":"), space1),
//     )
//     .parse(source)?;
//     let (source, spans) = many1(alt((
//         span_of_plain_text_for_shorthand_attr_value,
//         named_span,
//         span_of_escaped_character,
//         code_span_of_extra_shorthand_attr_value,
//     )))
//     .parse(source)?;
//     Ok((
//         source,
//         SpanBaseAttrV42::KeyValue {
//             key: key_spans.join(""),
//             spans,
//         },
//     ))
// }

// pub fn code_span_flag(source: &str) -> IResult<&str, Span> {
//     let (source, _) = tag("|").parse(source)?;
//     let (source, _) = space0.parse(source)?;
//     let (source, _) = opt(line_ending).parse(source)?;
//     let (source, _) = space0.parse(source)?;
//     let (source, parts) = many1(alt((
//         plain_text_string_base,
//         is_a("%@~*^![]{}<>_#:"),
//         escaped_character,
//     )))
//     .parse(source)?;
//     let (source, _) = space0.parse(source)?;
//     let (source, _) = opt(line_ending).parse(source)?;
//     let (source, _) = space0.parse(source)?;
//     Ok((
//         source,
//         SpanBaseAttrV42::Flag(SpanFlagAttrV42 {
//             text: parts.join(""),
//         }),
//     ))
// }

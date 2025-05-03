#![allow(unused)]
pub mod attr;
pub mod flag;
pub mod parsers;
pub mod strings;

use crate::span::Span;
use crate::span_metadata::attr::span_attr;
use crate::span_metadata::flag::span_flag;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::multi::many0;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum RawSpanMetadata {
    Attr { key: String, spans: Vec<Span> },
    Flag(String),
}

// TODO: create ``metadata_key`` or something
// like that that is used for flags and
// the keys for attrs so they have the same
// rules.

// NOTE: multiple attrs with the same
// key are collapsed into one. Adding
// spaces between this is an implementation
// detail if it's necessary.

// NOTE: Trailing whitespace is left in
// the spans. That means it's possible
// to have trailing white space at the
// end of an output. Removing that is
// the responsibility of the output
// implementation.

// pub fn span_metadata<'a>(
//     source: &'a str,
//     character: &'a str,
// ) -> IResult<&'a str, (Vec<String>, BTreeMap<String, Vec<Vec<Span>>>)> {
//     // Reminder: attrs first otherwise things go wrong with this setup
//     let (source, raw_metadata) = many0(alt((
//         |src| span_attr(src, character),
//         |src| span_flag(src, character),
//     )))
//     .parse(source)?;
//     let mut flags = raw_metadata
//         .iter()
//         .filter_map(|data| match data {
//             RawSpanMetadata::Flag(content) => Some(content.clone()),
//             _ => None,
//         })
//         .collect::<Vec<String>>();
//     let mut attrs: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
//     raw_metadata.iter().for_each(|data| match data {
//         RawSpanMetadata::Attr { key, spans } => {
//             match attrs.get_mut(key) {
//                 Some(v) => {
//                     v.push(spans.clone());
//                     ()
//                 }
//                 None => {
//                     attrs.insert(key.to_string(), vec![spans.clone()]);
//                     ()
//                 }
//             }
//             ()
//         }
//         _ => (),
//     });
//     Ok((source, (flags, attrs)))
// }

pub fn span_metadata<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, (Vec<String>, BTreeMap<String, Vec<Span>>)> {
    // Reminder: attrs first otherwise things go wrong with this setup
    let (source, raw_metadata) = many0(alt((
        |src| span_attr(src, character),
        |src| span_flag(src, character),
    )))
    .parse(source)?;
    let mut flags = raw_metadata
        .iter()
        .filter_map(|data| match data {
            RawSpanMetadata::Flag(content) => Some(content.clone()),
            _ => None,
        })
        .collect::<Vec<String>>();
    let mut attrs: BTreeMap<String, Vec<Span>> = BTreeMap::new();
    raw_metadata.iter().for_each(|data| if let RawSpanMetadata::Attr { key, spans } = data { match attrs.get_mut(key) {
        Some(v) => {
            spans.iter().for_each(|span| v.push(span.clone()));
        }
        None => {
            attrs.insert(key.to_string(), spans.clone());
        }
    } });
    Ok((source, (flags, attrs)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[test]
    fn single_flag_test() {
        let source = "|alfa``";
        let character = "`";
        let flags = vec!["alfa".to_string()];
        let attrs = BTreeMap::new();
        let left = (flags, attrs);
        let remainder = "``";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn single_attr_test() {
        let source = "|alfa: bravo``";
        let character = "`";
        let flags = vec![];
        let mut attrs = BTreeMap::new();
        attrs.insert(
            "alfa".to_string(),
            vec![Span::Text {
                content: "bravo".to_string(),
            }],
        );
        let left = (flags, attrs);
        let remainder = "``";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    #[ignore]
    fn single_attr_with_text_instead_of_nested_spans() {
        // NOTE: the backticks that would be a code
        // section in content are just text here.
        let source = "|alfa: bravo ``charlie``>>";
        let character = "`";
        let flags = vec![];
        let mut attrs = BTreeMap::new();
        attrs.insert(
            "alfa".to_string(),
            vec![Span::Text {
                content: "bravo ``charlie``".to_string(),
            }],
        );
        let left = (flags, attrs);
        let remainder = ">>";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn multi_attr_test() {
        let source = "|alfa: bravo|charlie: delta``";
        let character = "`";
        let flags = vec![];
        let mut attrs: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        attrs.insert(
            "alfa".to_string(),
            vec![Span::Text {
                content: "bravo".to_string(),
            }],
        );
        attrs.insert(
            "charlie".to_string(),
            vec![Span::Text {
                content: "delta".to_string(),
            }],
        );
        let left = (flags, attrs);
        let remainder = "``";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn single_attr_multiple_times_test() {
        let source = "|alfa: bravo|alfa: delta``";
        let character = "`";
        let flags = vec![];
        let mut attrs: BTreeMap<String, Vec<Span>> = BTreeMap::new();
        let alfa_vecs = vec![
            Span::Text {
                content: "bravo".to_string(),
            },
            Span::Text {
                content: "delta".to_string(),
            },
        ];
        attrs.insert("alfa".to_string(), alfa_vecs);
        let left = (flags, attrs);
        let remainder = "``";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }
}

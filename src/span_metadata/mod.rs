#![allow(unused)]
use crate::span::Span;
use crate::span_attr::span_attr;
use crate::span_flag::span_flag;
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

pub fn span_metadata<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, (Vec<String>, BTreeMap<String, Vec<Vec<Span>>>)> {
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
    let mut attrs: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
    raw_metadata.iter().for_each(|data| match data {
        RawSpanMetadata::Attr { key, spans } => {
            match attrs.get_mut(key) {
                Some(v) => {
                    v.push(spans.clone());
                    ()
                }
                None => {
                    attrs.insert(key.to_string(), vec![spans.clone()]);
                    ()
                }
            }
            ()
        }
        _ => (),
    });
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
            vec![vec![Span::Text {
                content: "bravo".to_string(),
            }]],
        );
        let left = (flags, attrs);
        let remainder = "``";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    #[test]
    fn multi_attr_test() {
        let source = "|alfa: bravo|charlie: delta``";
        let character = "`";
        let flags = vec![];
        let mut attrs: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
        attrs.insert(
            "alfa".to_string(),
            vec![vec![Span::Text {
                content: "bravo".to_string(),
            }]],
        );
        attrs.insert(
            "charlie".to_string(),
            vec![vec![Span::Text {
                content: "delta".to_string(),
            }]],
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
        let mut attrs: BTreeMap<String, Vec<Vec<Span>>> = BTreeMap::new();
        let alfa_vecs = vec![
            vec![Span::Text {
                content: "bravo".to_string(),
            }],
            vec![Span::Text {
                content: "delta".to_string(),
            }],
        ];
        attrs.insert("alfa".to_string(), alfa_vecs);
        let left = (flags, attrs);
        let remainder = "``";
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    //
}

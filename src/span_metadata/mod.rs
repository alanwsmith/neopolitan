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
    let mut attrs = BTreeMap::new();
    Ok((source, (flags, attrs)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::span::Span;
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case("|alfa``", "`", (vec!["alfa".to_string()], BTreeMap::new()), "``")]
    fn span_metadata_valid_tests(
        #[case] source: &str,
        #[case] character: &str,
        #[case] left: (Vec<String>, BTreeMap<String, Vec<Span>>),
        #[case] remainder: &str,
    ) {
        let right = span_metadata(source, character).unwrap();
        assert_eq!(left, right.1);
        assert_eq!(remainder, right.0);
    }

    //
}

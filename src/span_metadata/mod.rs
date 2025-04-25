#![allow(unused)]
use crate::span::Span;
use crate::span_flag::span_flag;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::multi::many0;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum RawSpanMetadata {
    Attribute { key: String, spans: Vec<Span> },
    Flag(String),
}

pub fn span_metadata<'a>(
    source: &'a str,
    character: &'a str,
) -> IResult<&'a str, (Vec<String>, BTreeMap<String, Vec<Span>>)> {
    // Reminder: attrs first otherwise things go wrong with this setup
    let (source, raw_metadata) =
        many0(alt((|src| span_flag(src, character),))).parse(source)?;
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

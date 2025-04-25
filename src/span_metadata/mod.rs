#![allow(unused)]
use crate::span::Span;
use crate::span_flag::span_flag;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::multi::many0;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum RawShorthandMetadata {
    Attribute { key: String, spans: Vec<Span> },
    Flag(String),
}

#[derive(Debug, PartialEq)]
pub enum RawShorthandMetadataDev {
    Flag(Vec<Span>),
}

pub fn span_metadata<'a>(
    source: &'a str,
    characters: &'a str,
) -> IResult<&'a str, (Vec<String>, BTreeMap<String, Vec<Span>>)> {
    let (source, raw_metadata) =
        many0(alt((|src| span_flag(src, characters),))).parse(source)?;
    // Reminder: attrs first otherwise things go wrong with this setup
    // let (source, metadata) =
    //     many0(alt((code_span_attribute, code_span_flag))).parse(source)?;
    let mut flags = vec![];
    let mut attrs = BTreeMap::new();
    Ok((source, (flags, attrs)))
}

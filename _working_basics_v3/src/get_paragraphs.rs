#![allow(non_snake_case)]
use crate::get_text::get_text;
use crate::section::Section;
use nom::branch::alt;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;
use std::collections::HashMap;

pub fn get_paragraphs(source: &str) -> IResult<&str, Vec<Section>> {
    let (source, paragraphs) = many_till(split_paragraphs, eof)(source)?;
    Ok((source, paragraphs.0))
}

pub fn split_paragraphs(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, raw_text) = alt((take_until1("\n\n"), rest))(source)?;
    let (source, _) = multispace0(source)?;
    let (_, text) = get_text(raw_text)?;
    Ok((
        source,
        Section::P {
            attributes: HashMap::new(),
            children: text,
        },
    ))
}
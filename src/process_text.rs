#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

pub fn process_text_old(source: &str) -> IResult<&str, Vec<Chunk>> {
    let response: Vec<Chunk> = vec![Chunk::Text {
        value: "Open the crate".to_string(),
    }];
    Ok(("", response))
}

pub fn process_text(source: &str) -> IResult<&str, Vec<Chunk>> {
    let mut response: Vec<Chunk> = vec![];
    let (source, pretext) = alt((tuple((take_until("`"), rest)), tuple((rest, rest))))(source)?;
    response.push(Chunk::Text {
        value: pretext.0.to_string(),
    });
    dbg!(&pretext);
    if pretext.1.is_empty() {
        Ok((source, response))
    } else {
        let (source, current) = tag("`")(pretext.1)?;
        let (source, code) = take_until("`")(source)?;
        dbg!(&code);
        let (source, current) = tag("`")(source)?;
        let (source, language) = take_until("`")(source)?;
        dbg!(&language);
        let (source, current) = tag("`")(source)?;
        dbg!(&source);
        response.push(Chunk::InlineCode {
            value: Some(code.to_string()),
            language: Some(language.to_string()),
            attributes: None,
        });
        Ok((source, response))
    }
}

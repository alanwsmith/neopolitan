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

pub fn process_text(source: &str) -> IResult<&str, Vec<Chunk>> {
    let response: Vec<Chunk> = vec![Chunk::Text {
        value: "Open the crate".to_string(),
    }];
    Ok(("", response))
}

pub fn process_text_dev(source: &str) -> IResult<&str, Vec<Chunk>> {
    let mut response: Vec<Chunk> = vec![];
    let (source, pretext) = take_until("`")(source)?;
    response.push(Chunk::Text {
        value: pretext.to_string(),
    });
    let (source, _) = tag("`")(source)?;
    let (source, text) = take_until("`")(source)?;
    let (source, _) = tag("`")(source)?;
    let (source, language) = take_until("`")(source)?;
    response.push(Chunk::InlineCode {
        value: Some(text.to_string()),
        language: Some(language.to_string()),
        attributes: None,
    });
    let (source, _) = take_until("`")(source)?;
    let (remainder, _) = tag("`")(source)?;
    let dev_response: Vec<Chunk> = vec![
        Chunk::Text {
            value: "The".to_string(),
        },
        Chunk::InlineCode {
            attributes: None,
            language: Some("rust".to_string()),
            value: Some("frosty".to_string()),
        },
    ];
    Ok((remainder, response))
}

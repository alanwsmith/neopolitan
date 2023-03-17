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
    let (source, containers) = many_till(text, eof)(source)?;
    let response = containers.0.concat();
    Ok(("", response))
}

#[derive(Debug)]
enum Target {
    Link { pretext: String },
    Rest { pretext: String },
    Code { pretext: String },
}

fn text(source: &str) -> IResult<&str, Vec<Chunk>> {
    dbg!(&source);
    let mut response: Vec<Chunk> = vec![];
    let (source, payload) = alt((
        tuple((
            take_until("<<link").map(|v: &str| Target::Link {
                pretext: v.to_string(),
            }),
            rest,
        )),
        tuple((
            take_until("`").map(|v: &str| Target::Code {
                pretext: v.to_string(),
            }),
            rest,
        )),
        tuple((
            rest.map(|v: &str| Target::Rest {
                pretext: v.to_string(),
            }),
            rest,
        )),
    ))(source)?;
    match payload.0 {
        Target::Code { pretext } => {
            dbg!(&pretext);
            response.push(Chunk::Text {
                value: pretext.to_string(),
            });
            let (source, current) = tag("`")(payload.1)?;
            let (source, code) = take_until("`")(source)?;
            let (source, current) = tag("`")(source)?;
            let (source, language) = take_until("`")(source)?;
            let (source, current) = tag("`")(source)?;
            response.push(Chunk::InlineCode {
                value: Some(code.to_string()),
                language: Some(language.to_string()),
                attributes: None,
            });
            Ok((source, response))
        }
        Target::Link { pretext } => {
            response.push(Chunk::Text {
                value: pretext.to_string(),
            });
            let (source, _) = tag("<<")(payload.1)?;
            let (source, kind) = take_until("|")(source)?;
            let (source, _) = tag("|")(source)?;
            let (source, value) = take_until("|")(source)?;
            let (source, _) = tag("|")(source)?;
            let (source, url) = take_until(">>")(source)?;
            let (source, _) = tag(">>")(source)?;
            response.push(Chunk::Link {
                value: Some(value.to_string()),
                url: Some(url.to_string()),
                attributes: None,
            });
            Ok((source, response))
        }
        Target::Rest { pretext } => {
            response.push(Chunk::Text {
                value: pretext.to_string(),
            });
            Ok((source, response))
        }
    }
}

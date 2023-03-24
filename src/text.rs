#![allow(warnings)]
use crate::chunk::Chunk;
use crate::inline_language::*;
use crate::language::*;
use crate::page::Page;
use crate::parse_text_attributes::parse_text_attributes;
use crate::split::split;
use crate::tag_attributes::*;
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

pub fn text(source: &str) -> IResult<&str, Option<Vec<Chunk>>> {
    let (source, _) = multispace0(source)?;
    let (source, containers) = many_till(text_parser, eof)(source)?;
    let response = containers.0.concat();
    Ok(("", Some(response)))
}

#[derive(Debug)]
enum Target<'a> {
    Code { pretext: String, divider: &'a str },
    Link { pretext: String },
    Rest { pretext: String },
    Strong { pretext: String },
}

fn text_parser(source: &str) -> IResult<&str, Vec<Chunk>> {
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
                divider: "`",
                pretext: v.to_string(),
            }),
            rest,
        )),
        tuple((
            take_until("*").map(|v: &str| Target::Strong {
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
        Target::Code { pretext, divider } => {
            // dbg!(&source);
            response.push(Chunk::Text {
                attributes: None,
                value: Some(pretext.to_string()),
            });
            let (source, current) = tag(divider)(payload.1)?;
            let (source, code) = take_until(divider)(source)?;
            let (source, current) = tag(divider)(source)?;
            let (source, raw_attributes) = take_until(divider)(source)?;
            let (source, current) = tag(divider)(source)?;
            let (_, language) = inline_language(raw_attributes)?;
            let (_, attributes) = parse_text_attributes(raw_attributes)?;
            response.push(Chunk::InlineCode {
                language,
                attributes,
                value: Some(code.to_string()),
            });
            Ok((source, response))
        }

        Target::Link { pretext } => {
            response.push(Chunk::Text {
                attributes: None,
                value: Some(pretext.to_string()),
            });
            let (source, _) = tag("<<")(payload.1)?;
            let (remainder, stuff) = take_until(">>")(source)?;
            let (_, stuff) = split(stuff, "|")?;
            let value = Some(stuff[2].to_string());
            let url = Some(stuff[1].to_string());
            let (remainder, _) = tag(">>")(remainder)?;
            if stuff.len() > 3 {
                let (_, attributes) = parse_text_attributes(stuff[3])?;
                response.push(Chunk::Link {
                    value,
                    url,
                    attributes,
                });
                Ok((remainder, response))
            } else {
                response.push(Chunk::Link {
                    value,
                    url,
                    attributes: None,
                });
                Ok((remainder, response))
            }
        }

        Target::Strong { pretext } => {
            response.push(Chunk::Text {
                attributes: None,
                value: Some(pretext.to_string()),
            });
            let (source, current) = tag("*")(payload.1)?;
            let (source, code) = take_until("*")(source)?;
            let (source, current) = tag("*")(source)?;
            let (source, raw_attributes) = take_until("*")(source)?;
            let (source, current) = tag("*")(source)?;
            let attributes = parse_text_attributes(raw_attributes).unwrap().1;
            response.push(Chunk::Strong {
                value: Some(code.to_string()),
                attributes: None,
            });
            Ok((source, response))
        }
        Target::Rest { pretext } => {
            response.push(Chunk::Text {
                attributes: None,
                value: Some(pretext.to_string()),
            });
            Ok((source, response))
        }
    }
}

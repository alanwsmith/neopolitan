#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
use crate::tag_attributes::*;
use crate::text_attributes::*;
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
            response.push(Chunk::Text {
                value: pretext.to_string(),
            });
            let (source, current) = tag(divider)(payload.1)?;
            let (source, code) = take_until(divider)(source)?;
            let (source, current) = tag(divider)(source)?;
            let (source, raw_attributes) = take_until(divider)(source)?;
            let (source, current) = tag(divider)(source)?;
            let attributes = text_attributes(raw_attributes).unwrap().1;
            let mut language: Option<String> = None;
            // TODO: See if there's a more direct way to
            // do this
            if attributes.as_ref().expect("check failed").is_empty() {
                // no attributes so no language
            } else {
                // dbg!(&attributes.as_ref().expect("check failed")[0]);
                match &attributes.as_ref().expect("check failed")[0] {
                    // first attribute is stand along so
                    // it's speced to be a langauge
                    (Some(lang), None) => {
                        language = Some(lang.to_string());
                    }
                    _ => {}
                }
            }
            let language = language;
            response.push(Chunk::InlineCode {
                value: Some(code.to_string()),
                language,
                attributes,
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
            let (source, raw_payload) = take_until(">>")(source)?;
            // dbg!(&raw_payload);
            let (source, _) = tag(">>")(source)?;
            let payload = tag_attributes(raw_payload).unwrap();
            // dbg!(&payload);

            response.push(Chunk::Link {
                value: Some(value.to_string()),
                // url: Some("https://paper.example.com/".to_string()),
                url: Some(payload.0.to_string()),
                // attributes: Some(vec![(Some("id".to_string()), Some("rider".to_string()))]),
                attributes: payload.1,
            });
            Ok((source, response))
        }
        Target::Strong { pretext } => {
            response.push(Chunk::Text {
                value: pretext.to_string(),
            });
            let (source, current) = tag("*")(payload.1)?;
            let (source, code) = take_until("*")(source)?;
            let (source, current) = tag("*")(source)?;
            let (source, language) = take_until("*")(source)?;
            let (source, current) = tag("*")(source)?;
            response.push(Chunk::Strong {
                value: Some(code.to_string()),
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

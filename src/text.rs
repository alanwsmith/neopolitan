#![allow(warnings)]
use crate::chunk::Chunk;
use crate::code_inline::*;
use crate::em::*;
use crate::inline_language::*;
use crate::language::*;
use crate::link::*;
use crate::page::Page;
use crate::parse_text_attributes::parse_text_attributes;
use crate::split::split;
use crate::strong::*;
use crate::tag_attributes::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_till;
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
use nom::combinator::success;
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
    let response = containers.0;
    Ok(("", Some(response)))
}

#[derive(Debug)]
enum Target {
    Code,
    CodeInlineTagsNoAttributes,
    Em,
    Link,
    // Rest { pretext: String },
    Strong,
}

fn text_parser(source: &str) -> IResult<&str, Chunk> {
    let mut response: Vec<Chunk> = vec![];
    let (text_to_process, plain_text) = take_till(|c| c == '<' || c == '*' || c == '`')(source)?;
    if plain_text.is_empty() {
        let (remainder, payload) = alt((
            tuple((
                tag("<<code|").map(|v: &str| Target::CodeInlineTagsNoAttributes),
                take_until(">>"),
                tag(">>"),
                success(""),
                success(""),
            )),
            tuple((
                tag("<<link").map(|v: &str| Target::Link),
                take_until(">>"),
                tag(">>"),
                success(""),
                success(""),
            )),
            tuple((
                tag("<<em").map(|v: &str| Target::Em),
                take_until(">>"),
                tag(">>"),
                success(""),
                success(""),
            )),
            tuple((
                tag("*").map(|v: &str| Target::Strong),
                take_until("*"),
                tag("*"),
                take_until("*"),
                tag("*"),
            )),
            tuple((
                tag("`").map(|v: &str| Target::Code),
                take_until("`"),
                tag("`"),
                take_until("`"),
                tag("`"),
            )),
        ))(text_to_process)?;
        match payload.0 {
            Target::CodeInlineTagsNoAttributes => {
                code_inline_tags_no_attributes(payload.1, payload.3, remainder)
            }
            Target::Code => code_inline(payload.1, payload.3, remainder),
            Target::Em => strong(payload.1, payload.3, remainder),
            Target::Link => link(payload.1, remainder),
            Target::Strong => strong(payload.1, payload.3, remainder),
            _ => Ok((remainder, Chunk::Placeholder)),
        }
    } else {
        Ok((
            text_to_process,
            Chunk::Text {
                attributes: None,
                value: Some(plain_text.to_string()),
            },
        ))
    }
}

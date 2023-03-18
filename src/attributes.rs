#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
use crate::process_text::*;
use crate::section::*;
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
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Attributes {
    pub values: Option<Vec<(Option<String>, Option<String>)>>,
}

pub fn attributes_old(source: &str) -> IResult<&str, Attributes> {
    let (remainder, source) = take_until("\n\n")(source)?;
    let (final_part, mut parts) = many0(part)(source)?;
    parts.push(final_part);
    let mut attribute_holder: Vec<(Option<String>, Option<String>)> = vec![];
    for part in parts.iter().skip(1) {
        let (a, b) = attribute(part)?;
        attribute_holder.push(b);
    }
    let result = Attributes {
        values: Some(attribute_holder),
    };
    Ok((remainder.trim(), result))
}

pub fn attributes(source: &str) -> IResult<&str, Option<Vec<(Option<String>, Option<String>)>>> {
    // dbg!(&source);
    let (remainder, source) = take_until("\n\n")(source)?;
    // dbg!(&source);
    // dbg!(&remainder);
    let (final_part, mut parts) = many0(part)(source)?;
    // dbg!(&parts);
    // dbg!(&final_part);
    if final_part.is_empty() {
        parts.push(final_part);
        // dbg!(&parts);
        let mut attribute_holder: Vec<(Option<String>, Option<String>)> = vec![];
        for part in parts.iter().skip(1) {
            let (a, b) = attribute(part)?;
            attribute_holder.push(b);
        }
        Ok((remainder.trim(), None))
    } else {
        parts.push(final_part);
        // dbg!(&parts);
        let mut attribute_holder: Vec<(Option<String>, Option<String>)> = vec![];
        for part in parts.iter().skip(1) {
            let (a, b) = attribute(part)?;
            attribute_holder.push(b);
        }
        Ok((remainder.trim(), Some(attribute_holder)))
    }
}

fn part(source: &str) -> IResult<&str, &str> {
    let (source, content) = take_until(">> ")(source)?;
    let (source, _) = tag(">> ")(source)?;
    Ok((source, content))
}

pub fn attribute(source: &str) -> IResult<&str, (Option<String>, Option<String>)> {
    let (v, k) = alt((tuple((take_until(":"), rest)), tuple((rest, rest))))(source)?;
    if k.1.is_empty() {
        Ok((v, (Some(k.0.trim().to_string()), None)))
    } else {
        let (v, _) = tag(":")(k.1)?;
        let (v, _) = multispace0(v)?;
        Ok((
            v,
            (Some(k.0.trim().to_string()), Some(v.trim().to_string())),
        ))
    }
}

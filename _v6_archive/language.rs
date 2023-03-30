#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::page::Page;
use crate::section::*;
use crate::text::*;
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

pub fn language(source: &str) -> IResult<&str, Option<String>> {
    // TODO: Move this to use `split`` if you need to mess
    // with it more
    let (remainder, content) = alt((take_until("\n\n"), rest))(source)?;
    let (content, _) = multispace0(content)?;
    let (_, content) = alt((
        tuple((tag(">>"), not_line_ending, rest)),
        tuple((rest, rest, rest)),
    ))(content)?;
    if content.1.is_empty() {
        Ok(("", None))
    } else {
        let (a, b) = multispace0(content.1)?;
        let (c, d) = alt((tuple((take_until(": "), tag(": "))), tuple((rest, rest))))(a)?;
        if d.1.is_empty() {
            Ok(("", Some(d.0.trim().to_string())))
        } else {
            Ok(("", None))
        }
    }
}

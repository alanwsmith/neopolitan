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

// this is the one for text attributes
// the other one is for attributes for
// sections: TODO: Rename the other
// one to `section_attributes``
//

pub fn tag_attributes(
    source: &str,
) -> IResult<&str, Option<Vec<(Option<String>, Option<String>)>>> {
    let payload: Vec<(Option<String>, Option<String>)> = vec![];
    let (remainder, mut parts) = many0(part)(source)?;
    parts.push(remainder);
    if parts.len() == 0 {
        Ok(("", None))
    } else {
        // dbg!(&parts);
        let response: Vec<(Option<String>, Option<String>)> = parts
            .iter()
            .map(|p| attribute(p).unwrap().1)
            .skip(1)
            .collect();
        Ok((&parts[0], Some(response)))
    }
}

fn get_parts(source: &str) -> IResult<&str, Option<Vec<&str>>> {
    let (remainder, mut parts) = many0(part)(source)?;
    parts.push(remainder);
    Ok(("", Some(parts)))
}

fn part(source: &str) -> IResult<&str, &str> {
    let (source, content) = take_until("|")(source)?;
    let (source, _) = tag("|")(source)?;
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

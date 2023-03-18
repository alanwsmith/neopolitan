#![allow(warnings)]
use crate::attributes::attributes;
use crate::chunk::Chunk;
use crate::code::*;
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
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

pub fn list(source: &str) -> IResult<&str, Section> {
    let (source, attributes) = attributes(source)?;
    let (remainder, mut parts) = many0(part)(source)?;
    parts.push(remainder.trim());
    let items: Vec<Chunk> = parts
        .iter()
        .skip(1)
        .map(|p| Chunk::ListItem {
            attributes: attributes.clone(),
            children: {
                Some(vec![Chunk::P {
                    attributes: None,
                    children: process_text_dev(p).unwrap().1,
                }])
            },
        })
        .collect();
    let response = Section::ListSection {
        attributes: None,
        children: Some(items),
    };
    Ok(("", response))
}

fn part(source: &str) -> IResult<&str, &str> {
    let (source, content) = take_until("-")(source)?;
    let (source, _) = tag("-")(source)?;
    Ok((source, content.trim()))
}

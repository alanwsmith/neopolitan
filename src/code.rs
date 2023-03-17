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
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

pub fn code(source: &str) -> IResult<&str, Section> {
    let (source, value) = multispace0(source)?;
    let (source, value) = separated_list0(tag(">> "), take_until(">> "))(source)?;
    if value.is_empty() {
        let response = Section::CodeSection {
            attributes: None,
            language: None,
            children: vec![Chunk::Text {
                value: source.trim().to_string(),
            }],
        };
        Ok(("", response))
    } else {
        let (source, language) = tag(">> ")(source)?;
        let (source, language) = not_line_ending(source)?;
        let response = Section::CodeSection {
            attributes: None,
            language: Some(language.to_string()),
            children: vec![Chunk::Text {
                value: source.trim().to_string(),
            }],
        };
        Ok(("", response))
    }
}

#![allow(warnings)]
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

pub fn note(source: &str) -> IResult<&str, Section> {
    let (source, _) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;
    let (source, _) = not_line_ending(source)?;
    let (source, _) = line_ending(source)?;

    dbg!(&source);

    let block = Section::NoteSection {
        attributes: None,
        children: Some(vec![Chunk::P {
            attributes: None,
            children: Some(vec![Chunk::Text {
                value: source.to_string(),
            }]),
        }]),
    };
    Ok(("", block))
}

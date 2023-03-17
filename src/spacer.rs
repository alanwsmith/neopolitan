#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
use crate::section::section;
use crate::section::Section;
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
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

// This one makes sure there's two newlines
// and then clears any extra ones. All of this
// deals with extras spaces and tabs too.
pub fn spacer(source: &str) -> IResult<&str, &str> {
    let (source, _) = space0(source)?;
    let (source, value) = line_ending(source)?;
    let (source, _) = space0(source)?;
    let (source, value) = line_ending(source)?;
    let (source, _) = multispace0(source)?;
    Ok((source, value))
}

#![allow(unused_imports)]
use crate::block::block::Block;
use crate::block::block::*;
use crate::content::content::*;
use crate::section::code_section::*;
use crate::section::p::*;
use crate::section::section_attributes::*;
use crate::section::section_attributes::*;
use crate::section::title::*;
use nom::branch::alt;
use nom::bytes::complete::is_not;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::combinator::opt;
use nom::combinator::rest;
use nom::error::Error;
use nom::multi::many0;
use nom::multi::many_till;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {
    Title {
        // has to be a vec because order matters
        // for the code sections
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Paragraphs {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CodeSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        tuple((tag("-> title\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| title(t.1).unwrap().1),
        tuple((tag("-> p\n"), alt((take_until("\n\n-> "), rest)))).map(|t| p(t.1).unwrap().1),
        tuple((tag("-> code\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| code_section(t.1).unwrap().1),
    ))(source)?;
    Ok((remainder, sec))
}

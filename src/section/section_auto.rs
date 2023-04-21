use crate::block::block::*;

// AUTO GENERATED START: calls //
HERHERHER
// AUTO GENERATED END: calls //

use crate::section::section_attributes::SectionAttribute;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {

    
// AUTO GENERATED START: enum //
HEREHREHRE
// AUTO GENERATED END: enum //



    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    let (remainder, section) = alt((
        
// AUTO GENERATED START: tags //
HEREHREHRE
// AUTO GENERATED END: tags //



    ))(remainder)?;
    Ok((remainder, section))
}

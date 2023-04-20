use crate::block::block::*;
use crate::section::section_attributes::SectionAttribute;
use crate::section::subtitle::*;
use crate::section::title::*;
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
    TitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    SubtitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    let (_, section) = alt((
        tuple((
            tag("title"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| title(t.3).unwrap().1),
        tuple((
            tag("subtitle"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| subtitle(t.3).unwrap().1),
    ))(remainder)?;
    Ok(("", section))
}

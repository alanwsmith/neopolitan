use crate::block::block::*;

// AUTO GENERATED START: calls //
use crate::section::aside::*;
use crate::section::blockquote::*;
use crate::section::h1::*;
use crate::section::h2::*;
use crate::section::h3::*;
use crate::section::h4::*;
use crate::section::h5::*;
use crate::section::h6::*;
use crate::section::note::*;
use crate::section::subtitle::*;
use crate::section::title::*;
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
AsideSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
BlockquoteSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H1Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H2Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H3Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H4Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H5Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
H6Section
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
NoteSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
SubtitleSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
TitleSection
 {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
// AUTO GENERATED END: enum //



    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    let (remainder, section) = alt((
        
// AUTO GENERATED START: tags //
tuple((
            tag("aside"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| aside(t.3).unwrap().1),
tuple((
            tag("blockquote"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| blockquote(t.3).unwrap().1),
tuple((
            tag("h1"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h1(t.3).unwrap().1),
tuple((
            tag("h2"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h2(t.3).unwrap().1),
tuple((
            tag("h3"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h3(t.3).unwrap().1),
tuple((
            tag("h4"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h4(t.3).unwrap().1),
tuple((
            tag("h5"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h5(t.3).unwrap().1),
tuple((
            tag("h6"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| h6(t.3).unwrap().1),
tuple((
            tag("note"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| note(t.3).unwrap().1),
tuple((
            tag("subtitle"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| subtitle(t.3).unwrap().1),
tuple((
            tag("title"),
            not_line_ending,
            line_ending,
            alt((take_until("\n\n-> "), rest)),
        ))
        .map(|t| title(t.3).unwrap().1),
// AUTO GENERATED END: tags //



    ))(remainder)?;
    Ok((remainder, section))
}

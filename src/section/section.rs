#![allow(unused_imports)]
use crate::block::block::Block;
use crate::block::block::*;
use crate::content::content::*;
use crate::section::aside::*;
use crate::section::blockquote::blockquote;
use crate::section::code_section::*;
use crate::section::div::*;
use crate::section::h1::*;
use crate::section::h2::*;
use crate::section::h3::*;
use crate::section::h4::*;
use crate::section::h5::*;
use crate::section::h6::*;
use crate::section::list::*;
use crate::section::note::note;
use crate::section::p::*;
use crate::section::section_attributes::*;
use crate::section::section_attributes::*;
use crate::section::subtitle::*;
use crate::section::title::*;
use crate::section::warning::*;
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
    AsideSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    BlockquoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    DivSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    CodeSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Block>,
    },
    H1Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H2Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H3Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H4Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H5Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    H6Section {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    NoteSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Paragraphs {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Subtitle {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Title {
        // has to be a vec because order matters
        // for the code sections
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    List {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    WarningSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        tuple((tag("-> aside\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| aside(t.1).unwrap().1),
        tuple((tag("-> blockquote\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| blockquote(t.1).unwrap().1),
        tuple((tag("-> code\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| code_section(t.1).unwrap().1),
            tuple((tag("-> div\n"), alt((take_until("\n\n-> "), rest))))
                .map(|t| div(t.1).unwrap().1),
        tuple((tag("-> h1\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h1(t.1).unwrap().1),
        tuple((tag("-> h2\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h2(t.1).unwrap().1),
        tuple((tag("-> h3\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h3(t.1).unwrap().1),
        tuple((tag("-> h4\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h4(t.1).unwrap().1),
        tuple((tag("-> h5\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h5(t.1).unwrap().1),
        tuple((tag("-> h6\n"), alt((take_until("\n\n-> "), rest)))).map(|t| h6(t.1).unwrap().1),
        tuple((tag("-> note\n"), alt((take_until("\n\n-> "), rest)))).map(|t| note(t.1).unwrap().1),
        tuple((tag("-> title\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| title(t.1).unwrap().1),
        tuple((tag("-> p\n"), alt((take_until("\n\n-> "), rest)))).map(|t| p(t.1).unwrap().1),
        tuple((tag("-> subtitle\n"), alt((take_until("\n\n-> "), rest))))
            .map(|t| subtitle(t.1).unwrap().1),
        tuple((tag("-> list\n"), alt((take_until("\n\n-> "), rest)))).map(|t| list(t.1).unwrap().1),
        tuple((tag("-> warning\n"), alt((take_until("\n\n-> "), rest)))).map(|t| warning(t.1).unwrap().1),

    
    ))(source)?;
    Ok((remainder, sec))
}

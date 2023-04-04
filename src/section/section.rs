#![allow(unused_imports)]
use crate::block::block::Block;
use crate::block::block::*;
use crate::section::section_attributes::*;
use crate::section::section_attributes::*;
// use crate::wrapper::wrapper::content;
use crate::content::content::*;
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

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        // has to be a vec becosre order matters
        // for the code sections
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Paragraphs {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
}

pub fn section(source: &str) -> IResult<&str, Section> {
    // dbg!(source);
    let (source, _) = multispace0(source)?;
    let (remainder, sec) = alt((
        tuple((tag("-> title\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (s, att_capture) = many0(preceded(tag(">> "), section_attribute))(t.1).unwrap();
            let attributes = if att_capture.is_empty() {
                None
            } else {
                Some(att_capture)
            };
            // still not sure this is the right way to go about this.
            let (a, _) = multispace0::<&str, Error<&str>>(s).unwrap();
            let (_, b) = many_till(block, eof)(a).unwrap();
            let children = if b.0.is_empty() { None } else { Some(b.0) };
            Section::Title {
                attributes,
                children,
            }
        }),
        tuple((tag("-> p\n\n"), alt((take_until("\n\n-> "), rest)))).map(|t| {
            let (_, b) = many_till(block, eof)(t.1).unwrap();
            if b.0.is_empty() {
                Section::Paragraphs {
                    attributes: None,
                    children: None,
                }
            } else {
                Section::Paragraphs {
                    attributes: None,
                    children: Some(b.0),
                }
            }
        }),
    ))(source)?;
    Ok((remainder, sec))
}

use crate::block::block::Block;
use crate::section::blurb::*;
use crate::section::list::*;
use crate::section::p::*;
use crate::section::title::title;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Section {
    Blurb {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    List {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    P {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    Title {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    // Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (a, b) = alt((
        tuple((tag("->"), space1, tag_no_case("blurb"), space0, newline)).map(|(_, _, _, _, _)| {
            Section::Blurb {
                attributes: None,
                children: None,
            }
        }),
        tuple((tag("->"), space1, tag_no_case("list"), space0, newline)).map(|(_, _, _, _, _)| {
            Section::List {
                attributes: None,
                children: None,
            }
        }),
        tuple((tag("->"), space1, tag_no_case("p"), space0, newline)).map(|(_, _, _, _, _)| {
            Section::P {
                attributes: None,
                children: None,
            }
        }),
        tuple((tag("->"), space1, tag_no_case("title"), space0, newline)).map(|(_, _, _, _, _)| {
            Section::Title {
                attributes: None,
                children: None,
            }
        }),
    ))(source)
    .map(|(a, b)| match b {
        Section::Blurb {
            attributes: _,
            children: _,
        } => blurb(a).unwrap(),
        Section::List {
            attributes: _,
            children: _,
        } => list(a).unwrap(),
        Section::P {
            attributes: _,
            children: _,
        } => p_section(a).unwrap(),
        Section::Title {
            attributes: _,
            children: _,
        } => title(a).unwrap(),
        // Section::Placeholder => (a, b),
        // _ => (a, b),
    })?;
    Ok((a, b))
}

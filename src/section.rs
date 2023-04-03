#![allow(warnings)]
use crate::block::Block;
use crate::content::Content;
use crate::title::title;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::multispace0;
use nom::character::complete::newline;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Section {
    Title {
        attributes: Option<HashMap<String, String>>,
        children: Option<Vec<Block>>,
    },
    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (a, b) = alt((
        tuple((tag("->"), space1, tag_no_case("title"), space0, newline)).map(|(_, _, _, _, _)| {
            Section::Title {
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
        Section::Title {
            attributes,
            children,
        } => title(a).unwrap(),
        Section::Placeholder => (a, b),
    })?;
    Ok(("", b))
}

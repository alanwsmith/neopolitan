#![allow(warnings)]
use crate::chunk::Chunk;
use crate::page::Page;
use crate::spacer::spacer;
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

#[derive(Debug, PartialEq)]
pub enum Section {
    TITLE { children: Vec<Chunk> },
    P { children: Vec<Chunk> },
    PLACEHOLDER,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, mut block) = alt((
        tag("-> TITLE").map(|_| Section::TITLE { children: vec![] }),
        tag("-> P").map(|_| Section::P { children: vec![] }),
    ))(source)?;
    let (source, _) = spacer(source)?;
    match block {
        Section::TITLE { ref mut children } => {
            let (return_content, content) = alt((take_until("\n-> "), rest))(source)?;
            *children = vec![Chunk::H1 {
                attributes: HashMap::new(),
                children: vec![Chunk::Text {
                    value: content.trim().to_string(),
                }],
            }];
            Ok(("", block))
        }
        _ => {
            let section = Section::PLACEHOLDER;
            Ok(("", section))
        }
    }
}

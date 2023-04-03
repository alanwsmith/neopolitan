#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::block::block::Block;
use crate::block::p::p;
use crate::content::content::Content;
use crate::section::section::Section;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = alt((take_until("->"), rest))(source)?;
    let (_, paragraphs) = many_till(p, eof)(captured)?;
    Ok((
        source,
        Section::Title {
            attributes: None,
            children: Some(paragraphs.0),
        },
    ))
}

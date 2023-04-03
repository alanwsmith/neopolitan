#![allow(unused_variables)]
#![allow(unused_imports)]
use crate::block::block::Block;
use crate::block::list_item::list_item;
use crate::content::content::Content;
use crate::section::section::Section;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;

pub fn list(source: &str) -> IResult<&str, Section> {
    let (source, _) = multispace0(source)?;
    let (source, captured) = alt((take_until("->"), rest))(source)?;
    let (_, list_items) = many_till(list_item, eof)(captured)?;
    Ok((
        source,
        Section::List {
            attributes: None,
            children: Some(list_items.0),
        },
    ))
}

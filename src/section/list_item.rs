use crate::block::block::*;
use crate::section::list_enum::*;
use crate::source_file::joiner::joiner;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;

pub fn list_item(source: &str) -> IResult<&str, ListItem> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-")(remainder)?;
    let (remainder, _) = multispace1(remainder)?;
    let (remainder, captured) = alt((take_until("\n\n-"), rest))(remainder)?;
    let (_, parts) = many_till(block, eof)(captured)?;
    let the_parts = Some(parts.0);
    let text_string = joiner(&the_parts);
    Ok((
        remainder,
        ListItem::Basic {
            children: Some(text_string),
        },
    ))
}

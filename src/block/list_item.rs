use crate::block::block::Block;
use crate::block::p::p;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;

pub fn list_item(source: &str) -> IResult<&str, Block> {
    let (source, captured) = alt((take_until("\n\n"), rest))(source)?;
    let (source, _) = multispace0(source)?;
    let (captured, _) = tag("- ")(captured)?;
    let (_, paragraphs) = many_till(p, eof)(captured)?;
    Ok((
        source,
        Block::ListItem {
            attributes: None,
            children: Some(paragraphs.0),
        },
    ))
}

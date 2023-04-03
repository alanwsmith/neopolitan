use crate::block::block::Block;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
// use nom::character::complete::multispace1;
use nom::combinator::rest;
use nom::IResult;

pub fn p(source: &str) -> IResult<&str, Block> {
    let (source, captured) = alt((take_until("\n\n"), rest))(source)?;
    let (source, _) = multispace0(source)?;
    let (_, b) = content(captured)?;
    Ok((
        source,
        Block::P {
            attributes: None,
            children: Some(b),
        },
    ))
}

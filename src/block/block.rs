// use crate::enums::Content;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Block {
    P { children: Option<Vec<Content>> },
}

pub fn block(source: &str) -> IResult<&str, Block> {
    // dbg!(source);
    let (remainder, content) = many_till(content, alt((tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: Some(content.0),
        },
    ))
}

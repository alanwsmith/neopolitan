// use crate::enums::Content;
use crate::block::block::Block;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn block_of_code(source: &str) -> IResult<&str, Block> {
    // dbg!(source);
    let (remainder, content) = many_till(content, alt((tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: Some(content.0),
        },
    ))
}

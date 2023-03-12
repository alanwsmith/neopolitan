use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;
use nom::Parser;

#[derive(Debug, PartialEq)]
pub enum Marker {
    BLURB,
    P,
    TITLE,
}

#[derive(Debug, PartialEq)]
pub enum Block {
    BLURB { source: String },
    P { source: String },
    TITLE { source: String },
}

pub fn get_blocks(source: &str) -> IResult<&str, Vec<Block>> {
    let (source, blocks) = many_till(block_splitter, eof)(source)?;
    Ok((source, blocks.0))
}

pub fn block_splitter(source: &str) -> IResult<&str, Block> {
    let (source, _) = multispace0(source)?;
    let (source, block_type) = alt((
        tag("-> TITLE").map(|_| Marker::TITLE),
        tag("-> P").map(|_| Marker::P),
        tag("-> BLURB").map(|_| Marker::BLURB),
    ))(source)?;
    let (source, _) = multispace0(source)?;
    let (source, content) = alt((take_until1("\n\n-> "), rest))(source)?;
    let (source, _) = multispace0(source)?;
    let content = content.trim();
    match block_type {
        Marker::TITLE => Ok((
            source,
            Block::TITLE {
                source: content.to_string(),
            },
        )),
        Marker::BLURB => Ok((
            source,
            Block::BLURB {
                source: content.to_string(),
            },
        )),
        Marker::P => Ok((
            source,
            Block::P {
                source: content.to_string(),
            },
        )),
    }
}

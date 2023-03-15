#![allow(non_snake_case)]
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until1;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;
use nom::Parser;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Marker {
    BLURB,
    P,
    TITLE,
    ATTRIBUTES,
    CATEGORIES,
    UNORDERED_LIST,
    ORDERED_LIST,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Block {
    BLURB { source: String },
    P { source: String },
    TITLE { source: String },
    ATTRIBUTES { source: String },
    CATEGORIES { source: String },
    UNORDERED_LIST { source: String },
    ORDERED_LIST { source: String },
    PLACEHOLDER,
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
        tag("-> ATTRIBUTES").map(|_| Marker::ATTRIBUTES),
        tag("-> CATEGORIES").map(|_| Marker::CATEGORIES),
        tag("-> LIST").map(|_| Marker::UNORDERED_LIST),
        tag("-> OLIST").map(|_| Marker::ORDERED_LIST),
    ))(source)?;
    let (source, _) = multispace0(source)?;
    let (source, Section) = alt((take_until1("\n\n-> "), rest))(source)?;
    let (source, _) = multispace0(source)?;
    let Section = Section.trim();
    match block_type {
        Marker::TITLE => Ok((
            source,
            Block::TITLE {
                source: Section.to_string(),
            },
        )),
        Marker::BLURB => Ok((
            source,
            Block::BLURB {
                source: Section.to_string(),
            },
        )),
        Marker::P => Ok((
            source,
            Block::P {
                source: Section.to_string(),
            },
        )),
        Marker::ATTRIBUTES => Ok((
            source,
            Block::ATTRIBUTES {
                source: Section.to_string(),
            },
        )),
        Marker::CATEGORIES => Ok((
            source,
            Block::CATEGORIES {
                source: Section.to_string(),
            },
        )),
        Marker::UNORDERED_LIST => Ok((
            source,
            Block::UNORDERED_LIST {
                source: Section.to_string(),
            },
        )),
        Marker::ORDERED_LIST => Ok((
            source,
            Block::ORDERED_LIST {
                source: Section.to_string(),
            },
        )),
    }
}

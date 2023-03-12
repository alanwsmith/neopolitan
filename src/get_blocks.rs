use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
// use nom::multi::many1;
use nom::branch::alt;
use nom::bytes::complete::take_until1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;
use nom::Parser;

pub enum Marker {
    TITLE,
    P,
}

#[derive(Debug, PartialEq)]
pub enum Block {
    TITLE { source: String },
    P { source: String },
}

pub fn get_blocks(source: &str) -> IResult<&str, Vec<Block>> {
    // let (source, _) = tag("-> ")(source)?;
    let (source, blocks) = many_till(block_splitter, eof)(source)?;
    // let (source, blocks) = many1(eof,)
    // dbg!(&blocks.0);
    // let blocks: Vec<Block> = vec![];
    Ok((source, blocks.0))
}

pub fn block_splitter(source: &str) -> IResult<&str, Block> {
    dbg!(source);
    let (source, _) = multispace0(source)?;
    let (source, block_type) = alt((
        tag("-> TITLE").map(|_| Marker::TITLE),
        tag("-> P").map(|_| Marker::P),
    ))(source)?;
    let (source, _) = multispace0(source)?;
    let (source, content) = alt((take_until1("\n\n-> "), rest))(source)?;
    let (source, _) = multispace0(source)?;

    dbg!(source);
    match block_type {
        Marker::TITLE => Ok((
            source,
            Block::TITLE {
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

    // Ok((
    //     source,
    //     Block::TITLE {
    //         source: "asdf".to_string(),
    //     },
    // ))

    // }

    // Ok((
    //     source,
    //     Block::TITLE {
    //         source: "asdf".to_string(),
    //     },
    // ))
}

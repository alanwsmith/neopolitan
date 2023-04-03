use crate::block::block::Block;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
// use nom::character::complete::multispace1;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
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

pub fn p_dev(source: &str) -> IResult<&str, Block> {
    let (source, captured) = alt((take_until("\n\n"), rest))(source)?;
    let (source, _) = multispace0(source)?;
    let (_, content) = many_till(content_dev, eof)(captured)?;
    Ok((
        source,
        Block::P {
            attributes: None,
            children: Some(
                content.0, // vec![
                          // Content::Text {
                          //     text: Some("the ".to_string()),
                          // },
                          // Content::Link {
                          //     source: None,
                          //     attributes: None,
                          //     url: Some("localhost:9090".to_string()),
                          //     text: Some("the ".to_string()),
                          // },
                          // Content::Text {
                          //     text: Some(" plank".to_string()),
                          // },
                          // ]
            ),
        },
    ))
}

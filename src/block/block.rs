use crate::section::attributes_for_section::*;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Block {
    P { children: Option<Vec<Content>> },
    CodeBlock { text: Option<String> },
    UnorderedListItem {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>> ,
    }
}

pub fn block(source: &str) -> IResult<&str, Block> {
    let (remainder, content) = many_till(content, alt((tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: Some(content.0),
        },
    ))
}

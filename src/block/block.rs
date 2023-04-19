use crate::snippet::snippet::*;
use nom::branch::alt;
use nom::bytes::complete::take_until;
use nom::combinator::eof;
use nom::combinator::rest;
use nom::multi::many_till;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub enum Block {
    Text { snippets: Option<Vec<Snippet>> },
    Placeholder,
}

pub fn block(source: &str) -> IResult<&str, Block> {
    let (_, captured) = alt((take_until("\n\n"), rest))(source)?;
    // dbg!(captured);
    let (remainder, snippets) = many_till(snippet, eof)(captured)?;
    let return_block = Block::Text {
        snippets: Some(snippets.0),
    };
    Ok((remainder.trim(), return_block))
}

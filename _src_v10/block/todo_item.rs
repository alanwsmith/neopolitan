use crate::block::block::*;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::peek;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;
// use nom::Parser;

pub fn todo_item(source: &str) -> IResult<&str, Block> {
    let (remainder, b) = tuple((multispace0, tag("["), take_until("]"), tag("] ")))(source)
        .map(|(x, z)| (x, z.2))?;
    let status: Option<String> = if b.is_empty() {
        None
    } else {
        Some(b.to_string())
    };

    let (remainder, content) =
        many_till(unordered_list_content, alt((tag("\n\n"), eof)))(remainder)?;
    Ok((
        remainder,
        Block::ToDoItem {
            status,
            attributes: None,
            children: Some(content.0),
        },
    ))
}

pub fn unordered_list_content(source: &str) -> IResult<&str, Block> {
    let (remainder, content) =
        many_till(content, alt((peek(tag("\n\n[")), tag("\n\n"), eof)))(source)?;
    Ok((
        remainder,
        Block::P {
            children: Some(content.0),
        },
    ))
}

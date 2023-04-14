use crate::block::block::*;
use crate::content::content::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::combinator::peek;
use nom::multi::many_till;
use nom::sequence::tuple;
use nom::IResult;

pub fn todo_item(source: &str) -> IResult<&str, Block> {
    let (remainder, _) = tuple((multispace0, tag("[] ")))(source)?;
    let (remainder, content) =
        many_till(unordered_list_content, alt((tag("\n\n"), eof)))(remainder)?;
    Ok((
        remainder,
        Block::ToDoItem {
            status: None,
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

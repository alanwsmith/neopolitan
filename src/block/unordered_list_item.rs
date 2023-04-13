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

pub fn unordered_list_item(source: &str) -> IResult<&str, Block> {
    dbg!("DDDDD");
    dbg!(source);
    // let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tuple((multispace0, tag("- ")))(source)?;
    dbg!("EEEEE");
    dbg!(remainder);
    let (remainder, content) =
        many_till(unordered_list_content, alt((tag("\n\n"), eof)))(remainder)?;
    dbg!("FFFFF");
    dbg!(&remainder);
    dbg!("GGGGG");
    dbg!(&content);
    Ok((
        remainder,
        Block::UnorderedListItem {
            attributes: None,
            children: Some(content.0),
        },
    ))
}

pub fn unordered_list_content(source: &str) -> IResult<&str, Block> {
    dbg!("HHHHH");
    dbg!(&source);
    // let (a, _) = tuple((multispace0, tag("- ")))(source)?;
    let (remainder, content) =
        many_till(content, alt((peek(tag("\n\n-")), tag("\n\n"), eof)))(source)?;
    dbg!("IIIII");
    dbg!(&remainder);
    dbg!("JJJJJ");
    dbg!(&content);
    Ok((
        remainder,
        Block::P {
            children: Some(content.0),
        },
    ))
}

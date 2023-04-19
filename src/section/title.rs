use crate::block::block::*;
use crate::section::section::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (_, blocks) = many_till(block, eof)(remainder)?;
    Ok((
        "",
        Section::TitleSection {
            attributes: None,
            children: Some(blocks.0),
        },
    ))
}

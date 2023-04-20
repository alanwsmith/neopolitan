use crate::block::block::*;
use crate::section::section::*;
use crate::section::section_attributes::*;
use nom::character::complete::multispace0;
use nom::combinator::eof;
use nom::multi::many_till;
use nom::IResult;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    // dbg!(&blocks);
    // dbg!(&remainder);
    Ok((
        remainder,
        Section::TitleSection {
            attributes: None,
            children: Some(blocks.0),
        },
    ))
}

pub fn title_dev(source: &str) -> IResult<&str, Section> {
    let (remainder, attributes) = section_attributes(source)?;
    // dbg!(&attributes);

    // dbg!(source);
    let (remainder, _) = multispace0(remainder)?;
    let (remainder, blocks) = many_till(block, eof)(remainder)?;
    // dbg!(&blocks);
    // dbg!(&remainder);
    Ok((
        remainder,
        Section::TitleSection {
            attributes,
            children: Some(blocks.0),
        },
    ))
}

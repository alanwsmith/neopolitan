use crate::block::block::*;
use crate::section::section_attributes::SectionAttribute;
use crate::section::title::*;
use crate::snippet::snippet::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::not_line_ending;
use nom::combinator::rest;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Section {
    TitleSection {
        attributes: Option<Vec<SectionAttribute>>,
        children: Option<Vec<Block>>,
    },
    Placeholder,
}

pub fn section(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    let (_, section) = alt((tuple((
        tag("title"),
        not_line_ending,
        line_ending,
        alt((take_until("\n\n-> "), rest)),
    ))
    .map(|t| title(t.3).unwrap().1),))(remainder)?;
    Ok(("", section))
}

pub fn section_dev(source: &str) -> IResult<&str, Section> {
    let (remainder, _) = multispace0(source)?;
    let (remainder, _) = tag("-> ")(remainder)?;
    // dbg!(remainder);
    let (_, section) = alt((tuple((
        tag("title"),
        not_line_ending,
        line_ending,
        alt((take_until("\n\n-> "), rest)),
    ))
    .map(|t| title_dev(t.3).unwrap().1),))(remainder)?;

    Ok((
        "",
        section,
        // Section::TitleSection {
        //     attributes: Some(vec![SectionAttribute::Attribute {
        //         key: Some("id".to_string()),
        //         value: Some("bravo".to_string()),
        //     }]),
        //     children: Some(vec![Block::Text {
        //         snippets: Some(vec![Snippet::Plain {
        //             text: Some("Set The Piece".to_string()),
        //         }]),
        //     }]),
        // },
    ))
}

use crate::content::Content;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Section {
    TITLE {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
    P {
        attributes: HashMap<String, String>,
        children: Vec<Content>,
    },
}

// use crate::get_paragraphs::*;
// use crate::get_title::*;
// use crate::h::*;
// use crate::p::*;
// use crate::parse::Marker;
// use crate::parse::Section;
// use crate::prep_attributes::*;
// use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::bytes::complete::take_until1;
// use nom::character::complete::multispace0;
// use nom::combinator::rest;
// use nom::IResult;
// use nom::Parser;

// pub fn section(source: &str) -> IResult<&str, Section> {
//     let (source, _) = multispace0(source)?;
//     let (source, section_type) = alt((
//         tag("-> TITLE").map(|_| Marker::Title),
//         tag("-> H1").map(|_| Marker::H1),
//         tag("-> H2").map(|_| Marker::H2),
//         tag("-> H3").map(|_| Marker::H3),
//         tag("-> H4").map(|_| Marker::H4),
//         tag("-> H5").map(|_| Marker::H5),
//         tag("-> H6").map(|_| Marker::H6),
//         tag("-> ATTRIBUTES").map(|_| Marker::ATTRIBUTES),
//         tag("-> P").map(|_| Marker::PARAGRAPHS),
//     ))(source)?;
//     let (source, content) = alt((take_until1("\n\n-> "), rest))(source)?;
//     match section_type {
//         // Marker::ATTRIBUTES => Ok((source, Section::ATTRIBUTES)),
//         Marker::ATTRIBUTES => Ok((source, prep_attributes(content).unwrap().1)),
//         Marker::H1 => Ok((source, h1(content).unwrap().1)),
//         Marker::H2 => Ok((source, h2(content).unwrap().1)),
//         Marker::H3 => Ok((source, h3(content).unwrap().1)),
//         Marker::H4 => Ok((source, h4(content).unwrap().1)),
//         Marker::H5 => Ok((source, h5(content).unwrap().1)),
//         Marker::H6 => Ok((source, h6(content).unwrap().1)),
//         Marker::PARAGRAPHS => Ok((source, p(content).unwrap().1)),
//         Marker::Title => Ok((source, get_title(content).unwrap().1)),
//     }
// }

// pub fn section_dev(source: &str) -> IResult<&str, Section> {
//     let (source, _) = multispace0(source)?;
//     let (source, section_type) = alt((
//         tag("-> TITLE").map(|_| Marker::Title),
//         tag("-> H1").map(|_| Marker::H1),
//         tag("-> H2").map(|_| Marker::H2),
//         tag("-> H3").map(|_| Marker::H3),
//         tag("-> H4").map(|_| Marker::H4),
//         tag("-> H5").map(|_| Marker::H5),
//         tag("-> H6").map(|_| Marker::H6),
//         tag("-> ATTRIBUTES").map(|_| Marker::ATTRIBUTES),
//         tag("-> PARAGRAPHS").map(|_| Marker::PARAGRAPHS),
//     ))(source)?;
//     let (source, content) = alt((take_until1("\n\n-> "), rest))(source)?;
//     match section_type {
//         // Marker::ATTRIBUTES => Ok((source, Section::ATTRIBUTES)),
//         Marker::ATTRIBUTES => Ok((source, prep_attributes(content).unwrap().1)),
//         Marker::H1 => Ok((source, h1(content).unwrap().1)),
//         Marker::H2 => Ok((source, h2(content).unwrap().1)),
//         Marker::H3 => Ok((source, h3(content).unwrap().1)),
//         Marker::H4 => Ok((source, h4(content).unwrap().1)),
//         Marker::H5 => Ok((source, h5(content).unwrap().1)),
//         Marker::H6 => Ok((source, h6(content).unwrap().1)),
//         Marker::PARAGRAPHS => Ok((source, get_paragraphs(content).unwrap().1)),
//         Marker::Title => Ok((source, get_title(content).unwrap().1)),
//     }
// }

#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::page::Page;
use crate::section::*;
use crate::text::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::character::complete::line_ending;
use nom::character::complete::multispace0;
use nom::character::complete::multispace1;
use nom::character::complete::not_line_ending;
use nom::character::complete::space0;
use nom::combinator::eof;
use nom::combinator::not;
use nom::combinator::rest;
use nom::error::Error;
use nom::error::ErrorKind;
use nom::multi::many0;
use nom::multi::many1;
use nom::multi::many_till;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::Err;
use nom::IResult;
use nom::Parser;
use std::collections::HashMap;

pub fn hr(source: &str) -> IResult<&str, Section> {
    // // dbg!(&source);
    // let (remainder, mut attributes) = attributes(source)?;
    // // dbg!(&attributes);
    // // dbg!(&remainder);
    // let (remainder, title) = alt((take_until("\n\n"), rest))(remainder)?;
    // let (remainder, _) = multispace0(remainder)?;
    // let (remainder, mut paragraph_texts) =
    //     separated_list0(tag("\n\n"), take_until("\n\n"))(remainder)?;
    // if remainder != "" {
    //     paragraph_texts.push(remainder.trim());
    // }
    // let mut chunks: Vec<Chunk> = vec![Chunk::H1 {
    //     // attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),
    //     attributes,
    //     children: Some(vec![Chunk::Text {
    //         attributes: None,
    //         value: Some(title.to_string()),
    //     }]),
    // }];
    // chunks.extend(paragraph_texts.iter().map(|p| Chunk::P {
    //     attributes: None,
    //     children: text(p).unwrap().1,
    // }));

    let response = Section::HRSection { attributes: None };

    Ok(("", response))
}

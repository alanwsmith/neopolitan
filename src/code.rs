#![allow(warnings)]
use crate::attributes::*;
use crate::chunk::Chunk;
use crate::language::language;
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

pub fn code(source: &str) -> IResult<&str, Section> {
    // dbg!(&source);
    let language = language(source).unwrap().1;
    let (remainder, attributes) = attributes(source)?;
    // dbg!(&attributes);

    let response = Section::CodeSection {
        attributes,
        language,
        children: Some(vec![Chunk::Text {
            attributes: None,
            value: Some(remainder.to_string()),
        }]),
    };
    Ok(("", response))

    // match attributes {
    //     Some(x) => {
    //         if x.len() == 1 {
    //             let response = Section::CodeSection {
    //                 attributes: attributes.unwrap(),
    //                 // attributes: None,
    //                 // language: Some(x[0].0.as_ref().unwrap().to_string()),
    //                 language,
    //                 children: Some(vec![Chunk::Text {
    //                     attributes: None,
    //                     value: Some(remainder.to_string()),
    //                 }]),
    //             };
    //             Ok(("", response))
    //         } else {
    //             let response = Section::CodeSection {
    //                 attributes: None,
    //                 // attributes,
    //                 // attributes: Some(
    //                 //     x.clone()
    //                 //         .into_iter()
    //                 //         .skip(1)
    //                 //         .collect::<Vec<(Option<String>, Option<String>)>>(),
    //                 // ),
    //                 //    language: Some(x[0].0.as_ref().unwrap().to_string()),
    //                 language,
    //                 children: Some(vec![Chunk::Text {
    //                     attributes: None,
    //                     value: Some(remainder.to_string()),
    //                 }]),
    //             };
    //             Ok(("", response))
    //         }
    //     }
    //     None => {
    //         let response = Section::CodeSection {
    //             attributes: None,
    //             language: None,
    //             children: Some(vec![Chunk::Text {
    //                 attributes: None,
    //                 value: Some(remainder.to_string()),
    //             }]),
    //         };
    //         Ok(("", response))
    //     }
    // }
}

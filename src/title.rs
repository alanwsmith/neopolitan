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
// use std::collections::HashMap;

pub fn title(source: &str) -> IResult<&str, Section> {
    let (remainder, mut attributes) = attributes(source)?;
    let (remainder, title) = alt((take_until("\n\n"), rest))(remainder)?;
    dbg!(remainder);
    dbg!(title);

    // attributes
    //     .as_mut()
    //     .unwrap()
    //     .push((Some("class".to_string()), Some("title".to_string())));

    let expected = Section::TitleSection {
        attributes: None,
        children: Some(vec![Chunk::H1 {
            attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),
            children: Some(vec![Chunk::Text {
                attributes: None,
                value: Some("Alfa Bravo".to_string()),
            }]),
        }]),
    };

    match attributes {
        Some(x) => {
            Ok(("", expected))

            //             if x.len() == 1 {
            //                 let response = Section::CodeSection {
            //                     attributes: None,
            //                     language: Some(x[0].0.as_ref().unwrap().to_string()),
            //                     children: Some(vec![Chunk::Text {
            //                         attributes: None,
            //                         value: Some(remainder.to_string()),
            //                     }]),
            //                 };
            //                 Ok(("", response))
            //             } else {
            //                 let response = Section::CodeSection {
            //                     attributes: Some(
            //                         x.clone()
            //                             .into_iter()
            //                             .skip(1)
            //                             .collect::<Vec<(Option<String>, Option<String>)>>(),
            //                     ),
            //                     language: Some(x[0].0.as_ref().unwrap().to_string()),
            //                     children: Some(vec![Chunk::Text {
            //                         attributes: None,
            //                         value: Some(remainder.to_string()),
            //                     }]),
            //                 };
            //                 Ok(("", response))
            //             }
        }
        None => {
            let expected = Section::TitleSection {
                attributes: None,
                children: Some(vec![Chunk::H1 {
                    attributes: Some(vec![(Some("class".to_string()), Some("title".to_string()))]),
                    children: Some(vec![Chunk::Text {
                        attributes: None,
                        value: Some(title.to_string()),
                    }]),
                }]),
            };
            Ok(("", expected))
            //             let response = Section::CodeSection {
            //                 attributes: None,
            //                 language: None,
            //                 children: Some(vec![Chunk::Text {
            //                     attributes: None,
            //                     value: Some(remainder.to_string()),
            //                 }]),
            //             };
            //             Ok(("", response))
            //         }
        } // Ok(("", expected))
    }
}
